use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).expect("Unable to create file");

    for book in books{
        writeln!(file, "{}, {}, {}", book.title, book.author, book.year).expect("Unable to write to file");
    }

    println!("Books saved to '{}'.", filename);
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    //Reading each line
    for line in reader.lines(){
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 3{
            let title = parts[0].trim().to_string();
            let author = parts[1].trim().to_string();
            let year = parts[2].trim().parse::<u16>().unwrap_or(0);
            //let year = parts[2].parse::<u16>().unwrap_or(000);

            books.push(Book{title, author, year});
        }
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "The Hobbit".to_string(), author: "J.R.R. Tolkien".to_string(), year: 1937 },
        Book { title: "The Maze Runner".to_string(), author: "James Dashner".to_string(), year: 2009 },
        Book { title: "The Hunger Games".to_string(), author: "Suzanne Collins".to_string(), year: 2008 },
        Book { title: "Harry Potter".to_string(), author: "J.K. Rowling".to_string(), year: 1997 },
        Book { title: "The Lord of the Rings".to_string(), author: "J.R.R. Tolkien".to_string(), year: 1954 },
        
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}