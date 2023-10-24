// lib.rs

use rusqlite::{Connection, Result};


pub fn execute_cud_query(query: &str) -> Result<Vec<String>, rusqlite::Error> {
    let conn = Connection::open("bookstore_inventory.db")?;

    match conn.execute(query, []) {
        Ok(_) => {
            let first_word = query.split_whitespace().next().unwrap_or("Query");
            let success_message = format!("Success: {} performed", first_word);
            Ok(vec![success_message])
        }
        Err(e) => Err(e),
    }
}


pub fn execute_read_query(query: &str) -> Result<Vec<std::collections::HashMap<String, String>>, rusqlite::Error> {
    let conn = Connection::open("bookstore_inventory.db")?;

    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        let mut result = std::collections::HashMap::new();
        result.insert("Title".to_string(), row.get("Title")?);
        result.insert("Author".to_string(), row.get("Author")?);
        result.insert("Genre".to_string(), row.get("Genre")?);
        let price: f64 = row.get("Price")?;
        result.insert("Price".to_string(), price.to_string());
        let stock: f64 = row.get("Stock")?;
        result.insert("Stock".to_string(), stock.to_string());
        Ok(result)
    })?;

    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }

    Ok(data)
}


