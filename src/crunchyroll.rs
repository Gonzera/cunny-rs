use crate::models::{Episode, Season, Token};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct CrunchyClient {
    client: Client,
    access_token: String,
    refresh_token: String,
    token_expires_in: u64,
    token_expires_at: u64,
}

impl CrunchyClient {
    pub async fn new(uname: &str, pw: &str, client: Client) -> Result<CrunchyClient, &'static str> {
        let mut form_data = HashMap::new();
        form_data.insert("username", uname);
        form_data.insert("password", pw);
        form_data.insert("grant_type", "password");
        form_data.insert("scope", "offline_access");
        let request = client
            .post("https://beta-api.crunchyroll.com/auth/v1/token")
            .header(
                "authorization",
                "Basic MWVud2kxNnp2dnI4eWI1ajRqc3A6MVBmSHpSRndlXzBKVzMxQzNwUDFQU2hNNWNzdTBpNS0=",
            ) // no idea what this is
            .header(
                "user-agent",
                "Crunchyroll/3.31.1 Android/8.0.0 okhttp/4.9.2",
            )
            .form(&form_data);
        let response = match request.send().await {
            Ok(res) => res,
            Err(e) => {
                println!("Error sending login request: {}", e);
                return Err("Failed to login");
            }
        };
        if response.status() != 200 {
            println!("Failed to login, is the password correct?");
            return Err("Response was not 200");
        }

        // let text = response.text().await.unwrap();
        // dbg!(&text);

        let token: Token = response.json().await.unwrap();
        let access_token = token.access_token.clone();
        let refresh_token = token.refresh_token.clone();
        let token_expires_in = token.expires_in;

        let mut cr = CrunchyClient {
            client,
            access_token,
            refresh_token,
            token_expires_in,
            token_expires_at: 0,
        };
        cr.set_token_expiry();

