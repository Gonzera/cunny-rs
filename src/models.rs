use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub account_id: String,
    pub country: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EpisodeData {
    pub audio_locale: String,
    pub subtitle_locales: Vec<String>,
    pub streams_link: String,
    pub duration_ms: u64,
    pub episode_number: u64,
    pub hd_flag: bool,
    pub id: String,
    pub identifier: String,
    pub is_clip: bool,
    pub is_dubbed: bool,
    pub is_mature: bool,
    pub is_premium_only: bool,
    pub season_id: String,
    pub season_number: u64,
    pub title: String,
    pub series_title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Episode {
    pub data: Vec<EpisodeData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonData {
    pub audio_locale: String,
    pub id: String,
    pub identifier: String,
    pub season_number: u64,
    pub number_of_episodes: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Season {
    pub data: Vec<SeasonData>,
}
