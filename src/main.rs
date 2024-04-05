use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpListener;

const BUFFER_SIZE: usize = 1024;

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
            let request = String::from_utf8(buffer).unwrap();
            println!("{}", request);


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
