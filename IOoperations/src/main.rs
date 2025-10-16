use std::io::{self, Write};
use std::process::Command;
use std::fs::File;
//use std::io::Write;

// struct Person {
//     name: String,
//     age: u32,
// }

//Function to read the input of the user
fn reading_from_console(message: &str) -> String {
    let mut buffer = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().to_string();
}

/// Function to read entire file using Rust standard I/O
// fn read_entire_file(filename: &String) {
//     let mut file = File::open(filename).unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     println!("\n File contents (entire):\n{}", contents);
// }

/// Function to read file line by line using BufReader
// fn read_file_line_by_line(filename: &str) {
//     let file = File::open(filename).unwrap();
//     let reader = BufReader::new(file);

//     println!("\nFile contents (line by line):");
//     for line in reader.lines() {
//         println!("{}", line.unwrap());
//     }
// }
//Function to read a file
fn read_file_linux(filename: String){
    let output = Command::new("cat")
        .arg(filename)
        .output()
        .expect("Failed to execute command");
    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}
//Function to create the file and write content
fn create_and_write_to_file(filename: &String) {
    let mut file = File::create(filename).unwrap();
    println!("Enter the content to write into the file (type END on a new line to finish):");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        if line.eq_ignore_ascii_case("END") {
            break;
        }
        writeln!(file, "{}", line).unwrap();
    }
    println!("File '{}' created and written successfully!", filename);
}

fn main(){
    loop{
        println!("What do you want to do today?");
        println!("1 => Create a file");
        println!("2 => Read from a file");
        println!("3 => Finish the program");

        let choice = reading_from_console("Type the number: ");

        match choice.as_str() {
            "1" => {
                let filename = reading_from_console("What file to create? ");
                create_and_write_to_file(&filename);
            }
            "2" => {
                let filename = reading_from_console("What file to open? ");
                read_file_linux(filename);

                // println!("\nChoose reading method:");
                // println!("1 => Read entire file");
                // println!("2 => Read line by line");
                // println!("3 => Read using Linux command");
                // let method = reading_from_console("Type your choice: ");

                // match method.as_str() {
                //     "1" => read_entire_file(&filename),
                //     "2" => read_file_line_by_line(&filename),
                //     "3" => read_file_linux(filename),
                //     _ => println!("Invalid reading method."),
                // }
            }
            "3" => {
                println!("Program finished. Goodbye!");
                break;
            }
            _ => println!("Invalid option, please enter 1, 2, or 3."),
        }
        // let message = "What do you want to do today? Type 1 to create file, 2 to read a file, 3 to finish the program".to_string();
        // println!("{}", message);
        // let choice = reading_from_console(&"Type the number".to_string()).parse().unwrap();

        // match choice{
        //     1 => {
        //         let message = "What file to create?".to_string();
        //         let filename = reading_from_console(&message);
        //         create_and_write_to_file(filename);
        //     }
        //     2 => {
        //         let message = "What file to open?".to_string();
        //         let filename = reading_from_console(&message);
        //         read_file_linux(filename);
        //     }
        //     3 => break,
        //     _ => {},
        // }
    }
    //Ask the user if he wants to create a file and write
    //=> create a file and write inside of it
    //accept everything from the console
    //filename he wants to create and actual content
    //refer to 03rust-file-operations.md

    //or he wants read from existent file

    //accept from the user file he wants to open
}
