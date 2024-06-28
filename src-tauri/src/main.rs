// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod database;
mod spider;
use anyhow::Result;
use log::{error, info};
use serde_json::to_string;
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
    page_number: i64,
) -> Result<String, String> {
    info!("spider_start started with username: {}, password: {}, date: {}, country: {}, page_number: {}", username, password, date, country, page_number);

    let mut spider = Spider::new(&username, &password, &date, &country, page_number);
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
#[tauri::command]
async fn spider_status() -> Result<String, String> {
    match Spider::status().await {
        Ok(status) => {
            info!("Spider status: {}", status);
            Ok(status)
        }
        Err(e) => {
            error!("Error getting spider status: {}", e);
            Err(format!("Error getting spider status: {}", e))
        }
    }
}

#[tauri::command]
fn spider_return_failed() -> Result<String, String> {
    match database::return_failed_operation() {
        Ok(spiders) => {
            info!("Failed operations returned successfully");
            Ok(to_string(&spiders).unwrap())
        }
        Err(e) => {
            error!("Error returning failed operations: {}", e);
            Err(format!("Error returning failed operations: {}", e))
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
    let config: Config = ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%d %H:%M:%S") // This sets the date format
        .build();
    WriteLogger::init(LevelFilter::Info, config, log_file).unwrap(); // For file logging

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            spider_start,
            spider_status,
            spider_return_failed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
