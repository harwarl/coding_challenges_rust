use std::{fs, io::Result};

use lb::Config;

fn main() {
    // Load the config
    let config = match load_config() {
        Ok(v) => v,
        Err(e) => {
            panic!("Error loading config, {e}")
        }
    };

    
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
