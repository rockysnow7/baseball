use indicatif::ProgressIterator;
use reqwest::Response;
use serde::{Serialize, Deserialize, Deserializer};

async fn send_request(url: &str, max_retries: usize) -> Result<Response, String> {
    for _ in 0..max_retries {
        let response = reqwest::get(url).await;
        if let Ok(response) = response {
            if response.status().is_success() {
                return Ok(response);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Err("Request failed".to_string())
}

fn parse_mlb_percentage(value: &serde_json::Value) -> Result<f32, String> {
    let Some(value) = value.as_str() else {
        // println!("{:?}", value);
        return Err("Value is not a string".to_string());
    };

    match value.parse::<f32>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Failed to parse value".to_string()),
    }
}

#[derive(Debug, Serialize)]
pub struct HittingStats {
    pub games_played: u8,
    pub ground_outs: u8,
    pub air_outs: u8,
    pub runs: u8,
    pub doubles: u8,
    pub triples: u8,
    pub home_runs: u8,
    pub strike_outs: u8,
    pub base_on_balls: u8,
    pub intentional_walks: u8,
    pub hits: u8,
    pub hit_by_pitch: u8,
    pub avg: f32,
    pub at_bats: u16,
    pub obp: f32,
    pub slg: f32,
    pub ops: f32,
    pub caught_stealing: u8,
    pub stolen_bases: u8,
    pub stolen_base_percentage: f32,
    pub ground_into_double_play: u8,
    pub number_of_pitches: u16,
    pub plate_appearances: u16,
    pub total_bases: u16,
    pub rbi: u8,
    pub left_on_base: u8,
    pub sac_bunts: u8,
    pub sac_flies: u8,
    pub babip: f32,
    pub ground_outs_to_airouts: f32,
    pub at_bats_per_home_run: f32,
}

impl<'de> Deserialize<'de> for HittingStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        Ok(HittingStats {
            games_played: value["gamesPlayed"].as_u64().unwrap_or(0) as u8,
            ground_outs: value["groundOuts"].as_u64().unwrap_or(0) as u8,
            air_outs: value["airOuts"].as_u64().unwrap_or(0) as u8,
            runs: value["runs"].as_u64().unwrap_or(0) as u8,
            doubles: value["doubles"].as_u64().unwrap_or(0) as u8,
            triples: value["triples"].as_u64().unwrap_or(0) as u8,
            home_runs: value["homeRuns"].as_u64().unwrap_or(0) as u8,
            strike_outs: value["strikeOuts"].as_u64().unwrap_or(0) as u8,
            base_on_balls: value["baseOnBalls"].as_u64().unwrap_or(0) as u8,
            intentional_walks: value["intentionalWalks"].as_u64().unwrap_or(0) as u8,
            hits: value["hits"].as_u64().unwrap_or(0) as u8,
            hit_by_pitch: value["hitByPitch"].as_u64().unwrap_or(0) as u8,
            avg: parse_mlb_percentage(&value["avg"]).map_err(serde::de::Error::custom)?,
            at_bats: value["atBats"].as_u64().unwrap_or(0) as u16,
            obp: parse_mlb_percentage(&value["obp"]).map_err(serde::de::Error::custom)?,
            slg: parse_mlb_percentage(&value["slg"]).map_err(serde::de::Error::custom)?,
            ops: parse_mlb_percentage(&value["ops"]).map_err(serde::de::Error::custom)?,
            caught_stealing: value["caughtStealing"].as_u64().unwrap_or(0) as u8,
            stolen_bases: value["stolenBases"].as_u64().unwrap_or(0) as u8,
            stolen_base_percentage: parse_mlb_percentage(&value["stolenBasePercentage"]).map_err(serde::de::Error::custom)?,
            ground_into_double_play: value["groundIntoDoublePlay"].as_u64().unwrap_or(0) as u8,
            number_of_pitches: value["numberOfPitches"].as_u64().unwrap_or(0) as u16,
            plate_appearances: value["plateAppearances"].as_u64().unwrap_or(0) as u16,
            total_bases: value["totalBases"].as_u64().unwrap_or(0) as u16,
            rbi: value["rbi"].as_u64().unwrap_or(0) as u8,
            left_on_base: value["leftOnBase"].as_u64().unwrap_or(0) as u8,
            sac_bunts: value["sacBunts"].as_u64().unwrap_or(0) as u8,
            sac_flies: value["sacFlies"].as_u64().unwrap_or(0) as u8,
            babip: parse_mlb_percentage(&value["babip"]).map_err(serde::de::Error::custom)?,
            ground_outs_to_airouts: parse_mlb_percentage(&value["groundOutsToAirouts"]).map_err(serde::de::Error::custom)?,
            at_bats_per_home_run: parse_mlb_percentage(&value["atBatsPerHomeRun"]).map_err(serde::de::Error::custom)?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct PitchingStats {
    pub games_played: u8,
    pub games_started: u8,
    pub ground_outs: u8,
    pub air_outs: u8,
    pub runs: u8,
    pub doubles: u8,
    pub triples: u8,
    pub home_runs: u8,
    pub strike_outs: u8,
    pub base_on_balls: u8,
    pub intentional_walks: u8,
    pub hits: u8,
    pub hit_by_pitch: u8,
    pub avg: f32,
    pub at_bats: u16,
    pub era: f32,
    pub innings_pitched: f32,
    pub wins: u8,
    pub losses: u8,
    pub ties: u8,
    pub saves: u8,
    pub save_opportunities: u8,
    pub holds: u8,
    pub blown_saves: u8,
    pub earned_runs: u8,
    pub whip: f32,
    pub batters_faced: u16,
    pub outs: u16,
    pub games_pitched: u8,
    pub complete_games: u8,
    pub shutouts: u8,
    pub strikes: u16,
    pub strike_percentage: f32,
    pub hit_batsmen: u8,
    pub balks: u8,
    pub wild_pitches: u8,
    pub pickoffs: u8,
    pub ground_outs_to_airouts: f32,
    pub win_percentage: f32,
    pub pitches_per_inning: f32,
    pub games_finished: u8,
    pub strikeout_walk_ratio: f32,
    pub strikeouts_per_9inn: f32,
    pub walks_per_9inn: f32,
    pub hits_per_9inn: f32,
    pub runs_scored_per_9: f32,
    pub home_runs_per_9: f32,
    pub sac_bunts: u8,
    pub sac_flies: u8,
}

impl<'de> Deserialize<'de> for PitchingStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        
        Ok(PitchingStats {
            games_played: value["gamesPlayed"].as_u64().unwrap_or(0) as u8,
            games_started: value["gamesStarted"].as_u64().unwrap_or(0) as u8,
            ground_outs: value["groundOuts"].as_u64().unwrap_or(0) as u8,
            air_outs: value["airOuts"].as_u64().unwrap_or(0) as u8,
            runs: value["runs"].as_u64().unwrap_or(0) as u8,
            doubles: value["doubles"].as_u64().unwrap_or(0) as u8,
            triples: value["triples"].as_u64().unwrap_or(0) as u8,
            home_runs: value["homeRuns"].as_u64().unwrap_or(0) as u8,
            strike_outs: value["strikeOuts"].as_u64().unwrap_or(0) as u8,
            base_on_balls: value["baseOnBalls"].as_u64().unwrap_or(0) as u8,
            intentional_walks: value["intentionalWalks"].as_u64().unwrap_or(0) as u8,
            hits: value["hits"].as_u64().unwrap_or(0) as u8,
            hit_by_pitch: value["hitByPitch"].as_u64().unwrap_or(0) as u8,
            avg: parse_mlb_percentage(&value["avg"]).map_err(serde::de::Error::custom)?,
            at_bats: value["atBats"].as_u64().unwrap_or(0) as u16,
            era: parse_mlb_percentage(&value["era"]).map_err(serde::de::Error::custom)?,
            innings_pitched: parse_mlb_percentage(&value["inningsPitched"]).map_err(serde::de::Error::custom)?,
            wins: value["wins"].as_u64().unwrap_or(0) as u8,
            losses: value["losses"].as_u64().unwrap_or(0) as u8,
            ties: value["ties"].as_u64().unwrap_or(0) as u8,
            saves: value["saves"].as_u64().unwrap_or(0) as u8,
            save_opportunities: value["saveOpportunities"].as_u64().unwrap_or(0) as u8,
            holds: value["holds"].as_u64().unwrap_or(0) as u8,
            blown_saves: value["blownSaves"].as_u64().unwrap_or(0) as u8,
            earned_runs: value["earnedRuns"].as_u64().unwrap_or(0) as u8,
            whip: parse_mlb_percentage(&value["whip"]).map_err(serde::de::Error::custom)?,
            batters_faced: value["battersFaced"].as_u64().unwrap_or(0) as u16,
            outs: value["outs"].as_u64().unwrap_or(0) as u16,
            games_pitched: value["gamesPitched"].as_u64().unwrap_or(0) as u8,
            complete_games: value["completeGames"].as_u64().unwrap_or(0) as u8,
            shutouts: value["shutouts"].as_u64().unwrap_or(0) as u8,
            strikes: value["strikes"].as_u64().unwrap_or(0) as u16,
            strike_percentage: parse_mlb_percentage(&value["strikePercentage"]).map_err(serde::de::Error::custom)?,
            hit_batsmen: value["hitBatsmen"].as_u64().unwrap_or(0) as u8,
            balks: value["balks"].as_u64().unwrap_or(0) as u8,
            wild_pitches: value["wildPitches"].as_u64().unwrap_or(0) as u8,
            pickoffs: value["pickoffs"].as_u64().unwrap_or(0) as u8,
            ground_outs_to_airouts: parse_mlb_percentage(&value["groundOutsToAirouts"]).map_err(serde::de::Error::custom)?,
            win_percentage: parse_mlb_percentage(&value["winPercentage"]).map_err(serde::de::Error::custom)?,
            pitches_per_inning: parse_mlb_percentage(&value["pitchesPerInning"]).map_err(serde::de::Error::custom)?,
            games_finished: value["gamesFinished"].as_u64().unwrap_or(0) as u8,
            strikeout_walk_ratio: parse_mlb_percentage(&value["strikeoutWalkRatio"]).map_err(serde::de::Error::custom)?,
            strikeouts_per_9inn: parse_mlb_percentage(&value["strikeoutsPer9Inn"]).map_err(serde::de::Error::custom)?,
            walks_per_9inn: parse_mlb_percentage(&value["walksPer9Inn"]).map_err(serde::de::Error::custom)?,
            hits_per_9inn: parse_mlb_percentage(&value["hitsPer9Inn"]).map_err(serde::de::Error::custom)?,
            runs_scored_per_9: parse_mlb_percentage(&value["runsScoredPer9"]).map_err(serde::de::Error::custom)?,
            home_runs_per_9: parse_mlb_percentage(&value["homeRunsPer9"]).map_err(serde::de::Error::custom)?,
            sac_bunts: value["sacBunts"].as_u64().unwrap_or(0) as u8,
            sac_flies: value["sacFlies"].as_u64().unwrap_or(0) as u8,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct FieldingStats {
    pub games_played: u8,
    pub games_started: u8,
    pub assists: u16,
    pub put_outs: u16,
    pub errors: u8,
    pub chances: u16,
    pub fielding: f32,
    pub range_factor_per_game: f32,
    pub range_factor_per_9inn: f32,
    pub innings: f32,
    pub games: u8,
    pub double_plays: u8,
    pub triple_plays: u8,
    pub throwing_errors: u8,
}

impl<'de> Deserialize<'de> for FieldingStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        
        Ok(FieldingStats {
            games_played: value["gamesPlayed"].as_u64().unwrap_or(0) as u8,
            games_started: value["gamesStarted"].as_u64().unwrap_or(0) as u8,
            assists: value["assists"].as_u64().unwrap_or(0) as u16,
            put_outs: value["putOuts"].as_u64().unwrap_or(0) as u16,
            errors: value["errors"].as_u64().unwrap_or(0) as u8,
            chances: value["chances"].as_u64().unwrap_or(0) as u16,
            fielding: parse_mlb_percentage(&value["fielding"]).map_err(serde::de::Error::custom)?,
            range_factor_per_game: parse_mlb_percentage(&value["rangeFactorPerGame"]).map_err(serde::de::Error::custom)?,
            range_factor_per_9inn: parse_mlb_percentage(&value["rangeFactorPer9Inn"]).map_err(serde::de::Error::custom)?,
            innings: parse_mlb_percentage(&value["innings"]).map_err(serde::de::Error::custom)?,
            games: value["games"].as_u64().unwrap_or(0) as u8,
            double_plays: value["doublePlays"].as_u64().unwrap_or(0) as u8,
            triple_plays: value["triplePlays"].as_u64().unwrap_or(0) as u8,
            throwing_errors: value["throwingErrors"].as_u64().unwrap_or(0) as u8,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl From<&str> for Date {
    fn from(date: &str) -> Self {
        let parts: Vec<&str> = date.split('-').collect();
        Date {
            year: parts[0].parse().unwrap(),
            month: parts[1].parse().unwrap(),
            day: parts[2].parse().unwrap(),
        }
    }
}

impl Date {
    pub fn previous_day(&self) -> Self {
        let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let mut year = self.year;
        let mut month = self.month;
        let mut day = self.day;

        if day > 1 {
            day -= 1;
        } else if month > 1 {
            month -= 1;
            day = month_days[month as usize - 1];
        } else {
            year -= 1;
            month = 12;
            day = month_days[month as usize - 1];
        }

        Date { year, month, day }
    }

    pub fn minus_n_days(&self, n: u8) -> Self {
        let mut date = *self;
        for _ in 0..n {
            date = date.previous_day();
        }
        date
    }
}

#[derive(Debug, Serialize)]
pub struct Team {
    pub team_id: u8,
    pub batting_stats: HittingStats,
    pub pitching_stats: PitchingStats,
    pub fielding_stats: FieldingStats,
}

impl Team {
    pub async fn by_date(team_id: u8, end_date: &Date) -> Result<Self, String> {
        let season = end_date.year;
        let start_date = end_date.minus_n_days(30);
        let url = format!(
            "https://statsapi.mlb.com/api/v1/teams/{}/stats?season={}&stats=byDateRange&group=hitting,pitching,fielding&startDate={}&endDate={}",
            team_id,
            season,
            start_date.to_string(),
            end_date.to_string(),
        );
        //let stats = reqwest::get(&url).await.unwrap().json::<serde_json::Value>().await.unwrap();
        let stats = match send_request(&url, 5).await {
            Ok(stats) => stats.json::<serde_json::Value>().await.unwrap(),
            Err(e) => return Err(e),
        };
        // println!("{:#?}", stats);

        let batting_stats = match serde_json::from_value(stats["stats"][0]["splits"][0]["stat"].clone()) {
            Ok(stats) => stats,
            Err(_) => return Err(format!("Failed to parse batting stats: {url}")),
        };
        let pitching_stats = match serde_json::from_value(stats["stats"][1]["splits"][0]["stat"].clone()) {
            Ok(stats) => stats,
            Err(_) => return Err(format!("Failed to parse pitching stats: {url}")),
        };
        let fielding_stats = match serde_json::from_value(stats["stats"][2]["splits"][0]["stat"].clone()) {
            Ok(stats) => stats,
            Err(_) => return Err(format!("Failed to parse fielding stats: {url}")),
        };

        Ok(Team {
            team_id,
            batting_stats,
            pitching_stats,
            fielding_stats,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct GameResult {
    pub home_team_score: u8,
    pub away_team_score: u8,
}

#[derive(Debug, Serialize)]
pub struct GameSetting {
    pub date: Date,
}

#[derive(Debug, Serialize)]
pub struct Game {
    pub setting: GameSetting,
    pub home_team: Team,
    pub away_team: Team,
    pub result: GameResult,
}

impl Game {
    pub async fn from_value(game_value: &serde_json::Value) -> Result<Self, String> {
        let game = &game_value["games"][0];

        let status_code = game["status"]["statusCode"].as_str().unwrap();
        if status_code != "F" {
            return Err("Game is not finished".to_string());
        }

        let date = Date::from(game["officialDate"].as_str().unwrap());
        let setting = GameSetting { date };

        let home_team_id = game["teams"]["home"]["team"]["id"].as_u64().unwrap() as u8;
        let away_team_id = game["teams"]["away"]["team"]["id"].as_u64().unwrap() as u8;

        let home_team = Team::by_date(home_team_id, &date).await?;
        let away_team = Team::by_date(away_team_id, &date).await?;

        let home_team_score = match game["teams"]["home"]["score"].as_u64() {
            Some(score) => score as u8,
            None => return Err("Failed to parse home team score".to_string()),
        };
        let away_team_score = game["teams"]["away"]["score"].as_u64().unwrap() as u8;
        let result = GameResult { home_team_score, away_team_score };

        Ok(Game {
            setting,
            home_team,
            away_team,
            result,
        })
    }

    pub async fn get_all_by_team_in_season(team_id: u8, season: u16, show_errors: bool) -> Vec<Self> {
        let url = format!(
            "https://statsapi.mlb.com/api/v1/schedule?sportId=1&teamId={}&season={}",
            team_id,
            season,
        );
        //let schedule = reqwest::get(&url).await.unwrap().json::<serde_json::Value>().await.unwrap();
        let schedule = match send_request(&url, 5).await {
            Ok(schedule) => schedule.json::<serde_json::Value>().await.unwrap(),
            Err(e) => {
                if show_errors {
                    eprintln!("{}", e);
                }
                return Vec::new();
            }
        };

        let mut games = Vec::new();
        for (i, game) in schedule["dates"].as_array().unwrap().iter().enumerate().progress() {
            if i == 0 {
                continue;
            }
            let game = match Game::from_value(&game).await {
                Ok(game) => game,
                Err(e) => {
                    if show_errors {
                        eprintln!("{}", e);
                    }
                    continue;
                }
            };
            games.push(game);
        }

        games
    }

    pub fn save(&self) -> Result<(), String> {
        // Create base directories
        std::fs::create_dir_all(format!(
            "data/{}/{}", 
            self.setting.date.year,
            self.home_team.team_id
        )).map_err(|e| format!("Failed to create directories: {}", e))?;

        // Create file path
        let file_path = format!(
            "data/{}/{}/{}.json",
            self.setting.date.year,
            self.home_team.team_id,
            self.setting.date.to_string()
        );

        // Serialize and save
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize game: {}", e))?;
        std::fs::write(&file_path, json)
            .map_err(|e| format!("Failed to write file {}: {}", file_path, e))?;

        Ok(())
    }
}
