// main.rs

use rust_cli::{execute_cud_query, execute_read_query, OperationResult};
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
            OperationResult::Read(data) => {
                for item in data {
                    println!("{}", item);
                }
            }
            OperationResult::Error(msg) => println!("{}", msg),
            _ => {}
        }
    } else if first_word == "insert" || first_word == "update" || first_word == "delete" {
        match execute_cud_query(query) {
            OperationResult::Success(msg) => println!("{}", msg),
            OperationResult::Error(msg) => println!("{}", msg),
            _ => {}
        }
    } else {
        println!("Unsupported operation: {}", first_word);
    }
}
