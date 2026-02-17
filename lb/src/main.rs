use std::{
    fs,
    io::{BufRead, BufReader, Read, Result, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use lb::{Config, LB, Server};
use serde_json::ser;

fn main() {
    // Load the config
    let config = match load_config() {
        Ok(v) => v,
        Err(e) => {
            panic!("Error loading config, {e}")
        }
    };

    // parse the health config
    let health_check_interval = Duration::from_secs(
        config
            .health_check_interval
            .parse::<u64>()
            .expect("Failed to parse interval"),
    );

    let mut servers: Vec<Server> = Vec::new();
    // get the servers from config and create their structs and loop through them
    for current_server in &config.servers {
        let server = Server::new(current_server.to_string());

        // add to array of servers
        servers.push(server);
    }

    println!("Checking Servers");
    for server in &servers {
        let mut s = server.clone();
        thread::spawn(move || {
            check_health(&mut s, health_check_interval);
        });
    }

    // start the Load balancer
    let mut lb = LB::new();

    // start a new http server and listen to "/" route
    println!("Load Balancer started");

    let addr = "127.0.0.1:".to_string() + &config.port.to_string();

    println!("Addr: {addr}");

    // Start a single threaded web server
    let listener = TcpListener::bind(addr).expect("Could not start up Tcp server");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_connection(stream, &mut lb, &servers);
    }
}

fn load_config() -> Result<Config> {
    // load the file using fs read to string
    let config = match fs::read_to_string("config/config.json") {
        Ok(v) => v,
        Err(e) => {
            panic!("Error loading file, {e}")
        }
    };
    // parse into json using serde_json
    let parsed_config = serde_json::from_str(&config).expect("Error Parsing config");
    Ok(parsed_config)
}

fn check_health(server: &mut Server, interval: Duration) {
    loop {
        let health = server.check_health();
        if !health {
            println!("{} is down", server.url);
        }
        thread::sleep(interval);
    }
}

fn handle_connection(mut stream: TcpStream, lb: &mut LB, servers: &Vec<Server>) {
    // Handle the first responses (get the socket address)
    let peer = stream.peer_addr().unwrap();
    let mut buf_reader = BufReader::new(&mut stream);
    println!("Received Request from {}", peer.ip());

    // Read full HTTP request (headers)
    let request_lines: Vec<String> = Vec::new();
    let mut request_line = String::new();

    loop {
        let mut line = String::new();
        let bytes = buf_reader.read_line(&mut line).unwrap();

        if bytes == 0 {
            return;
        }

        let trimmed = line.trim_end().to_string();
        println!("{}", trimmed);

        if trimmed.is_empty() { // End of headers
            break;
        }

        if request_line.is_empty() {
            request_line = trimmed.clone();
        }
    }
    
    if request_line == "GET / HTTP/1.1" {
        // send some form of response
        // get the next server
        match lb.get_next(servers) {
            Some(server) => {
                // Connect to the server
                match TcpStream::connect(&server.url) {
                    Ok(mut backend_stream) => {
                        // Write to the server
                        backend_stream
                            .write_all(format!("{}\r\n\r\n", request_line).as_bytes())
                            .expect("Cannot write to server");

                        // Read from the server
                        let mut backend_reader = BufReader::new(&backend_stream);
                        let mut buffer: Vec<u8> = Vec::new();

                        // read to the buffer
                        backend_reader.read_to_end(&mut buffer).unwrap();

                        // Read response from backend
                        let mut buffer = [0u8; 4096];

                        loop {
                            let n = backend_stream.read(&mut buffer).unwrap();
                            if n == 0 {
                                break; // backend closed connection
                            }

                            // Send backend response to client
                            stream.write_all(&buffer[..n]).unwrap();
                        }
                    }
                    Err(_) => {
                        // send a 503 error to the user
                        send_http_response(
                            &mut stream,
                            "422 unprocessible entity",
                            "Unprocessible entity",
                        )
                        .unwrap();
                    }
                }
            }
            None => {
                // send a 503 error to the user
                send_http_response(
                    &mut stream,
                    "503 Service Unavailable",
                    "No healthy server available",
                )
                .unwrap();
            }
        }
    }
}

fn send_http_response(
    stream: &mut TcpStream,
    status_code: &str,
    body: &str,
) -> std::io::Result<()> {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        body.len(),
        body
    );

    // Write using stream
    stream
        .write_all(response.as_bytes())
        .expect("Failed to write");
    Ok(())
}


