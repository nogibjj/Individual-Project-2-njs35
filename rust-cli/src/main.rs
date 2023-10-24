use rust_cli::{execute_cud_query, execute_read_query};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <query>", args[0]);
        return;
    }

    let query = &args[1..].join(" ");

    let first_word = query.split_whitespace().next().unwrap_or("").to_lowercase();

    if first_word == "select" || first_word == "show" {
        match execute_read_query(query) {
            Ok(data) => {
                for item in data {
                    print!("Title: {}, Author: {}, Genre: {}, Price: {}, Stock: {}\n",
                        item.get("Title").unwrap_or(&"".to_string()),
                        item.get("Author").unwrap_or(&"".to_string()),
                        item.get("Genre").unwrap_or(&"".to_string()),
                        item.get("Price").unwrap_or(&"".to_string()),
                        item.get("Stock").unwrap_or(&"".to_string())
                    );
                }
            }
            Err(e) => println!("Error executing read query: {:?}", e),
        }
    } else if first_word == "insert" || first_word == "update" || first_word == "delete" {
        match execute_cud_query(query) {
            Ok(msg) => println!("{}", msg[0]), // Assuming the message is the first element in the vector
            Err(e) => println!("Error executing CUD query: {:?}", e),
        }
    } else {
        println!("Unsupported operation: {}", first_word);
    }
}
