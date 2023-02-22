use std::net::TcpStream;
use std::io::{stdin, stdout, prelude::*};
use std::env;
use surrealHttp::*;

fn get_input(prompt: String) -> String {
    print!("{} ", prompt);
    let mut s = String::new();
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Invalid string provided.");
    s
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8000")?;

    let config = DbConfig {
        database: "test".to_string(),
        namespace: "test".to_string(),
        user: "root".to_string(),
        pass: "root".to_string()
    };

    let mut handler = DbHandler::new(config, &mut stream);

    loop {
        let command = get_input("SURREALDB @ http://localhost:8000>".to_string());
        if command == "EXIT" {
            break;
        }
        let response = handler.run_command(command.trim().to_string());
        println!("{:?}", response.body);
    }

    
    Ok(())
}