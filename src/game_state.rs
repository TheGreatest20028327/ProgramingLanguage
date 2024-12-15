use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub points: u32,
}

// Load the game state from the JSON file
pub fn load_game() -> GameState {
    if Path::new("game_state.json").exists() {
        let data = fs::read_to_string("game_state.json").unwrap();
        serde_json::from_str(&data).unwrap_or(GameState { points: 0 })
    } else {
        GameState { points: 0 }
    }
}

// Save the game state to a JSON file
pub fn save_game(game_state: &GameState) {
    let serialized = serde_json::to_string(game_state).unwrap();
    fs::write("game_state.json", serialized).unwrap();
}