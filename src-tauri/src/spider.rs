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
        let pass = if password.eq("") { "456bet888"} else { password };

        Self {
            username: name.to_string(),
            password: pass.to_string(),
            date: date.to_string(),
            country_code: country_code.to_string(),
            token: "".to_string(),
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

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