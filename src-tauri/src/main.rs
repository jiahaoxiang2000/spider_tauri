// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod spider;
use anyhow::Result;
use log::{error, info};
use simplelog::*;
use spider::Spider;
use std::fs::OpenOptions;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn spider_start(
    username: String,
    password: String,
    date: String,
    country: String,
) -> Result<String, String> {
    let mut spider = Spider::new(&username, &password, &date, &country);
    match spider.start().await {
        Ok(_) => {
            info!("Spider started successfully");
            Ok("Spider started successfully".to_string())
        }
        Err(e) => {
            error!("Error starting spider: {}", e);
            Err(format!("Error starting spider: {}", e))
        }
    }
}

fn main() {
    // Initialize simplelog
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("spider_app.log")
        .unwrap();
    let config = ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%d %H:%M:%S") // This sets the date format
        .build();
    WriteLogger::init(LevelFilter::Info, config, log_file).unwrap(); // For file logging

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![spider_start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
