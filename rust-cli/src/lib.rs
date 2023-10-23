// lib.rs

use rusqlite::{Connection, Result};
use std::fmt::Debug;

pub enum OperationResult<T: Debug> {
    Success(String),
    Read(T),
    Error(String),
}

pub fn execute_cud_query(query: &str) -> OperationResult<Vec<String>> {
    let conn = Connection::open("bookstore_inventory.db");
    match conn {
        Ok(conn) => {
            match conn.execute(query, []) {
                Ok(_) => {
                    let first_word = query.split_whitespace().next().unwrap_or("Query");
                    OperationResult::Success(format!("Success: {} performed", first_word))
                }
                Err(e) => OperationResult::Error(format!("Error executing query: {:?}", e)),
            }
        }
        Err(e) => OperationResult::Error(format!("Error opening database connection: {:?}", e)),
    }
}

pub fn execute_read_query(query: &str) -> OperationResult<Vec<String>> {
    let conn = Connection::open("bookstore_inventory.db");
    
    match conn {
        Ok(conn) => {
            match conn.prepare(query) {
                Ok(mut stmt) => {
                    let rows = stmt.query_map([], |row| row.get(0));
                    match rows {
                        Ok(rows) => {
                            let data: Vec<String> = rows.filter_map(Result::ok).collect();
                            OperationResult::Read(data)
                        }
                        Err(e) => OperationResult::Error(format!("Error querying data: {:?}", e)),
                    }
                }
                Err(e) => OperationResult::Error(format!("Error preparing query: {:?}", e)),
            }
        }
        Err(e) => OperationResult::Error(format!("Error opening database connection: {:?}", e)),
    }
}