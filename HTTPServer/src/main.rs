/* HTTP Server */
/* Author : Mariana V & Dominic C */
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::Arc;
use threadpool::ThreadPool;
use std::collections::HashMap; // Para almacenar sesiones simples

// Estructura para representar una sesión simple
struct Session {
    id: String,
}

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";
    let end_point: String = HOST.to_owned() + ":" + PORT;

    // Crear el listener del servidor
    let listener = TcpListener::bind(end_point).unwrap();
    println!("Web server is listening at port {}", PORT);

    // Crear un pool de hilos con 4 hilos
    let pool = ThreadPool::new(4);

    // Utilizar Arc para permitir que múltiples hilos compartan el listener
    let listener = Arc::new(listener);

    // HashMap para almacenar las sesiones (en memoria)
    let session_store: Arc<std::sync::Mutex<HashMap<String, Session>>> = Arc::new(std::sync::Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Clonar el TcpListener y el session_store para pasarlo al hilo
        let listener = Arc::clone(&listener);
        let session_store = Arc::clone(&session_store);

        // Enviar la conexión al thread pool para que sea procesada en un hilo
        pool.execute(move || {
            handle_connection(stream, session_store);
        });
    }
}

// Función principal que maneja la conexión
fn handle_connection(mut stream: TcpStream, session_store: Arc<std::sync::Mutex<HashMap<String, Session>>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let cookie = extract_cookie(&request);

    // Verificar si existe una sesión activa
    let session_id = get_session_id(&cookie, &session_store);

    // Procesar la solicitud
    let (status_line, contents, cookie_header) = process_request(&request, session_id, &session_store);

    // Enviar la respuesta
    send_response(stream, status_line, contents, cookie_header);
}

// Función que procesa la solicitud y devuelve los datos de la respuesta
fn process_request(
    request: &str,
    session_id: Option<String>,
    session_store: &Arc<std::sync::Mutex<HashMap<String, Session>>>
) -> (&'static str, String, Option<String>) {
    let get = "GET / HTTP/1.1";
    let post = "POST /data HTTP/1.1";
    let put = "PUT /data HTTP/1.1";
    let delete = "DELETE /data HTTP/1.1";
    let login = "GET /login HTTP/1.1";
    let logout = "GET /logout HTTP/1.1";

    if request.starts_with(get) {
        handle_get_request(session_id)
    } else if request.starts_with(post) {
        handle_post_request(request)
    } else if request.starts_with(put) {
        handle_put_request(request)
    } else if request.starts_with(delete) {
        handle_delete_request()
    } else if request.starts_with(login) {
        handle_login_request(session_store)
    } else if request.starts_with(logout) {
        handle_logout_request(session_id, session_store)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "Resource not found".to_string(), None)
    }
}

// Funciones específicas para manejar las diferentes solicitudes
fn handle_get_request(session_id: Option<String>) -> (&'static str, String, Option<String>) {
    if let Some(session_id) = session_id {
        (
            "HTTP/1.1 200 OK",
            format!("Welcome back, user with session: {}", session_id),
            None,
        )
    } else {
        (
            "HTTP/1.1 200 OK",
            fs::read_to_string("index.html").unwrap(),
            None,
        )
    }
}

fn handle_post_request(request: &str) -> (&'static str, String, Option<String>) {
    let body = extract_body(request);
    println!("POST data received: {}", body);
    ("HTTP/1.1 200 OK", format!("POST data: {}", body), None)
}

fn handle_put_request(request: &str) -> (&'static str, String, Option<String>) {
    let body = extract_body(request);
    println!("PUT data received: {}", body);
    ("HTTP/1.1 200 OK", format!("PUT data updated: {}", body), None)
}

fn handle_delete_request() -> (&'static str, String, Option<String>) {
    println!("DELETE request received");
    ("HTTP/1.1 200 OK", "Resource deleted".to_string(), None)
}

fn handle_login_request(
    session_store: &Arc<std::sync::Mutex<HashMap<String, Session>>>
) -> (&'static str, String, Option<String>) {
    println!("Logged in successfully");
    let new_session_id = "12345"; // Simulación de un ID de sesión único
    let session = Session {
        id: new_session_id.to_string(),
    };
    session_store.lock().unwrap().insert(new_session_id.to_string(), session);

    (
        "HTTP/1.1 200 OK",
        "Logged in successfully".to_string(),
        Some(format!("Set-Cookie: session_id={}; HttpOnly", new_session_id)),
    )
}

fn handle_logout_request(
    session_id: Option<String>,
    session_store: &Arc<std::sync::Mutex<HashMap<String, Session>>>
) -> (&'static str, String, Option<String>) {
    println!("Logged out successfully");
    if let Some(session_id) = session_id {
        session_store.lock().unwrap().remove(&session_id);
    }
    (
        "HTTP/1.1 200 OK",
        "Logged out successfully".to_string(),
        Some("Set-Cookie: session_id=deleted; Max-Age=0".to_string()),
    )
}

// Función para enviar la respuesta al cliente
fn send_response(mut stream: TcpStream, status_line: &str, contents: String, cookie_header: Option<String>) {
    let response = if let Some(cookie_header) = cookie_header {
        format!(
            "{}\r\n{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            cookie_header,
            contents.len(),
            contents
        )
    } else {
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        )
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Función para obtener el ID de la sesión de la cookie
fn get_session_id(cookie: &Option<String>, session_store: &Arc<std::sync::Mutex<HashMap<String, Session>>>) -> Option<String> {
    if let Some(session_cookie) = cookie {
        let session_store = session_store.lock().unwrap();
        if session_store.contains_key(session_cookie) {
            Some(session_cookie.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

// Función para extraer el cuerpo de la solicitud (usado para POST y PUT)
fn extract_body(request: &str) -> &str {
    request.split("\r\n\r\n").nth(1).unwrap_or("")
}

// Función para extraer la cookie del request (si existe)
fn extract_cookie(request: &str) -> Option<String> {
    request
        .lines()
        .find(|line| line.starts_with("Cookie:"))
        .and_then(|cookie_line| {
            cookie_line
                .split(';')
                .find(|part| part.trim().starts_with("session_id="))
                .map(|part| part.trim().replace("session_id=", ""))
        })
}
