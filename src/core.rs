//! The core module of DDOS which handles initialisation and persistent state

// stdlib dependencies
use std::fs;
use std::sync::Mutex;
use std::fs::*;
use std::ffi::*;
use std::io::prelude::*;
use std::collections::*;

use serde_json;

use LOG;


#[derive(Serialize, Deserialize)]
struct HostFile {
    hosts: Vec<Host>,
}


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
        };
    }

    /// A function that syncs the current state to disk
    pub fn sync(&self) {
        println!("Starting sync...");

        let k = self.keys.lock().unwrap();

        /* Move the hosts into a convenient list we can serialise */
        let mut hosts: Vec<Host> = Vec::new();
        for (name, ip) in k.iter() {

            println!("{} ->{}", name, ip);
            let h = Host {
                name: name.clone(),
                ip: ip.clone(),
            };

            hosts.push(h);
        }
 
        println!("After sync...", )

        // let json = serde_json::to_string(&HostFile { hosts: hosts });
        // println!("{}", json.unwrap());
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

            /* If it's not a public key */
            let extension = path.extension();
            if extension != Some(OsStr::new("pub")){
                continue;
            }

            let ppp = path.clone().into_os_string().into_string();
            LOG.status(&format!("Loading authorised pub key: {:?}", ppp.unwrap()));

            let mut key_file = File::open(&path).unwrap();
            let mut key_str = String::new();
            key_file.read_to_string(&mut key_str).unwrap();
            drop(key_file);

            // LOG.status(&format!("Adding keyfile {}", key_str));
            keys.push(key_str);
        }

        return keys;
    }

    fn get_hosts(path: &str) -> HashMap<String, String> {
        let mut hosts: HashMap<String, String> = HashMap::new();

        // TODO: Read data from some form of config!
        let k = String::from("kookiejar.tech");
        let v = String::from("67.61.79.0"); // 🌈
        hosts.insert(k, v);

        return hosts;
    }
}
