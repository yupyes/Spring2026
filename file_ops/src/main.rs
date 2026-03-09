use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let status = Command::new("ls")
                .arg(&path) // Added & to borrow instead of move
                .status()
                .expect("Failed to execute ls");

            if !status.success() {
                eprintln!("Error: Could not list directory");
            }
        }

        FileOperation::Display(path) => {
            let status = Command::new("cat")
                .arg(&path) // Added & to borrow instead of move
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                eprintln!("Error: Could not display file");
            }
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);

            let status = Command::new("sh")
                .arg("-c")
                .arg(&command) // Added & to borrow instead of move
                .status()
                .expect("Failed to create file");

            if status.success() {
                println!("File '{}' created successfully.", path);
            } else {
                eprintln!("Error: Could not create file");
            }
        }

        FileOperation::Remove(path) => {
            let status = Command::new("rm")
                .arg(&path) // Added & to borrow instead of move
                .status()
                .expect("Failed to remove file");

            if status.success() {
                println!("File '{}' removed successfully.", path);
            } else {
                eprintln!("Error: Could not remove file");
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if !status.success() {
                eprintln!("Error: Could not print working directory");
            }
        }
    }
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        print!("\nEnter your choice (0-5): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter directory path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::List(path.trim().to_string()));
            }

            "2" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::Display(path.trim().to_string()));
            }

            "3" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                print!("Enter content: ");
                io::stdout().flush().unwrap();
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();

                perform_operation(FileOperation::Create(
                    path.trim().to_string(),
                    content.trim().to_string(),
                ));
            }

            "4" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::Remove(path.trim().to_string()));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please enter a number between 0 and 5.");
            }
        }
    }
}