//! use the sqlite db to store the data

use rusqlite::{params, Connection, Result};

use crate::spider::Spider;

/// Establishes a connection to the default SQLite database.
fn connect_to_default_db() -> Result<Connection> {
    let conn = Connection::open("spider.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS failed_operations (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            date TEXT NOT NULL,
            country_code TEXT NOT NULL,
            token TEXT NOT NULL,
            page_number INTEGER NOT NULL,
            failure_reason TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
         )",
        [],
    )?;
    Ok(conn)
}

/// log the failed spider operation, for some function, like rerun the last failure spider.
pub fn log_failed_operation(spider: &Spider, failure_reason: &str) -> Result<usize> {
    let conn = connect_to_default_db().unwrap();
    conn.execute(
        "INSERT INTO failed_operations (
            username, password, date, country_code, token, page_number, failure_reason
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            spider.username,
            spider.password,
            spider.date,
            spider.country_code,
            spider.token,
            spider.page_number,
            failure_reason,
        ],
    )
}

pub fn return_failed_operation() -> Result<Vec<Spider>> {
    let conn = connect_to_default_db().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM failed_operations")?;
    let spider_iter = stmt.query_map([], |row| {
        Ok(Spider {
            username: row.get(1)?,
            password: row.get(2)?,
            date: row.get(3)?,
            country_code: row.get(4)?,
            token: row.get(5)?,
            page_number: row.get(6)?,
        })
    })?;
    let mut spiders = Vec::new();
    for spider in spider_iter {
        spiders.push(spider.unwrap());
    }
    Ok(spiders)
}

// test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_failed_operation() {
        let result = return_failed_operation();
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_log_failed_operation() {
        let spider = Spider {
            username: "test".to_string(),
            password: "test".to_string(),
            date: "2021-01-01".to_string(),
            country_code: "US".to_string(),
            token: "test".to_string(),
            page_number: 1,
        };
        let failure_reason = "test";
        let result = log_failed_operation(&spider, failure_reason);
        assert_eq!(result.is_ok(), true);
    }
}
