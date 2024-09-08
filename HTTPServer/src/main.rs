/* HTTP Server */
/* Author : Mariana V & Dominic C */
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";
    let end_point: String = HOST.to_owned() + ":" + PORT;
    let listener = TcpListener::bind(end_point).unwrap();
    println!("Web server is listening at port {}", PORT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Convertir el buffer en cadena
    let request = String::from_utf8_lossy(&buffer[..]);

    // Rutas específicas para cada operación
    let get = "GET / HTTP/1.1";
    let post = "POST /data HTTP/1.1";
    let put = "PUT /data HTTP/1.1";
    let delete = "DELETE /data HTTP/1.1";

    // Procesar solicitud y respuesta basada en la ruta y método
    let (status_line, contents) = if request.starts_with(get) {
        ("HTTP/1.1 200 OK", fs::read_to_string("index.html").unwrap())
    } else if request.starts_with(post) {
        let body = extract_body(&request);
        println!("POST data received: {}", body);
        ("HTTP/1.1 200 OK", format!("POST data: {}", body))
    } else if request.starts_with(put) {
        let body = extract_body(&request);
        println!("PUT data received: {}", body);
        ("HTTP/1.1 200 OK", format!("PUT data updated: {}", body))
    } else if request.starts_with(delete) {
        ("HTTP/1.1 200 OK", "Resource deleted".to_string())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "Resource not found".to_string())
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Función para extraer el cuerpo de la solicitud (usado para POST y PUT)
fn extract_body(request: &str) -> &str {
    request.split("\r\n\r\n").nth(1).unwrap_or("")
}
