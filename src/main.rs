use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpListener;

const BUFFER_SIZE: usize = 1024;

#[derive(Debug)]
enum ContentType {
    TextPlain,
    ApplicationJson,
}

#[derive(Debug)]
struct Request {
    method: String,
    resource: String,
    content_type: ContentType,
    body: String,
}

impl Request {
    fn new(raw: String) -> Self {
        println!("{raw}");
        let mut lines = raw.lines();
        let mut first_line = lines.next().expect("Invalid HTTP request!").split_whitespace();
        let method = first_line.next().expect("Missing Method name").to_string();
        let resource = first_line.next().expect("Missing resource name").to_string();
        let mut content_type = ContentType::TextPlain;
        while let Some(line) = lines.next() {
            let line = line.to_lowercase();
            let mut line = line.split(':');
            if let Some("content-type") = line.next() {
                let content = line.next().unwrap().trim();
                content_type = match content {
                    "text/plain" => ContentType::TextPlain,
                    _ => ContentType::ApplicationJson,
                }
            }
        }
        Request { method, resource, content_type, body: String::new() }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Couldn't start server on port 8080");
    // Intercept streams
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Create a reader to read bytes from the stream
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            // Full buffer
            let mut buffer: Vec<u8> = vec![];

            let mut buf = [0u8; BUFFER_SIZE];
            loop {
                let bytes_read = reader.read(&mut buf).unwrap();
                buffer.extend_from_slice(&buf[..bytes_read]);

                // Indicates we read the entire stream
                if bytes_read < BUFFER_SIZE {
                    break;
                }
            }
            let request = Request::new(String::from_utf8(buffer).unwrap());
            println!("{:?}", request);


            let mut writer = BufWriter::new(stream);
            writer
                .write(
                    b"HTTP/1.1 200 OK
Content-Type: text/plain

Hello there
",
                )
                .unwrap();
        }
    }
}
