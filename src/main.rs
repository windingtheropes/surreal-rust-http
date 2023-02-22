use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::str;
use json::*;
fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8000")?;

    let body = String::from("CREATE account SET name = 1");
    let db = "test";
    let ns = "test";

    // Format headers and request, break lines at \r\n and split headers and body with \r\n\r\n
    let headers = vec![
        "POST /sql HTTP/1.1".to_string(),
        "Host: localhost:8000".to_string(),
        "Connection: keep-alive".to_string(),
        format!("Authorization: Basic {}", "cm9vdDpyb290"),
        "Accept: application/json".to_string(),
        "Content-Type: text/plain".to_string(),
        format!("DB: {}", db),
        format!("NS: {}", ns),
        format!("Content-Length: {}", body.len())
    ].join("\r\n");
    let request = [headers, body].join("\r\n\r\n");

    // Send the request
    stream.write(request.as_bytes())?;
    
    // Read the response
    let mut buf = [0; 512];
    stream.read(&mut buf[..])?;
    let response = str::from_utf8(&buf).unwrap();

    // Parse the response into two parts: headers and body
    let response_parts: Vec<&str> = response.split("\r\n\r\n").collect();
    let (headers, body) = (response_parts[0], response_parts[1].trim());

    // Print the parsed information from the response
    println!("Headers:\n{}\n", headers);
    // Body needs to be cleaned up, unwanted escapes at the end.
    println!("Body:\n{:?}\n", body);
    
    Ok(())

}