extern crate lazy_static;

extern crate chrono;
use anyhow::Result;
use chrono::{Local, Utc};
use csv::WriterBuilder;
use dirs::{desktop_dir, home_dir};
use log::{debug, error, info};
use serde::Serialize;
use serde_json::{json, Value};
use simplelog::*;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::sync::Once;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::database::log_failed_operation;

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

#[derive(Serialize)]
#[derive(Debug)]
pub struct Spider {
    pub username: String,
    pub password: String,
    pub date: String,
    pub country_code: String,
    pub token: String,
    pub page_number: i64,
}


lazy_static::lazy_static! {
    static ref SPIDER_STATUS: Mutex<String> = Mutex::new(String::new());
}

impl Spider {
    pub fn new(
        username: &str,
        password: &str,
        date: &str,
        country: &str,
        page_number: i64,
    ) -> Self {
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
            page_number: page_number,
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

    async fn fetch_url(&self, query: &str) -> Result<String> {
        let url = format!(
            "https://web.antgst.com/antgst/sms/otpPremium/channel/sendRecordList?{}",
            query
        );
        debug!("fetch URL: {}", url);
        let respond = reqwest::Client::new()
            .get(url)
            .header("X-Access-Token", self.token.clone())
            .send()
            .await?
            .text()
            .await?;
        Ok(respond)
    }

    async fn fetch_data(&mut self) -> Result<String> {
        let timestamp = Utc::now().timestamp();
        let mut page_number = 1;
        let query = format!(
            "_t={}&day={}&countryCode={}&column=createtime&order=desc&gatewayDr=000&pageNo={}&pageSize=100",
            timestamp, self.date, self.country_code, page_number
        );

        let response = self.fetch_url(&query).await?;
        let v: Value = serde_json::from_str(&response)?;
        let pages = v["result"]["pages"].as_i64().unwrap_or_default();
        info!("Total item phone number: {}", pages * 100);

        let now = Local::now();
        let formatted_time = now.format("%Y-%m-%d_%H-%M-%S").to_string();
        let store_file_name = format!("data_{}.csv", formatted_time);

        let start_page_number = self.page_number;

        for i in start_page_number..=pages {
            {
                let mut status = SPIDER_STATUS.lock().await;
                *status = format!("Fetching data from page {} of {}", i * 100, pages * 100);
            }

            page_number = i;
            self.page_number = page_number;
            let query = format!(
                "_t={}&day={}&countryCode={}&column=createtime&order=desc&gatewayDr=000&pageNo={}&pageSize=100",
                timestamp, self.date, self.country_code, page_number
            );
            // if error log and return
            let response = match self.fetch_url(&query).await {
                Ok(response) => response,
                Err(e) => {
                    // Handle other errors
                    error!("fetch error: {}", e);
                    log_failed_operation(self, &e.to_string()).unwrap();
                    return Err(e);
                }
            };
            let v: Value = serde_json::from_str(&response)?;
            let records = v["result"]["records"].as_array().unwrap();
            self.store_data_csv(records, &store_file_name).unwrap();
            sleep(Duration::from_secs_f32(0.5)).await;
        }

        Ok("Data fetched successfully".to_string())
    }

    fn store_data_csv(&self, data: &Vec<Value>, file_name: &str) -> Result<()> {
        let desktop_path =
            desktop_dir().unwrap_or_else(|| home_dir().expect("Home directory not found"));
        let data_folder = desktop_path.join("data");
        let binding = data_folder.clone();
        if !data_folder.exists() {
            fs::create_dir(data_folder).unwrap();
        }
        let data_folders = binding.to_str().unwrap();

        // Adjust `file_name` to include the "data" folder in its path
        let file_name = format!("{}/{}", data_folders, file_name);

        let file_exists = Path::new(&file_name).exists();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&file_name)
            .unwrap();

        let mut headers = HashSet::new();
        for item in data {
            if let Value::Object(obj) = item {
                for key in obj.keys() {
                    headers.insert(key.clone());
                }
            }
        }
        // Convert headers HashSet into a Vec<String> and sort it for consistent column order
        let mut headers: Vec<String> = headers.into_iter().collect();
        headers.sort();

        // Create a CSV writer
        let mut wtr = WriterBuilder::new().from_writer(file);

        // Write the headers to the CSV only if the file did not exist before
        if !file_exists {
            wtr.write_record(&headers).unwrap();
        }

        // Now, write each record
        for item in data {
            if let Value::Object(obj) = item {
                // Create a record where each cell corresponds to a header, using empty string for missing keys
                let record: Vec<String> = headers
                    .iter()
                    .map(|header| {
                        obj.get(header).map_or_else(
                            || "".to_string(),
                            |value| match value {
                                Value::String(s) => s.clone(),
                                _ => value.to_string(),
                            },
                        )
                    })
                    .collect();

                // Write the record to the CSV file
                wtr.write_record(&record).unwrap();
            }
        }

        // Ensure all writes are flushed to the CSV file
        wtr.flush().unwrap();
        // Implement this function to store the data in CSV format
        Ok(())
    }

    pub async fn start(&mut self) -> Result<String> {
        // Use the runtime to block on the async functions
        self.get_token().await?;
        self.fetch_data().await
    }

    pub async fn status() -> Result<String> {
        let status = SPIDER_STATUS.lock().await;
        Ok(status.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_fetch_data_with_query() {
        initialize_logger();
        let mut spider = Spider::new("", "", "2024-06-23", "All", 1);
        let _ = spider.get_token().await;
        let result = spider.fetch_data().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_url_with_query() {
        initialize_logger();
        let mut spider = Spider::new("", "", "2024-06-23", "All", 1);
        let _ = spider.get_token().await;
        let query = format!("_t={}&day={}&countryCode={}&column=createtime&order=desc&gatewayDr=000&pageNo=1&pageSize=100", Utc::now().timestamp(), spider.date,spider.country_code);
        let result = spider.fetch_url(&query).await;
        assert!(result.is_ok());
    }

    // This test checks for failure in token retrieval due to network or other errors
    #[tokio::test]
    async fn test_get_token() {
        initialize_logger();
        let mut spider = Spider::new("", "", "2024-06-24", "All", 1);
        assert_eq!(spider.token, "");
        let result = spider.get_token().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_spider_new_with_valid_credentials() {
        let spider = Spider::new("testuser", "testpass", "2023-04-01", "Brazil", 1);
        assert_eq!(spider.username, "testuser");
        assert_eq!(spider.password, "testpass");
        assert_eq!(spider.date, "2023-04-01");
        assert_eq!(spider.country_code, "0055");
        assert_eq!(spider.token, "");
    }

    #[test]
    fn test_spider_new_with_empty_username_and_password() {
        let spider = Spider::new("", "", "2023-04-01", "India", 1);
        assert_eq!(spider.username, "456bet");
        assert_eq!(spider.password, "456bet888");
        assert_eq!(spider.date, "2023-04-01");
        assert_eq!(spider.country_code, "0091");
        assert_eq!(spider.token, "");
    }

    #[test]
    fn test_spider_new_with_unsupported_country() {
        let spider = Spider::new("testuser", "testpass", "2023-04-01", "All",1);
        assert_eq!(spider.country_code, "");
    }
}
