use std::io::{stdin, stdout, prelude::*};
use std::str;
use std::net::TcpStream;
use std::env;
use json::*;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

struct DbConfig {
    database: String,
    namespace: String, 
    user: String, 
    pass: String
}

impl DbConfig {
    fn get_auth_string(&self) -> String {
        general_purpose::STANDARD.encode(format!("{}:{}", &self.user, &self.pass).as_bytes())
    }
}
fn get_header<'a>(header: String, headers: &'a Vec<&'a str>) -> Option<&'a str> {
    for h in headers {
        if h.starts_with(&header) {
            let parts = h.split(":").collect::<Vec<&str>>();
            let value = parts[1].trim();
            return Some(value)
        }
    }
    None
}

fn run_command(stream: &mut TcpStream, config: &DbConfig, command: String) {
    // Format headers and request, break lines at \r\n and split headers and body with \r\n\r\n
    let headers = vec![
        "POST /sql HTTP/1.1".to_string(),
        "Host: localhost:8000".to_string(),
        "Connection: keep-alive".to_string(),
        format!("Authorization: Basic {}", config.get_auth_string()),
        "Accept: application/json".to_string(),
        "Content-Type: text/plain".to_string(),
        format!("DB: {}", config.database),
        format!("NS: {}", config.namespace),
        format!("Content-Length: {}", command.len())
    ].join("\r\n");
    let request = [headers, command].join("\r\n\r\n");

    // Send the request
    stream.write(request.as_bytes()).unwrap();
    
    // Read the response
    let mut buf = [0; 32768];
    stream.read(&mut buf[..]).unwrap();
    let response = str::from_utf8(&buf).unwrap();

    // Parse the response into two parts: headers and body
    let response_parts: Vec<&str> = response.split("\r\n\r\n").collect();
    let (headers, body_bytes) = (response_parts[0].lines().collect::<Vec<&str>>(), response_parts[1].as_bytes());

    // Parse headers
    let content_length = get_header(String::from("content-length"), &headers).unwrap().parse::<u32>().unwrap();

    // Print the parsed information from the response
    // println!("Headers:\n{:?}\n", headers);
    
    // Clear empty bytes from body
    let mut filtered_bytes: Vec<u8>=  vec![];
    for i in 0..content_length {
        filtered_bytes.push(body_bytes[i as usize]);
    }

    // Body JSON as string
    let body = str::from_utf8(&filtered_bytes).unwrap();

    // Parsed body json
    // println!("{body}");
    let parsed = json::parse(&body).unwrap();
    if parsed.members().len() > 0 {
        let response = &parsed[0];
        println!("Result:\n{}\n", response["result"]);
    }
}

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

    loop {
        let command = get_input("ENTER CAPS LOCK TEXT >".to_string());
        if command == "EXIT" {
            break;
        }
        run_command(&mut stream, &config, command.trim().to_string());
    }

    
    Ok(())
}