/// Small dynamic DNS utily with RESTful API for configuration
///
/// 

extern crate clap;

// stdlib dependencies
use std::fs;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

// Internal modules
mod parameters;


/// Holds the core state for DDOS
struct DDOS {
    lua: String,
    hosts: String,
    keys: Vec<String>,
    api_port: u32,
}

/// Main application entry point
fn main() {
    let name = env::args().next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from).unwrap();

    let version = "0.1.0";

    let app = parameters::initialise(name, version);
    let matches = app.get_matches();

    /* Get argument values or set some sane defaults */
    let port = matches.value_of("port").unwrap_or("8001");
    let config: String = matches.value_of("config").unwrap_or("ddos.lua").to_string();
    let hoststore: String = matches.value_of("hoststore").unwrap_or("hosts.json").to_string();
    let keystore: String = matches.value_of("keystore").unwrap_or("keys/").to_string();

    // println!("Port: {}", port);
    // println!("Config: {}", config);
    // println!("Hoststore: {}", hoststore);
    // println!("Keystore: {}", keystore);

    // match read_keys_from_directory(keystore) {
    //     Some(_) => {},
    //      None => {
    //         println!("Failed to open key store!");

    //      },
    //  }
}


fn read_keys_from_directory(path: String) -> Option<Vec<String>> {
    let mut keys: Vec<String> = Vec::new();

    let paths = match fs::read_dir(path) {
        Ok(file) => file,
        _ => return None
    };

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    return Option::from(keys);
}