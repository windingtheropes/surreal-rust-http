use std::io::{stdin, stdout, prelude::*};
use std::env;
use json::JsonValue::{Null};
use surreal_http::*;
use std::collections::HashMap;

fn get_input(prompt: String) -> String {
    print!("{} ", prompt);
    let mut s = String::new();
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Invalid string provided.");
    s.trim().to_string()
}
struct RuntimeOptions {
    database: String,
    namespace: String,
    user: String,
    pass: String,
    address: String,
}
impl RuntimeOptions {
    fn new(args: Vec<String>) -> Option<RuntimeOptions> {
        let mut options: HashMap<String, String> = HashMap::new();
        let string_options = vec!["address", "ns", "db", "user", "pass"];
        for arg in args {
            if arg.starts_with("--") {
                let parts = arg.split("=").collect::<Vec<&str>>();
    
                if parts.len() < 2 { continue }
                if parts.len() > 2 { continue }
                
                let key = parts[0].replace("--", "");
                let value = parts[1];
                
                options.insert(key, value.to_string());
            }
        }
        

        for o in string_options {
            if !options.contains_key(&o.to_owned()) {
                println!("Missing --{} option.", o);
                return None
            }
        }

        Some(RuntimeOptions {
            namespace: options.get("ns").unwrap().to_string(),
            database: options.get("db").unwrap().to_string(),
            address: options.get("address").unwrap().to_string(),
            user: options.get("user").unwrap().to_string(),
            pass: options.get("pass").unwrap().to_string()
        })
        
    }   
}

fn main() -> std::io::Result<()> {
    
    
    // Get the runtime arguments, then remove the first, which is the executable
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);

    let options = RuntimeOptions::new(args);
    match options {
        Some(_) => {},
        None => {
            return Err(std::io::ErrorKind::Other.into());
        }
    }
    let options = options.unwrap();

    // Initialize tcpstream
    println!("Connecting to SurrealDB at {}", &options.address);

    let config = DbConfig {
        database: options.database.clone(),
        namespace: options.namespace.clone(),
        user: options.user.clone(),
        pass: options.pass.clone(),
        address: options.address.clone(),
    };

    let handler = DbHandler::new(config);
    match handler {
        Err(e) => {
            return Err(e)
        },
        _ => {}
    }
    let mut handler = handler.unwrap();
    
    
    loop {
        let command = get_input(format!("SURREALDB @ {}>", options.address));
        if command == "EXIT" {
            return Ok(())
        }
        
        let response = handler.run_command(command.trim().to_string()).unwrap();
        let parsed_body = json::parse(&response.body).unwrap();

        // If there is a code found, this is an erorr response
        let code = &parsed_body["code"];
        if code.to_owned() != Null {
            if code.to_string().starts_with("4") {
                let information = &parsed_body["information"];
                println!("{}[{}] {}{}", "\u{001b}[31m", code, information, "\u{001b}[0m");
            }
        }

        // If the result is inside an array, the request went through as SQL
        let interior = &parsed_body[0];
        if interior.to_owned() != Null {
            let status = &interior["status"];
            let time = &interior["time"];
            let result = &interior["result"]; // result for success
            let detail = &interior["detail"]; // detail for errors

            if status == "ERR" {
                println!("{}{} in {}{}", "\u{001b}[33m", status, time, "\u{001b}[0m");

                println!();

                println!("{}", detail);
            }
            else if status == "OK" {
                println!("{}{} in {}{}", "\u{001b}[36m", status, time, "\u{001b}[0m");

                println!("{}", interior);

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

            
        }
        // println!("{}", &parsed_body);
    }
}