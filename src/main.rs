/// Small dynamic DNS utily with RESTful API for configuration
///
/// 

extern crate clap;
extern crate toml;
#[macro_use]
extern crate serde_derive;

// stdlib dependencies
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

// Internal modules
mod parameters;
mod errors;
use errors::Errors;

/// 
#[derive(Debug, Deserialize)]
struct Config {
    keys: Option<String>,
    hooks: Option<String>,
    port: Option<String>,
    hosts: Option<String>
}

/// Holds the core state for DDOS (Mirrors ddos.toml)
struct DDOS {
    lua: String,
    hosts: String,
    keys: Vec<String>,
    api_port: u32,
}

static APP_VERSION: &'static str = "0.1.0";
static APP_NAME: &'static str = "ddos";
static DEF_PORT: &'static str = "8001";
static DEF_CONFIG: &'static str = "ddos.toml";
static DEF_HOOKS: &'static str = "ddos.lua";
static DEF_HOSTS: &'static str = "hosts.json";
static DEF_KEYS: &'static str = "keys/";


/// Main application entry point
fn main() {

    /* First initialise our errors endpoint */
    let err = Errors { name: APP_NAME, version: APP_VERSION };

    /* Intialise the parameter system and get all provided */
    let app = parameters::initialise(APP_NAME, APP_VERSION);
    let matches = app.get_matches();

    /* Bind any overriding config path that was provided */
    let cfg: String = matches.value_of("config").unwrap_or(DEF_CONFIG).to_string();

    /* See if the provided config is loadable (exists && valid) */
    let cfg_path = Path::new(&cfg);
    if !cfg_path.exists() {
        err.log_and_die("Provided configuration wasn't found!", 255);
    }

    /* Open the config file */
    let mut cfg_file = File::open(&cfg_path).unwrap_or_else(|_| {
        err.log_and_die("Provided configuration wasn't found!", 255)
    });

    /* Read it like a book */
    let mut cfg_string = String::new();
    cfg_file.read_to_string(&mut cfg_string).unwrap_or_else(|_| {
        err.log_and_die("Failed to read config file contents!", 255)
    });

    let cfg_toml: Config = toml::from_str(&cfg_string).unwrap_or_else(|_| {
        err.log_and_die("Failed to parse config. Is the TOML syntax correct?", 255);
    });

    /* Bind config values or assume defaults if missing */
    let keys_path: &str = &cfg_toml.keys.unwrap_or(String::from(DEF_KEYS));
    let hosts_path: &str = &cfg_toml.hosts.unwrap_or(String::from(DEF_HOSTS));
    let hooks_path: &str = &cfg_toml.hooks.unwrap_or(String::from(DEF_HOOKS));
    let port_config: &str = &cfg_toml.port.unwrap_or(String::from(DEF_PORT));

    /* Bind either CLI override or what was previous determined as port from CONFIG */
    let port = matches.value_of("port").unwrap_or(port_config);

    /* We no longer need the config file */
    drop(cfg_file);

    println!("{}", keys_path);
    println!("{}", hosts_path);
    println!("{}", hooks_path);
    println!("{}", port);

    /* Check if the provided directories exist */
    if !Path::new(&keys_path).exists() {
        err.log_and_die("Provided keys directory didn't exist!", 255);
    }

    let keys: Vec<String> = read_keys_from_directory(keys_path);
}


/// Read keys from a directory that we already know exists
fn read_keys_from_directory(path: &str) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();

    /*  */
    let paths = match fs::read_dir(path) {
        Ok(file) => file,
        _ => return Vec::new()
    };

    for path in paths {
        
    }

    return keys;
}

// fn read_keys_from_directory(path: String) -> Option<Vec<String>> {
//     let mut keys: Vec<String> = Vec::new();

//     let paths = match fs::read_dir(path) {
//         Ok(file) => file,
//         _ => return None
//     };

//     for path in paths {
//         println!("Name: {}", path.unwrap().path().display())
//     }

//     return Option::from(keys);
// }