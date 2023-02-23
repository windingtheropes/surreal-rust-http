use std::io::{prelude::*};
use std::str;
use std::net::TcpStream;
use base64::{Engine as _, engine::{general_purpose}};

const BUF_SIZE: usize = 16384; 

pub struct RequestResponse {
    pub headers: Vec<String>,
    pub body: String
}
impl RequestResponse {
    fn new(data: [u8; BUF_SIZE]) -> RequestResponse {
        let response = str::from_utf8(&data).unwrap();

        // Parse the response into two parts: headers and body
        let response_parts: Vec<&str> = response.split("\r\n\r\n").collect();
        let (headers, body_bytes) = (response_parts[0].lines().collect::<Vec<&str>>(), response_parts[1].as_bytes());
        
        // Make headers owned
        let headers = || -> Vec<String> {
            let mut a = vec![];
            for h in headers {
                let owned = h.to_owned();
                a.push(owned);
            }
            a
        }();

        // Parse headers
        let content_length = get_header(String::from("content-length"), &headers).unwrap().parse::<u32>().unwrap();

        // Create a new array which only includes the content-length of bytes, clearing unused bytes at the end of the buffer
        let mut filtered_bytes: Vec<u8>=  vec![];
        for i in 0..content_length {
            filtered_bytes.push(body_bytes[i as usize]);
        }

         // Convert filtered_bytes to a utf8 string, giving the response body
        let body = str::from_utf8(&filtered_bytes).unwrap().to_owned();
        
        // let data = data.to_owned();
        return RequestResponse { headers, body }
    }
}
pub struct DbConfig {
    pub database: String,
    pub namespace: String, 
    pub user: String, 
    pub pass: String,
    pub address: String,
}
impl DbConfig {
    fn get_auth_string(&self) -> String {
        general_purpose::STANDARD.encode(format!("{}:{}", &self.user, &self.pass).as_bytes())
    }
}

pub struct DbHandler {
    config: DbConfig,
    stream: TcpStream
}
impl DbHandler {
    pub fn run_command(&mut self, command: String) -> RequestResponse {
        // Format headers and request, break lines at \r\n and split headers and body with \r\n\r\n
        let headers = vec![
            "POST /sql HTTP/1.1".to_string(),
            "Host: localhost:8000".to_string(),
            "Connection: keep-alive".to_string(),
            format!("Authorization: Basic {}", self.config.get_auth_string()),
            "Accept: application/json".to_string(),
            "Content-Type: text/plain".to_string(),
            format!("DB: {}", self.config.database),
            format!("NS: {}", self.config.namespace),
            format!("Content-Length: {}", command.len())
        ].join("\r\n");
        let request = [headers, command].join("\r\n\r\n");

        // Send the request
        self.stream.write(request.as_bytes()).unwrap();
        
        // Read the response
        let mut buf = [0; BUF_SIZE];
        self.stream.read(&mut buf[..]).unwrap();

        return RequestResponse::new(buf)
    }
    pub fn new(config: DbConfig) -> DbHandler {
        let stream = TcpStream::connect(&config.address).unwrap();
        DbHandler { config, stream }
    }
}

fn get_header<'a>(header: String, headers: &Vec<String>) -> Option<&str> {
    for h in headers {
        if h.starts_with(&header) {
            let parts = h.split(":").collect::<Vec<&str>>();
            let value = parts[1].trim();
            return Some(value)
        }
    }
    None
}