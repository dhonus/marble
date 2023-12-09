// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use rodio::source::{Source};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
   
    unsafe {
        match &TX {
            Some(tx) => {
                tx.send(AudioCommand::Play).unwrap();
            }
            None => {}
        }
    }

    
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug)]
enum AudioCommand {
    Play,
    Stop,
}

#[tauri::command]
fn play_audio() {
    println!("Playing audio");
    unsafe {
        match &TX {
            Some(tx) => {
                tx.send(AudioCommand::Play).unwrap();
            }
            None => {}
        }
    }
}

fn audio_thread(rx: Receiver<AudioCommand>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("/home/daniel/Downloads/siddhartha/Hesse-Saddhartha_08.mp3").unwrap());
    let source = Decoder::new(file).unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    loop {
        match rx.recv() {
            Ok(command) => match command {
                AudioCommand::Play => {
                    sink.play();
                    sink.sleep_until_end();
                }
                AudioCommand::Stop => {
                    sink.stop();
                }
            },
            Err(_) => break,
        }
    }
}

// let's make tx and rx global
static mut TX: Option<Sender<AudioCommand>> = None;

fn main() {

    let (tx, rx) = channel();
    unsafe {
        TX = Some(tx);
    }

    // Spawn the audio thread
    let audio_thread_handle = thread::spawn(move || {
        audio_thread(rx);
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    unsafe {
        match &TX {
            Some(tx) => {
                tx.send(AudioCommand::Stop).unwrap();
            }
            None => {}
        }
    }
    audio_thread_handle.join().unwrap();

}
