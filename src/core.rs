/// The core module of DDOS which handles initialisation and persistent state

// stdlib dependencies
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use errors::Errors;
use LOG;


/// The core ddos state
pub struct DDOS {
    lua: String,
    hosts: String,
    keys: Vec<String>,
    api_port: u32,
}


impl DDOS {
    
    pub fn new(lua_path: &str, host_path: &str, key_path: &str, port: u32) -> DDOS {
        
        /* Read state from disk */
        let keys = DDOS::get_keys(key_path);
        let hosts = DDOS::get_hosts(host_path);

        return DDOS {
            lua: String::from(""), 
            hosts: String::from(""), 
            api_port: port,
            keys: keys, 
        }
    }


    /// Read keys from a directory that we already know exists
    fn get_keys(path: &str) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();

        /*  */
        let items = match fs::read_dir(path) {
            Ok(file) => file,
            _ => return Vec::new()
        };

        LOG.status("Opened key directory");

        for item in items {
            let path = match item {
                Ok(i) => i.path(),
                _ => continue,
            };

            let ppp = path.clone();
            LOG.status(&format!("Loading authorised key: '{:?}'", ppp.into_os_string().into_string()));

            /* If it's not a public key */
            if !path.ends_with(".pub") {
                continue;
            }

            let mut key_file = File::open(&path).unwrap();
            let mut key_str = String::new();
            key_file.read_to_string(&mut key_str).unwrap();
            drop(key_file);


            // LOG.status(&format!("Adding keyfile {}", &path));
            keys.push(key_str);
        }

        return keys;
    }

    fn get_hosts(path: &str) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();

        return keys;
    }


}

// pub fn fooooooo() {
//     ERRORS.log_and_die("", 255);
// }