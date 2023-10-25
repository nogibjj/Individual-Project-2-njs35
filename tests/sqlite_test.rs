use rust_cli::{execute_cud_query, execute_read_query};

#[test]
fn test_select() {
    let query = "SELECT * FROM books LIMIT 3";
    match execute_read_query(query) {
        Ok(data) => {
            for item in data {
                print!(
                    "Title: {}, Author: {}, Genre: {}, Price: {}, Stock: {}\n",
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
}

#[test]
fn test_cud() {
    //create
    let query = "INSERT INTO books (Title, Author, Genre, Price, Stock) VALUES ('The Great Gatsby TEST', 'F. Scott Fitzgerald', 'Fiction', 7.99, 3)";
    match execute_cud_query(query) {
        Ok(msg) => println!("{}", msg[0]), // Assuming the message is the first element in the vector
        Err(e) => println!("Error executing CUD query: {:?}", e),
    }
    //update
    let query = "UPDATE books SET Stock = 5 WHERE Title = 'The Great Gatsby TEST'";
    match execute_cud_query(query) {
        Ok(msg) => println!("{}", msg[0]), // Assuming the message is the first element in the vector
        Err(e) => println!("Error executing CUD query: {:?}", e),
    }
    //delete
    let query = "DELETE FROM books WHERE Title = 'The Great Gatsby TEST'";
    match execute_cud_query(query) {
        Ok(msg) => println!("{}", msg[0]), // Assuming the message is the first element in the vector
        Err(e) => println!("Error executing CUD query: {:?}", e),
    }
}