        Ok(cr)
    }

    // Helper function so that we don't keep rewriting the same thing
    async fn build_request(
        &mut self,
        url: &str,
        query: Vec<(&str, &str)>,
    ) -> reqwest::RequestBuilder {
        // Checks the auth token
        if !self.is_valid_token() {
            println!("Refreshing token");
            self.refresh_token().await;
        }
        let auth = format!("Bearer {}", self.access_token);
        // let query = vec![("locale", "en-US"), ("preferred_audio_language", "ja-JP")];

        let request = self
            .client
            .get(url)
            .query(&query)
            .header("authorization", auth.as_str())
            .header("accept-encoding", "gzip")
            .header(
                "user-agent",
                "Crunchyroll/3.31.1 Android/8.0.0 okhttp/4.9.2",
            );

        request
    }

    pub fn set_token_expiry(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The fabric of time is broken")
            .as_secs();
        let expiry = now + self.token_expires_in;
        self.token_expires_at = expiry;
    }

    fn is_valid_token(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The fabric of time is broken")
            .as_secs();
        // remove 3 seconds to make sure xD
        now < self.token_expires_at - 3
    }

    async fn refresh_token(&mut self) {
        let mut form_data = HashMap::new();
        form_data.insert("refresh_token", self.refresh_token.as_str());
        form_data.insert("grant_type", "refresh_token");
        // form_data.insert("scope", "offline_access");
        let request = self
            .client
            .post("https://beta-api.crunchyroll.com/auth/v1/token")
            .header(
                "authorization",
                "Basic MWVud2kxNnp2dnI4eWI1ajRqc3A6MVBmSHpSRndlXzBKVzMxQzNwUDFQU2hNNWNzdTBpNS0=",
            ) // no idea what this is
            .header(
                "user-agent",
                "Crunchyroll/3.31.1 Android/8.0.0 okhttp/4.9.2",
            )
            .form(&form_data);
        let response = match request.send().await {
            Ok(res) => res,
            Err(e) => {
                println!("Error sending login request: {}", e);
                return;
            }
        };
        if response.status() != 200 {
            println!("Failed to refresh access token");
        }
        let json: serde_json::Value = response.json().await.unwrap();
        let access_token = json["access_token"].to_string().replace("\"", "");
        self.access_token = access_token;
    }

    pub async fn get_series_seasons(
        &mut self,
        series_guid: String,
        locale: String,
        audio_locale: String,
    ) -> Result<Season, &'static str> {
        let url = format!(
            "https://beta-api.crunchyroll.com/content/v2/cms/series/{}/seasons",
            series_guid
        );
        let query = vec![
            ("locale", locale.as_str()),
            ("preferred_audio_language", audio_locale.as_str()),
        ];
        let request = self.build_request(&url, query).await;

        let response = match request.send().await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to get episodes: {}", e);
                return Err("Failed to get episodes");
            }
        };

        if response.status() != 200 {
            eprintln!(
                "Failed to get season data. Response status: {}",
                response.status()
            );
            return Err("Crunchyroll API return status not 200");
        }

        let season: Season = match response.json().await {
            Ok(s) => s,
            Err(_) => return Err("Failed to parse Season json"),
        };
        Ok(season)
    }

    pub async fn get_episode(
        &mut self,
        episode_id: String,
        locale: String,
        audio_locale: String,
    ) -> Result<Episode, &'static str> {
        let url = format!(
            "https://beta-api.crunchyroll.com/content/v2/cms/episodes/{}",
            episode_id
        );
        let query = vec![
            ("locale", locale.as_str()),
            ("preferred_audio_language", audio_locale.as_str()),
        ];

        let request = self.build_request(&url, query).await;

        let response = match request.send().await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to send request. Error: {}", e);
                return Err("Failed to send request");
            }
        };

        if response.status() != 200 {
            eprintln!("Response status was not 200. Status: {}", response.status());
            return Err("Failed to get episode");
        }

        let episode: Episode = match response.json().await {
            Ok(episode) => episode,
            Err(e) => {
                eprintln!("Failed to parse episode json: {}", e);
                return Err("Failed to parse episode json");
            }
        };

        Ok(episode)
    }

    pub async fn get_season_episodes(
        &mut self,
        season_id: String,
        locale: String,
        audio_locale: String,
    ) -> Result<Episode, &'static str> {
        let url = format!(
            "https://beta-api.crunchyroll.com/content/v2/cms/seasons/{}/episodes",
            season_id
        );
        let query = vec![
            ("locale", locale.as_str()),
            ("preferred_audio_language", audio_locale.as_str()),
        ];

        let request = self.build_request(&url, query).await;

        let response = match request.send().await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to get season episodes, Error: {}", e);
                return Err("Failed to get season episodes");
            }
        };

        if response.status() != 200 {
            eprintln!("oops");
            return Err("Failed to get episodes");
        }

        // let episodes: Episode = response.json().await.unwrap();
        let episodes = match response.json().await {
            Ok(e) => e,
            Err(_) => {
                return Err("Failed to parse episodes json");
            }
        };

        Ok(episodes)
    }

    pub async fn get_episode_stream(
        &mut self,
        episode_id: String,
        locale: String,
        audio_locale: String,
    ) -> Result<String, &'static str> {
        // The json for the stream data is very complicated
        // So until I have the time to create structs for it I will just use a Value and pray
        let url = format!("https://beta-api.crunchyroll.com{}", episode_id);
        println!("{}", url);
        let query = vec![
            ("streams", "all"),
            ("textType", "all"),
            ("locale", locale.as_str()),
            ("preferred_audio_language", audio_locale.as_str()),
        ];

        let request = self.build_request(&url, query).await;

        let response = match request.send().await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to get episode streams. Error: {}", e);
                return Err("Failed to get streams");
            }
        };

        if response.status() != 200 {
            eprintln!("Request was not 200, status: {}", response.status());
            return Err("A");
        }

        let json: Value = response.json().await.unwrap();

        let url = json["data"][0]["multitrack_text_hls"]["en-US"]["url"]
            .as_str()
            .unwrap();
        Ok(String::from(url))
    }
}
