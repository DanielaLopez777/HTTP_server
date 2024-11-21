use std::io::{Read, Write};
use std::{thread, time};
include!("../src/main.rs");

fn start_server() {
    // Inicia el servidor en un hilo separado
    thread::spawn(|| {
        let server = WebServer::new("127.0.0.1:8080".to_string(), "./static".to_string());
        server.run();
    });

    // Espera unos segundos para asegurarte de que el servidor haya iniciado
    thread::sleep(time::Duration::from_secs(2));
}

fn make_request(path: &str) -> String {
    // Realiza una solicitud HTTP al servidor
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    
    // Envia una solicitud GET
    let request = format!("GET /{} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n", path);
    stream.write_all(request.as_bytes()).unwrap();
    
    // Lee la respuesta del servidor
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    
    response
}

#[test]
fn test_file_served_correctly() {
    start_server();

    // Realiza una solicitud para un archivo existente (por ejemplo, "home.html")
    let response = make_request("home.html");

    // Verifica que la respuesta sea correcta (código 200 OK)
    assert!(response.contains("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Type: text/html"));
}

#[test]
fn test_file_not_found() {
    start_server();

    // Realiza una solicitud para un archivo no existente (por ejemplo, "notfound.html")
    let response = make_request("notfound.html");

    // Verifica que la respuesta sea un error 404
    assert!(response.contains("HTTP/1.1 404 Not Found"));
}

#[test]
fn test_404_error_page_served() {
    start_server();

    // Solicita un archivo no existente, debe devolver la página de error personalizada 404.html
    let response = make_request("notfound.html");

    // Verifica que la respuesta contenga la página de error personalizada
    assert!(response.contains("HTTP/1.1 404 Not Found"));
    assert!(response.contains("Content-Type: text/html"));
}

#[test]
fn test_invalid_request() {
    start_server();

    // Realiza una solicitud HTTP inválida
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let invalid_request = "INVALID REQUEST\r\n";
    stream.write_all(invalid_request.as_bytes()).unwrap();

    // Lee la respuesta
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    // Verifica que la respuesta sea un error 400
    assert!(response.contains("HTTP/1.1 400 Bad Request"));
}
