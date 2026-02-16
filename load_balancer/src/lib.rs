use std::{net::{IpAddr, TcpStream, ToSocketAddrs}, os::unix::net::SocketAddr, sync::{Arc, Mutex}, time::Duration};

pub struct LoadBalancer {
    current: Arc<Mutex<i32>>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        LoadBalancer {
            current: Arc::new(Mutex::new(0)),
        }
    }

    // Get the next server using round robin algorithm
    pub fn get_next_server(&mut self, servers: &Vec<Server>) -> Option<Server> {
        let mut current = self.current.lock().unwrap();
        let len = servers.len() as i32;

        for _ in 0..len {
            // get index of the current server
            let idx = *current % len as i32;
            *current += 1;

            // check if the server is healthy
            let server = servers[idx as usize].clone();
            if server.health_check() {
                return Some(server);
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Server {
    pub url: String,
    is_healthy: Arc<Mutex<bool>>,
}

impl Server {
    pub fn new(url: String) -> Self {
        Server {
            url,
            is_healthy: Arc::new(Mutex::new(true)),
        }
    }

    pub fn health_check(&self) -> bool {
        // use tcp to check the server and set health as either true or false
        // convert the address to IPV4
        println!("Checking server health: {}", self.url);
        
        let socket_addr = self.url.to_socket_addrs().expect("Error converting url").next().expect("No Ip address found");
        let is_alive = TcpStream::connect_timeout( &socket_addr, Duration::from_secs(1)).is_ok();
        println!("Server {} is alive? {}", self.url, is_alive);
        self.set_health(is_alive);
        is_alive
    }

    pub fn set_health(&self, healthy: bool) {
        let mut health = self.is_healthy.lock().unwrap();
        *health = healthy;
    }
}
