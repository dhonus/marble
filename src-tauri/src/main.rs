#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// serialize
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::ptr::null;
use mp3_duration;
use serde_json::json;
use serde_json::Value;
use std::io::prelude::*;
use tauri::api::path;

#[derive(Serialize, Deserialize, Debug)]
struct AudioBook {
    title: String,
    author: String,
    location: String,
    chapters: Vec<Chapter>,
    last_played: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Chapter {
    title: String,
    file: String,
    length: u32, // in seconds
}

#[derive(Serialize, Deserialize, Debug)]
struct AppData {
    library: Vec<AudioBook>,
    current_book: AudioBook,
    current_chapter: Chapter,
    current_position: u32,
    current_speed: u32,
}

// the imported books as they come
#[derive(Serialize, Deserialize, Debug)]
struct ToImportBook {
   name: String,
   author: String,
   path: String,
}

#[tauri::command]
fn greet(name: &str) -> String {    
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn import_book(book: ToImportBook) -> String{
    println!("Adding book to library: {:?}", book);

    // first get the list of files in the directory
    let mut files = Vec::new();
    let paths = std::fs::read_dir(book.path.clone()).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".mp3") {
            files.push(path_str.to_string());
        }
    }
    // sort the files
    files.sort();

    // create the chapters
    let mut chapters = Vec::new();
    for file in files {
        let path = Path::new(&file);
        let duration = mp3_duration::from_path(&path).unwrap();
        println!("File duration: {:?}", duration);

        let chapter = Chapter {
            title: file.split("/").last().unwrap().to_string(),
            file: file,
            length: duration.as_secs() as u32,
        };
        chapters.push(chapter);
    }

    // create the book
    let finalBook = AudioBook {
        title: book.name,
        author: book.author,
        location: book.path,
        chapters: chapters,
        last_played: String::from(""),
    };

    println!("Final book: {:?}", finalBook);

    // add the book to the library
    // the library lives in the app date folder under library.json
    // if the file doesn't exist, create it

    let p = path::data_dir().expect("Could not get data dir");
    let the_path = p.to_str().expect("Could not get path as str");

    println!("The current directory is {} and the suggested if", the_path);

    let path = Path::new(the_path).join("library.json");
    println!("Path: {:?}", path);
    if !path.exists() {
        let mut file = std::fs::File::create(path).unwrap();
        let json = json!({
            "library": [finalBook]
        });
        let json_string = serde_json::to_string(&json).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
    } else {
        // read the file
        let mut file = std::fs::File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut json: Value = serde_json::from_str(&contents).unwrap();
        let library = json["library"].as_array_mut().unwrap();
        library.push(serde_json::to_value(finalBook).unwrap());
        let json_string = serde_json::to_string(&json).unwrap();

        file = std::fs::File::create(&path).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
    }

    format!("OK")
}

#[tauri::command]
fn get_app_data() -> AppData {
    // get library
    let p = path::data_dir().expect("Could not get data dir");
    let the_path = p.to_str().expect("Could not get path as str");
    let path = Path::new(the_path).join("library.json");

    if !path.exists() {
        let mut file = std::fs::File::create(path.clone()).unwrap();
        let json = json!({
            "library": []
        });
        let json_string = serde_json::to_string(&json).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
    }

    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let json: Value = serde_json::from_str(&contents).unwrap();
    let library = json["library"].as_array().unwrap();
    println!("Library: {:?}", library);

    let mut appData = AppData {
        library: serde_json::from_value(json["library"].clone()).unwrap(),
        current_book: AudioBook {
            title: String::from(""),
            author: String::from(""),
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
        current_speed: 1,
    };

    appData
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_data, import_book, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
