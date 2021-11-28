use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;


fn main() {
    // Usually should not unwrap and instead handle execeptions gracefully
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = match fs::read_to_string("hello.html") {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    // To ensure a valid HTTP response, we add the Content-Length header which is set to the size of our response body
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // Usually should not unwrap and instead handle execeptions gracefully
    stream.write(response.as_bytes()).unwrap();
    // Flush will wait and prevent the program from continuing until all the bytes are written to the connection
    stream.flush().unwrap();

}
