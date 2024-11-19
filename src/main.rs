use static_web_server::ThreadPool;
use std::{
    fs,
    //Let read and write into the stream
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

struct WebServer {
    address: String,      // The address to bind the server to (e.g., "127.0.0.1:8080")
    directory: String,    // The directory to serve files from
}

impl WebServer
{
    //Creates a new instance of Web Server
    pub fn new(address: String, directory: String) -> Self
    {
        WebServer{address, directory,}
    }

    pub fn run(&self)
    {
        //bind -> connect to a port to listen
        //Unwrap stops the program if errors happen
        //Returns a new instance of TcpListener
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        
        //Create 4 threads in a ThreadPool
        let pool = ThreadPool::new(4);

        //A stream is an open connection between a client and a server
        //Incoming method returns an iterator with a sequence of streams
        //Process each connection attempt  and produce streams to handle
        for stream in listener.incoming().take(2) //Only accepts two request before shutting down
        {
            let stream = stream.unwrap(); 

            pool.execute(|| {
                handle_connection(stream);
            });
        }

        println!("Shutting down.");
    }
}

fn main() {
    let server = WebServer::new("127.0.0.1:8080".to_string(), "./static".to_string());
    println!("Server initialized at {} serving files from {}", server.address, server.directory);
    server.run();
}

fn handle_connection(mut stream: TcpStream) {
    //BufReader adds buffereng by managing calls to the st::io::Read trait method
    //New BuffReader instance that wraps a mutable reference to the stream
    let buf_reader = BufReader::new(&mut stream);

    //Next gets the first item of the iterator
    //The first unwrap is to stop if the iterator has no items
    //The second unwrap handles the result
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "home.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "home.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}