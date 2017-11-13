//! The core module of DDOS which handles initialisation and persistent state

// stdlib dependencies
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use std::fs::*;
use std::io::prelude::*;
use std::collections::*;
use serde_json;

use LOG;


/// Represents the json config which should become it's own module
#[derive(Serialize, Deserialize)]
struct HostFile {
    hosts: Vec<Host>,
}

/// Represents a host in the json config
#[derive(Serialize, Deserialize)]
struct Host {
    name: String,
    ip: String,
}

/// The core ddos state
pub struct DDOS {
    pub hosts: Mutex<HashMap<String, String>>,
    pub keys: Mutex<HashMap<String, String>>,
    pub lua: String,
    pub api_port: u32,
    host_path: String,
    key_path: String,
}


impl DDOS {
    #[allow(unused_variables)]
    pub fn new(lua_path: &str, host_path: &str, key_path: &str, port: u32) -> DDOS {

        /* Read state from disk */
        let keys = DDOS::get_authorized(key_path);
        let hosts = DDOS::get_hosts(host_path);

        return DDOS {
            lua: String::from(""),
            hosts: Mutex::new(hosts),
            api_port: port,
            keys: Mutex::new(keys),
            host_path: String::from(host_path),
            key_path: String::from(key_path),
        };
    }

    /// A function that syncs the current state to disk
    pub fn sync(&self) {

        /* Lock the hosts store */
        let h = self.hosts.lock().unwrap();

        /* Move the hosts into a convenient list we can serialise */
        let mut hosts: Vec<Host> = Vec::new();
        for (name, ip) in h.iter() {
            hosts.push(Host {
                name: name.clone(),
                ip: ip.clone(),
            });
        }

        let json = serde_json::to_string(&HostFile { hosts: hosts });
        let file = File::open(&self.host_path);
        // write!(file, "{:?}", json);
    }

    fn get_authorized(path: &str) -> HashMap<String, String> {
        let mut auth: HashMap<String, String> = HashMap::new();

        let items = match fs::read_dir(path) {
            Ok(file) => {
                LOG.status("Opened secrets directory");
                file // Match this
            }
            _ => {
                LOG.warn("Failed to open secrets directory!");
                return HashMap::new();
            }
        };

        /* Loop over "secrets" - keys are file names */
        for item in items {
            let path = match item {
                Ok(i) => i.path(),
                _ => continue,
            };

            let name = match path.as_path().file_name() {
                Some(i) => String::from(i.to_str().unwrap()),
                _ => continue,
            };
            let mut secret_f = File::open(&path).unwrap();
            let mut secret = String::new();
            secret_f.read_to_string(&mut secret).unwrap();
            drop(secret_f);

            LOG.status(&secret);

            /* Store name-secret combo in map */
            LOG.status(&format!("Scoped a secret for '{}'", &name));
            auth.insert(name, secret);
        }

        return auth;
    }

    fn get_hosts(path: &str) -> HashMap<String, String> {
        let mut hosts = HashMap::new();

        // TODO: Read data from some form of config!
        let k = String::from("kookiejar.tech");
        let v = String::from("67.61.79.0"); // 🌈
        hosts.insert(k, v);

        return hosts;
    }
}
