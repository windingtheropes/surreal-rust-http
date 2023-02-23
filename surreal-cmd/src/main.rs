use std::net::TcpStream;
use std::io::{stdin, stdout, prelude::*};
use std::env;
use json::JsonValue::{self, Null};
use json::object::{self};
use surrealHttp::*;

fn get_input(prompt: String) -> String {
    print!("{} ", prompt);
    let mut s = String::new();
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Invalid string provided.");
    s.trim().to_string()
}

fn main() -> std::io::Result<()> {
    
    
    // Get the runtime arguments, then remove the first, which is the executable
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);

    // Arguments and default values
    let mut address = "localhost:8000".to_string();
    // app.exe --arg=value
    for arg in args {
        if arg.starts_with("--") {
            let parts = arg.split("=").collect::<Vec<&str>>();

            if parts.len() < 2 { continue }
            if parts.len() > 2 { continue }
            
            let key = parts[0].replace("--", "");
            let value = parts[1];
            
            if key == "address" {
                address=value.to_string()
            }
        }
    }

    // Initialize tcpstream
    println!("Connecting to SurrealDB at {}", &address);

    let config = DbConfig {
        database: "test".to_string(),
        namespace: "test".to_string(),
        user: "root".to_string(),
        pass: "root".to_string(),
        address: address.clone(),
    };

    let mut handler = DbHandler::new(config);

    loop {
        let command = get_input(format!("SURREALDB @ {}>", address));
        if command == "EXIT" {
            return Ok(())
        }

        let response = handler.run_command(command.trim().to_string());
        let parsed_body = json::parse(&response.body).unwrap();

        // If there is a code found, this is an erorr response
        let code = &parsed_body["code"];
        if code.to_owned() != Null {
            if code.to_string().starts_with("4") {
                let information = &parsed_body["information"];
                println!("{}[{}] {}{}", "\u{001b}[31m", code, information, "\u{001b}[0m");
            }
        }

        // If the result is inside an array, the request worked
        let interior = &parsed_body[0];
        if interior.to_owned() != Null {
            let status = &interior["status"];
            let time = &interior["time"];
            let result = &interior["result"];

            println!("{}{} in {}{}", "\u{001b}[36m", status, time, "\u{001b}[0m");

            println!();

            // split the result into lines, then print them on their own lines
            // separate objects by another linebreak by checking char1 for an opening curly brace
            let lines = result.to_string();
            let lines = lines.split_inclusive(",").collect::<Vec<&str>>();

            for line in lines {
                if line.starts_with("{") {
                    println!("\n{}", line);
                }
                else {
                    println!("{}", line);
                } 
            }
        }
        // println!("{}", &parsed_body);
    }
}