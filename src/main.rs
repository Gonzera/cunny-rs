// TODO: Remove this later
#![allow(dead_code)]
#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::Command;

use crate::models::{Episode, EpisodeData, Season};
use clap::Parser;

mod crunchyroll;
mod models;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Crunchyroll username or email
    #[arg(short, long)]
    user: String,
    /// Crunchyroll password
    #[arg(short, long)]
    password: String,
    /// Save directory, by the default it will create a new directory with the show's title
    /// Use this option to overwrite this behavior
    #[arg(short, long)]
    directory: Option<String>,
    /// ID of the show
    /// This option will download all seasons and episodes
    #[arg(short, long, conflicts_with_all = &["season_id", "episode_id"])]
    show_id: Option<String>,
    /// ID of the season
    /// This option will download all episodes in the season
    #[arg(long, conflicts_with_all = &["show_id", "episode_id"])]
    season_id: Option<String>,
    /// ID of the episode
    /// This option will download only a single episode
    #[arg(short, long, conflicts_with_all = &["season_id", "show_id"])]
    episode_id: Option<String>,
    // Locale, default: en-US
    #[arg(long, default_value_t = String::from("en-US"))]
    locale: String,
    // Audio locale, default: ja-JP
    #[arg(long, default_value_t = String::from("ja-JP"))]
    audio_locale: String,
    // Subtitle language, default: en-US
    #[arg(long, default_value_t = String::from("en-US"))]
    substitles: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let username = args.user;
    let password = args.password;
    let client = reqwest::Client::new();
    let mut cr =
        crunchyroll::CrunchyClient::new(username.as_str(), password.as_str(), client.clone())
            .await
            .unwrap();

    // Entire show flow
    if args.show_id.is_some() {
        let seasons: Season = match cr
            .get_series_seasons(
                args.show_id.unwrap(),
                args.locale.clone(),
                args.audio_locale.clone(),
            )
            .await
        {
            Ok(s) => s,
            Err(_) => return,
        };
        for data in seasons.data {
            println!(
                "Getting episodes for season {}, total episodes: {}",
                data.identifier, data.number_of_episodes
            );
            let episodes: Episode = match cr
                .get_season_episodes(data.id, args.locale.clone(), args.audio_locale.clone())
                .await
            {
                Ok(episodes) => episodes,
                Err(e) => {
                    println!("error {}", e);
                    return;
                }
            };
            if args.directory.is_some() {
                if let Err(err) = change_current_directory(args.directory.clone().unwrap().as_str())
                {
                    eprintln!("Error: {}", err);
                    return;
                }
            } else {
                if let Err(err) = change_current_directory(&episodes.data[0].series_title.as_str())
                {
                    eprintln!("Error: {}", err);
                    return;
                }
            }
            for ep in episodes.data {
                let file_name = build_file_name(&ep);
                println!("{}", ep.streams_link);
                let stream_url = cr
                    .get_episode_stream(
                        ep.streams_link,
                        args.locale.clone(),
                        args.audio_locale.clone(),
                    )
                    .await
                    .unwrap();
                save_episode(stream_url, file_name);
            }
        }
    }

    // Single episode flow
    if args.episode_id.is_some() {
        let episode: Episode = match cr
            .get_episode(
                args.episode_id.unwrap(),
                args.locale.clone(),
                args.audio_locale.clone(),
            )
            .await
        {
            Ok(episode) => episode,
            Err(_) => {
                return;
            }
        };
        let stream_url = cr
            .get_episode_stream(
                episode.data[0].streams_link.clone(),
                args.locale,
                args.audio_locale,
            )
            .await
            .unwrap();
        let file_name = build_file_name(&episode.data[0]);
        if args.directory.is_some() {
            if let Err(err) = change_current_directory(args.directory.unwrap().as_str()) {
                eprintln!("Error: {}", err);
                return;
            }
        } else {
            if let Err(err) = change_current_directory(episode.data[0].series_title.as_str()) {
                eprintln!("Error: {}", err);
                return;
            }
        }
        save_episode(stream_url, file_name);
    }
}

fn build_file_name(ep: &EpisodeData) -> String {
    format!(
        "{}_S{}E{}.mp4",
        ep.series_title, ep.season_number, ep.episode_number
    )
}

fn change_current_directory(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the directory exists
    if !fs::metadata(path).is_ok() {
        // Create the directory if it doesn't exist
        fs::create_dir_all(path)?;
        // println!("Created directory: {}", path);
    }

    // Change the current working directory
    env::set_current_dir(path)?;

    Ok(())
}
fn save_episode(url: String, file_name: String) {
    println!("downloading, this may take a while... {}", file_name);
    let i = format!("-i {url}");
    let output = Command::new("ffmpeg")
        .arg("-user_agent")
        .arg("Crunchyroll/3.31.1 Android/8.0.0 okhttp/4.9.2")
        .arg("-i")
        .arg(&url)
        .arg("-c")
        .arg("copy")
        .arg(&file_name)
        .output()
        .unwrap();
    if !output.status.success() {
        println!("Failed to download episode.")
    } else {
        println!("Download completed")
    }
}
