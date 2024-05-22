use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    dotenv::dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    print_guide();
    loop {
        println!("Please choose an option:");
        println!("1. Recover Keys");
        println!("2. Exit");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                if let Err(e) = recover_keys(&server_address) {
                    eprintln!("Error: {}", e);
                }
            },
            "2" => {
                println!("Exiting KeyFinder...");
                break;
            },
            _ => println!("Invalid option."),
        }
    }
}

fn print_guide() {
    println!("Welcome to KeyFinder!");
    println!("This tool helps you recover your keys securely.");
}

fn recover_keys(server_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your email associated with the keys:");
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim(); 

    if !email.contains('@') {
        eprintln!("Invalid email format. Please try again.");
        return Ok(());
    }

    let mut stream = TcpStream::connect(server_address)?;
    stream.write_all(email.as_bytes())?;
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let response = str::from_utf8(&buffer[..bytes_read])?;
    println!("Server response: {}", response);
    Ok(())
}