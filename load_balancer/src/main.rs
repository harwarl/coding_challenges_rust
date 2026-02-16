/**
 * Build a Load Balancer that ->
 * - Distributes incoming request to a pool of servers
 * - Health Check the servers
 * - Handle a server going offline
 * - Handle a server coming back online
 */
mod lib;
use std::{
    fs::read_to_string, io::{BufRead, BufReader}, net::{TcpListener, TcpStream}, thread, time::{Duration, Instant}
};

use load_balancer::{LoadBalancer, Server};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    servers: Vec<String>,
    port: String,
    health_check_interval: String,
}

fn main() {
    // Create a server on port 80
    // load the config file
    let config = load_config();
    println!("config : {:?}", config);

    // parse for the health check interval
    let health_check_interval = config
        .health_check_interval
        .parse::<u64>()
        .expect("Invalid Health check Interval");

    let health_duration = Duration::from_secs(health_check_interval);

    // Start up a server on config port
    let mut servers: Vec<Server> = Vec::new();

    for current in &config.servers {
        // save the server
        let server = Server::new(current.clone());

        // append the server
        servers.push(server);

        // check the health of the server and allow threads to do this
        for server in &servers {
            let s = server.clone();

            thread::spawn(move || {
                check_health(&s, health_duration);
            });
        }
    }

    // start the load balancer
    let mut lb: LoadBalancer = LoadBalancer::new();

    // start a new http server and listen to "/" route
    println!("Load Balancer started");

    let addr = "127.0.0.1:".to_string() + &config.port.to_string();
    println!("address being: {}", addr);
    // Start a single threaded web server
    let listener = TcpListener::bind(addr).expect("Could not start up Tcp server");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut lb, &servers);
    }
}

fn load_config() -> Config {
    // Load the configuration from a json file
    let config_data =
        std::fs::read_to_string("config.json").expect("Failed to load the config file");
    serde_json::from_str(&config_data).expect("Failed to parse config data")
}

fn check_health(server: &Server, interval: Duration) {
    loop {
        // Check the server by making a http/tcp call to the server
        server.set_health(true);
        // println!("{} => {}", server.url, true);
        thread::sleep(interval);
    }
}

fn handle_connection(stream: TcpStream, lb: &mut LoadBalancer,servers: &Vec<Server>) {
    let buffer = BufReader::new(stream);

    let request_line = buffer.lines().next().unwrap().unwrap();

    println!("{}", request_line);

    if request_line == "GET / HTTP/1.1" {
        // ("HTTP/1.1 200 OK", "response.html")
        let server = match lb.get_next_server(servers) {
            Some(v) => v,
            None => {
                panic!("No healthy Server available");
            }
        };

        print!("server: {}", server.url);
    }

    // let contents = read_to_string("output.txt").unwrap();
    // let length = contents.len();
    // // let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);
    // // stream.write_all(response.as_bytes()).unwrap();
}
