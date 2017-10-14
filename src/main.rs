//! Small dynamic DNS utily with RESTful API for configuration
//!
//!

// WTF: Why does this have to be on top?
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate clap;
extern crate toml;
extern crate colored;

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
use errors::{Errors, ErrorType};

mod core;
use core::DDOS;

mod rest;

/// Struct that represents a TOML configuration on disk
#[derive(Debug, Deserialize)]
struct TomlConfig {
    keys: Option<String>,
    hooks: Option<String>,
    port: Option<String>,
    hosts: Option<String>
}

/* Define some constants */
const APP_VERSION: &'static str = "0.1.0";
const APP_NAME: &'static str = "ddos";
const DEF_PORT: &'static str = "8001";
const DEF_CONFIG: &'static str = "ddos.toml";
const DEF_HOOKS: &'static str = "ddos.lua";
const DEF_HOSTS: &'static str = "hosts.json";
const DEF_KEYS: &'static str = "keys/";

static LOG: Errors = Errors { name: APP_NAME, version: APP_VERSION };


/// Safely load a config file from disk. Needs to be TOML encoded
fn load_config(path: String) -> TomlConfig {

    /* See if the provided config is loadable (exists && valid) */
    let cfg_path = Path::new(&path);
    if !cfg_path.exists() {
        LOG.log_and_die("Provided configuration wasn't found!", 255);
    }

    /* Open the config file */
    let mut cfg_file = File::open(&cfg_path).unwrap_or_else(|_| {
        LOG.log_and_die("Provided configuration wasn't found!", 255)
    });

    /* Read it like a book */
    let mut cfg_string = String::new();
    cfg_file.read_to_string(&mut cfg_string).unwrap_or_else(|_| {
        LOG.log_and_die("Failed to read config file contents!", 255)
    });

    let cfg_toml: TomlConfig = toml::from_str(&cfg_string).unwrap_or_else(|_| {
        LOG.log_and_die("Failed to parse config. Is the TOML syntax correct?", 255);
    });

    drop(cfg_file);
    return cfg_toml;
}


/// Main application entry point
fn main() {

    /* Initialise parameter parser & get all matches */
    let matches = parameters::initialise(APP_NAME, APP_VERSION).get_matches();

    /* Find out what config to load and load it */
    let cfg: String = matches.value_of("config").unwrap_or(DEF_CONFIG).to_string();
    let cfg_toml: TomlConfig = load_config(cfg);

    /* Bind required values from config or defaults */
    let keys_path: &str = &cfg_toml.keys.unwrap_or(String::from(DEF_KEYS));
    let hosts_path: &str = &cfg_toml.hosts.unwrap_or(String::from(DEF_HOSTS));
    let lua_path: &str = &cfg_toml.hooks.unwrap_or(String::from(DEF_HOOKS));
    let port_config: &str = &cfg_toml.port.unwrap_or(String::from(DEF_PORT));

    /* Override port with CLI value if it was provided */
    let port: u32 = matches.value_of("port").unwrap_or(port_config).parse::<u32>().unwrap_or_else(|_| {
        LOG.log_and_die("Invalid port; must be unsigned integer!", 255);
    });

    LOG.log("Loaded config successfully!", ErrorType::Status);

    /* Initialise the main state and try (🤞) to load required files */
    let state = DDOS::new(lua_path, hosts_path, keys_path, port);
    
    /* Initialise the REST API (🚀) with referenced state */
    rest::initialise(state);
    
    // 9:45
}


