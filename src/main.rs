use static_web_server::ThreadPool;
use std::{
    fs,
    //Let read and write into the stream
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};
use mime_guess::from_path; //From an url returns the mime type

pub struct WebServer {
    address: String,      // The address to bind the server to (e.g., "127.0.0.1:8080")
    directory: String,    // The directory to serve files from
}

 impl WebServer
{
    //Creates a new instance of Web Server
    pub fn new(address: String, directory: String) -> Self
    {
        WebServer{address, directory}
    }

    pub fn run(&self)
    {
        //bind -> connect to a port to listen
        //Unwrap stops the program if errors happen
        //Returns a new instance of TcpListener
        let listener = TcpListener::bind(&self.address).unwrap_or_else( |_| {
            panic!("Could not bind to address {}", self.address);
        });
        
        //Create 4 threads in a ThreadPool
        let pool = ThreadPool::new(4);

        //A stream is an open connection between a client and a server
        //Incoming method returns an iterator with a sequence of streams
        //Process each connection attempt  and produce streams to handle
        for stream in listener.incoming()
        {
            let stream = match stream
            {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Failed to establish a connection.");
                    continue;
                }
            };

            let directory = self.directory.clone();
            pool.execute( move || {
                handle_connection(stream, &directory);
            });
        }

        println!("Shutting down.");
    }
}

fn handle_connection(mut stream: TcpStream, directory: &str) {
    //BufReader adds buffereng by managing calls to the st::io::Read trait method
    //New BuffReader instance that wraps a mutable reference to the stream
    let buf_reader = BufReader::new(&mut stream);

    //Next gets the first item of the iterator
    //The first unwrap is to stop if the iterator has no items
    //The second unwrap handles the result
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => {
            respond_with_error(&mut stream, "400 Bad Request", "Invalid HTTP request.");
            return;
        }
    };

    //Extract the request path
    let requested_path = match parse_request(&request_line)
    {
        Some(path) => path,
        None => {
            respond_with_error(&mut stream, "400 Bad Request", "Invalid HTTP request.");
            return;
        }
    };
    
    //Determine the full path of the requested file
    let full_path = Path::new(directory).join(&requested_path);

    if full_path.is_dir()
    {
        let default_file = full_path.join("home.html");
        if let Ok(contents) = fs::read(&default_file)
        {
            let mime_type = from_path(&default_file).first_or_text_plain();
            respond_with_file(&mut stream, "200 OK", &mime_type.to_string(), &contents);
        }
        else
        {
            respond_with_error(&mut stream, "404 Not Found", "Default file not found.");
        }
        return;
    }

    match fs::read(&full_path)
    {
        Ok(contents) => {
            let mime_type = from_path(&full_path).first_or_text_plain();
            respond_with_file(&mut stream, "200 OK", &mime_type.to_string(), &contents);
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            let error_page = Path::new(directory).join("404.html");
            if let Ok(contents) = fs::read(&error_page)
            {
                let mime_type = from_path(&error_page).first_or_text_plain();
                respond_with_file(&mut stream, "404 Not Found", &mime_type.to_string(), &contents);
            }
            else 
            {
                respond_with_error(&mut stream, "404 Not Found", "File not found.");
            }
        }
        Err(_) => {
            respond_with_error(&mut stream, "500 Internal Server Error", "Server error.");
        }
    }
}

//Parses the HTTP request line to extract the requested path
fn parse_request(request_line: &str) -> Option<String> {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 || parts[0] != "GET"
    {
        return None;
    }

    let path = parts[1];
    Some(path.trim_start_matches('/').to_string())
}

//Sends an error response to the client
fn respond_with_error(stream: &mut TcpStream, status: &str, message: &str)
{
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).unwrap_or_else(|_| {
        eprintln!("Failed to send error response.");
    });
}

//Send a file response to the client
fn respond_with_file(stream: &mut TcpStream, status: &str, mime_type: &str, contents: &[u8])
{
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {mime_type}\r\nContent-Length: {}\r\n\r\n",
        contents.len()
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(contents).unwrap_or_else(|_| {
        eprintln!("Failed to send file content.");
    });
}
fn main() {
    let server = WebServer::new("127.0.0.1:8080".to_string(), "./static".to_string());
    println!("Server initialized at {} serving files from {}", server.address, server.directory);
    server.run();
}