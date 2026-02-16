use std::{
    io::Result,
    net::{TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
    time::Duration,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LB {
    current: Arc<Mutex<u32>>,
}

impl LB {
    /// Creates a new Load balancer
    pub fn new() -> LB {
        LB {
            current: Arc::new(Mutex::new(0)),
        }
    }

    pub fn get_next(&self, servers: &Vec<Server>) -> Option<Server> {
        let mut current = self.current.lock().unwrap();
        let len = servers.len();

        for _ in 0..len {
            // get the current index
            let idx = *current % len as u32;
            *current += 1;

            let mut server = servers[idx as usize].clone();
            if server.check_health() {
                return Some(server);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    pub url: String,
    is_healthy: Arc<Mutex<bool>>,
}

impl Server {
    /// Creates a new Server
    ///
    /// The url parameter is the url of the server
    /// The is_healthy tag is the unique identifier to know if the server is healthy during the last check and its constantly being updated
    pub fn new(url: String) -> Server {
        Server {
            url,
            is_healthy: Arc::new(Mutex::new(true)),
        }
    }

    pub fn check_health(&mut self) -> bool {
        // use Tcp to check if the url is valid and if connection is alive
        // convert string url to socketAddr
        let socket_addr = self
            .url
            .to_socket_addrs()
            .expect("Could not convert to socket Addrs")
            .next()
            .expect("No address found");

        // Make a timed out connect to the url
        let is_healthy = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(2)).is_ok();
        let health_set = self.set_health(is_healthy).is_ok();
        is_healthy && health_set
    }

    pub fn set_health(&self, healthy: bool) -> Result<()> {
        let mut health = self.is_healthy.lock().unwrap();
        *health = healthy;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub port: String,
    pub health_check_interval: String,
    pub servers: Vec<String>,
}
