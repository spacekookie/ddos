//! The core module of DDOS which handles initialisation and persistent state

// stdlib dependencies
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::sync::Mutex;

use serde_json;
use std::collections::*;

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
    pub api_port: u32,
    host_path: String,
}

impl DDOS {
    #[allow(unused_variables)]
    pub fn new(host_path: &str, key_path: &str, port: u32) -> DDOS {
        /* Read state from disk */
        let keys = DDOS::get_authorized(key_path);
        let hosts = DDOS::get_hosts(host_path);

        DDOS {
            hosts: Mutex::new(hosts),
            api_port: port,
            keys: Mutex::new(keys),
            host_path: String::from(host_path),
        }
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

        let json = serde_json::to_string(&HostFile { hosts: hosts }).unwrap();
        let mut file = File::create(&self.host_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
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

            /* Store name-secret combo in map */
            LOG.status(&format!("Scoped a secret for '{}'...", &name));
            auth.insert(name, secret);
        }

        auth
    }

    fn get_hosts(path: &str) -> HashMap<String, String> {
        let mut hosts = HashMap::new();

        let mut hosts_json = String::new();

        match File::open(path) {
            /* Executed if the file exists and we can work with it */
            Ok(ref mut f) => {
                f.read_to_string(&mut hosts_json).unwrap();
                let hostfile: HostFile = serde_json::from_str(&hosts_json).unwrap();

                for host in &hostfile.hosts {
                    let name = &host.name;
                    let ip = &host.ip;

                    hosts.insert(name.clone(), ip.clone());
                }
            }

            /* Executed if the config doesn't exist */
            Err(_) => {
                let mut file = File::create(path).unwrap();
                file.write_all(b"{ \"hosts\": [] }").unwrap();
            }
        };

        hosts
    }
}
