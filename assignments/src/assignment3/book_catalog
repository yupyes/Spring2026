use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // Create the file (or overwrite it if it exists)
    let mut file = File::create(filename).expect("Could not create file");

    for book in books {
        // Write each book as a comma-separated line
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .expect("Failed to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        
        // Split the line by commas into a vector of strings
        let parts: Vec<&str> = line.split(',').collect();

        // Ensure we have exactly 3 parts (title, author, year)
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            // Parse the year string into a u16
            let year = parts[2].parse::<u16>().expect("Could not parse year");

            books.push(Book { title, author, year });
        }
    }
    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}