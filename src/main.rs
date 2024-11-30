use std::env;
use indicatif::{ProgressIterator, ProgressStyle};

mod model;

// all mlb team ids
const TEAM_IDS: [u8; 30] = [108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 158];

/// Remove the completed team from the list of teams to be processed in the given season.
fn save_progress(season: u16, completed_team_id: u8) {
    let mut progress = serde_json::from_str::<serde_json::Value>(std::fs::read_to_string("data/progress.json").unwrap_or("{}".to_string()).as_str()).unwrap();

    if progress.get(&season.to_string()).is_none() {
        progress[season.to_string()] = serde_json::Value::Array(TEAM_IDS.iter().map(|id| serde_json::Value::Number(serde_json::Number::from(*id))).collect());
    }

    let progress_season = progress.get_mut(&season.to_string()).unwrap().as_array_mut().unwrap();
    progress_season.retain(|id| id.as_u64().unwrap() != completed_team_id as u64);

    std::fs::write("data/progress.json", serde_json::to_string_pretty(&progress).unwrap()).unwrap();
}

#[tokio::main]
async fn main() {
    let season = env::args().nth(1).unwrap().parse::<u16>().unwrap();

    /*for id in team_ids.iter().progress_with_style(ProgressStyle::default_bar().template("{wide_bar} {pos}/{len} | elapsed: {elapsed_precise}, eta: {eta_precise}").unwrap()) {
        let games = model::Game::get_all_by_team_in_season(*id, season, false).await;
        for game in games {
            let _ = game.save();
        }
    }*/

    let progress = serde_json::from_str::<serde_json::Value>(std::fs::read_to_string("data/progress.json").unwrap_or("{}".to_string()).as_str()).unwrap();
    let progress_season = match progress.get(&season.to_string()) {
        Some(progress_season) => progress_season.as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u8).collect(),
        None => TEAM_IDS.to_vec(),
    };
    println!("Processing season {} for {} teams ({:?})", season, progress_season.len(), progress_season);

    let progress_style = ProgressStyle::default_bar().template("{wide_bar} {pos}/{len} | elapsed: {elapsed_precise}, eta: {eta_precise}").unwrap();
    for id in progress_season.iter().progress_with_style(progress_style) {
        let games = model::Game::get_all_by_team_in_season(*id, season, false).await;
        for game in games {
            let _ = game.save();
        }
        save_progress(season, *id);
    }
}
