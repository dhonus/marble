#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// serialize
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AudioBook {
    title: String,
    location: String,
    chapters: Vec<Chapter>,
    last_played: String,
}
#[derive(Serialize, Deserialize)]
struct Chapter {
    title: String,
    file: String,
    length: u32, // in seconds
}

#[derive(Serialize, Deserialize)]
struct AppData {
    library: Vec<AudioBook>,
    current_book: AudioBook,
    current_chapter: Chapter,
    current_position: u32,
    current_speed: u32,
}

#[tauri::command]
fn greet(name: &str) -> String {    
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_data() -> AppData {
    let mut appData = AppData {
        library: Vec::new(),
        current_book: AudioBook {
            title: String::from(""),
            location: String::from(""),
            chapters: Vec::new(),
            last_played: String::from(""),
        },
        current_chapter: Chapter {
            title: String::from(""),
            file: String::from(""),
            length: 0,
        },
        current_position: 0,
        current_speed: 64,
    };
    // send appData to JS
    appData
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_data, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
