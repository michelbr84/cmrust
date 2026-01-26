// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod models;

use models::DisplayPlayer;
use cm_core::world::player::{Player, Position};
use cm_core::ids::{NationId, PlayerId};
use chrono::NaiveDate;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_new_game(name: &str, surname: &str, nation_id: u32, team_id: &str) -> String {
    println!("Starting new game for {} {} (Nation: {}, Team: {})", name, surname, nation_id, team_id);
    format!("Game started for {} {}", name, surname)
}

#[tauri::command]
fn get_team_squad(team_id: u32) -> Vec<DisplayPlayer> {
    // In a real app, we would access the global State<GameState> and get real players.
    // For now, we generate random players on the fly to prove the connection.
    let mut players = Vec::new();
    
    // Generate 25 players
    for i in 1..=25 {
        let pos = if i == 1 { Position::Goalkeeper } 
                  else if i <= 8 { Position::DefenderCenter }
                  else if i <= 16 { Position::MidfielderCenter }
                  else { Position::ForwardCenter };
                  
        let p = Player::new(
            PlayerId::new((i + (team_id * 100)).to_string()), // Unique ID based on team
            format!("Player"), 
            format!("{}", i),
            NationId::new("1"), // Default
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            pos
        );
        players.push(DisplayPlayer::from(&p));
    }
    
    players
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_new_game, get_team_squad])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
