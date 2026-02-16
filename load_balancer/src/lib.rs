use std::sync::{Arc, Mutex};

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
        *self.is_healthy.lock().unwrap()
    }

    pub fn set_health(&self, healthy: bool) {
        let mut health = self.is_healthy.lock().unwrap();
        *health = healthy;
    }
}
