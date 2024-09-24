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

fn handle_connection(mut stream: TcpStream, session_store: Arc<std::sync::Mutex<HashMap<String, Session>>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Convertir el buffer en cadena
    let request = String::from_utf8_lossy(&buffer[..]);

    // Rutas específicas para cada operación
    let get = "GET / HTTP/1.1";
    let post = "POST /data HTTP/1.1";
    let put = "PUT /data HTTP/1.1";
    let delete = "DELETE /data HTTP/1.1";
    let login = "GET /login HTTP/1.1";
    let logout = "GET /logout HTTP/1.1";

    // Leer cookies del request, si existen
    let cookie = extract_cookie(&request);

    // Verificar si hay una sesión existente
    let session_id = if let Some(session_cookie) = cookie {
        // Buscar si existe una sesión con esa cookie
        let session_store = session_store.lock().unwrap();
        if session_store.contains_key(&session_cookie) {
            Some(session_cookie)
        } else {
            None
        }
    } else {
        None
    };

    // Procesar solicitud y respuesta basada en la ruta y método
    let (status_line, contents, cookie_header) = if request.starts_with(get) {
        println!("GET Received");
        if let Some(session_id) = session_id {
            (
                "HTTP/1.1 200 OK",
                format!("Welcome back, user with session: {}", session_id),
                None,
            )
        } else {
            ("HTTP/1.1 200 OK", fs::read_to_string("index.html").unwrap(), None)
        }
    } else if request.starts_with(post) {
        let body = extract_body(&request);
        println!("POST data received: {}", body);
        ("HTTP/1.1 200 OK", format!("POST data: {}", body), None)
    } else if request.starts_with(put) {
        let body = extract_body(&request);
        println!("PUT data received: {}", body);
        ("HTTP/1.1 200 OK", format!("PUT data updated: {}", body), None)
    } else if request.starts_with(delete) {
        println!("DELTETE data received");
        ("HTTP/1.1 200 OK", "Resource deleted".to_string(), None)
    } else if request.starts_with(login) {
        println!("Logged in successfully");
        // Crear una nueva sesión para el usuario al hacer login
        let new_session_id = "12345"; // Simulación de un ID de sesión único
        let session = Session {
            id: new_session_id.to_string(),
        };
        session_store.lock().unwrap().insert(new_session_id.to_string(), session);

        // Establecer la cookie con el ID de sesión
        (
            "HTTP/1.1 200 OK",
            "Logged in successfully".to_string(),
            Some(format!("Set-Cookie: session_id={}; HttpOnly", new_session_id)),
        )
    } else if request.starts_with(logout) {
        println!("Logged out successfully");
        // Eliminar la sesión y la cookie al hacer logout
        if let Some(session_id) = session_id {
            session_store.lock().unwrap().remove(&session_id);
        }

        // Instrucción para eliminar la cookie al lado del cliente
        (
            "HTTP/1.1 200 OK",
            "Logged out successfully".to_string(),
            Some("Set-Cookie: session_id=deleted; Max-Age=0".to_string()),
        )
    } else {
        ("HTTP/1.1 404 NOT FOUND", "Resource not found".to_string(), None)
    };

    // Incluir la cabecera de la cookie si es necesario
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
