extern crate chrono;
use anyhow::Result;
use chrono::Utc;
use log::{error, info};
use serde_json::{json, Value};
use simplelog::*;
use std::fs::{File, OpenOptions};
use std::sync::Once;

#[derive(Debug)]
struct Spider {
    username: String,
    password: String,
    date: String,
    country_code: String,
    token: String,
}

impl Spider {
    fn new(username: &str, password: &str, date: &str, country: &str) -> Self {
        let country_code = match country {
            "Brazil" => "0055",
            "India" => "0091",
            "Indonesia" => "0062",
            "Philippines" => "0063",
            "Pakistan" => "0092",
            _ => "",
        };
        let name = if username.eq("") { "456bet" } else { username };
        let pass = if password.eq("") {
            "456bet888"
        } else {
            password
        };

        Self {
            username: name.to_string(),
            password: pass.to_string(),
            date: date.to_string(),
            country_code: country_code.to_string(),
            token: "".to_string(),
        }
    }

    async fn get_token(&mut self) -> Result<()> {
        let timestamp = Utc::now().timestamp();
        let url = format!(
            "https://web.antgst.com/antgst/sys/getCheckCode?_t={}",
            timestamp
        );

        let response = reqwest::get(url).await?;

        if response.status().is_success() {
            let body = response.text().await?;
            // Parse the body as JSON
            let v: Value = serde_json::from_str(&body)?;

            // Extract the code and key using dynamic access
            let code = v["result"]["code"].as_str().unwrap_or_default();
            let key = v["result"]["key"].as_str().unwrap_or_default();

            info!("Code: {}, Key: {}", code, key);
            // Update the token here
            let body = json!({
                "username": self.username,
                "password": self.password,
                "captcha": code,
                "checkKey": key,
                "remember_me": true
            })
            .to_string();

            let respose = reqwest::Client::new()
                .post("https://web.antgst.com/antgst/sys/login")
                .body(body)
                .header("Content-Type", "application/json")
                .send()
                .await?
                .text()
                .await?;
            let v: Value = serde_json::from_str(&respose)?;
            self.token = v["result"]["token"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            info!("Token: {}", self.token);
        } else {
            error!("Failed to getCheckCode: {}", response.status());
        }

        Ok(())
    }
}

static INIT: Once = Once::new();

pub fn initialize_logger() {
    INIT.call_once(|| {
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
    });
}
#[cfg(test)]
mod tests {
    use super::*;

    // This test checks for failure in token retrieval due to network or other errors
    #[tokio::test]
    async fn test_get_token() {
        initialize_logger();
        let mut spider = Spider::new("", "", "2024-06-24", "All");
        assert_eq!(spider.token, "");
        let result = spider.get_token().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_spider_new_with_valid_credentials() {
        let spider = Spider::new("testuser", "testpass", "2023-04-01", "Brazil");
        assert_eq!(spider.username, "testuser");
        assert_eq!(spider.password, "testpass");
        assert_eq!(spider.date, "2023-04-01");
        assert_eq!(spider.country_code, "0055");
        assert_eq!(spider.token, "");
    }

    #[test]
    fn test_spider_new_with_empty_username_and_password() {
        let spider = Spider::new("", "", "2023-04-01", "India");
        assert_eq!(spider.username, "456bet");
        assert_eq!(spider.password, "456bet888");
        assert_eq!(spider.date, "2023-04-01");
        assert_eq!(spider.country_code, "0091");
        assert_eq!(spider.token, "");
    }

    #[test]
    fn test_spider_new_with_unsupported_country() {
        let spider = Spider::new("testuser", "testpass", "2023-04-01", "All");
        assert_eq!(spider.country_code, "");
    }
}
