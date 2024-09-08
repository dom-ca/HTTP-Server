/* Simple HTTP Server */
/* Author : Mariana V & Dominic C */
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    /* Creating a Local TcpListener at Port 8477 */
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";
    /* Concatenating Host address and Port to Create Final Endpoint */
    let end_point: String = HOST.to_owned() + ":" + PORT;
    /*Creating TCP Listener at our end point */
    let listener = TcpListener::bind(end_point).unwrap();
    println!("Web server is listening at port {}", PORT);
    /* Conneting to any incoming connections */
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        // Call function to process any incomming connections
        handle_connection(_stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response_contents = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_contents.len(),
        response_contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
