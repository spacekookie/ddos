//! Wrapper module around luadns
//!
//! This is pretty suboptimal and should be changed. Maybe for a pure Rust
//!   DNS implementation.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::thread::{spawn, JoinHandle};

use std::collections::HashMap;
use std::sync::Mutex;

/// The main state held (and given to C) for DNS resolution
#[derive(Debug)]
#[repr(C)]
pub struct DNState {
    pub hosts: Mutex<HashMap<String, String>>,
    // thread: JoinHandle<()>
}

/// A struct that represents an IP address between Rust and C.
/// Has always 16 places for compatibility with IPv6
#[repr(C)]
pub struct IPAddress {
    addr: [c_int; 16],
}

extern "C" {
    fn start_dns_server(port: i32);
    fn set_state(state: *const DNState);
    fn set_callback(_type: i32, cb: extern "C" fn(*const c_void, *const c_char) -> IPAddress);
}

impl DNState {
    // state: &'a Mutex<HashMap<String, String>>
    pub fn new(port: i32) -> DNState {
        let dns = DNState {
            hosts: Mutex::new(HashMap::new()),
            // thread: spawn(move || { }),
        };

        unsafe {
            set_state(&dns as *const _);

            spawn(move || {
                set_callback(4, ipv4_callback);
                set_callback(6, ipv6_callback);
                start_dns_server(port);
            });
        }

        return dns;
    }
}

/// A simple function which resolves IPv4 queries in our known hosts hashtable
extern "C" fn ipv4_callback(state: *const c_void, string: *const c_char) -> IPAddress {
    let host_name = unsafe { CStr::from_ptr(string).to_str().unwrap() };
    let state_data: &DNState = unsafe { &*(state as *const DNState) };
    let host_data = state_data.hosts.lock().unwrap();

    return match host_data.get(host_name) {
        Some(val) => {
            let split = val.split(".");

            let mut ctr = 0;
            let mut addr: [i32; 16] = [0; 16];
            for s in split {
                let val = s.parse::<i32>().unwrap();
                addr[ctr] = val;
                ctr += 1;
            }

            return IPAddress { addr };
        }
        _ => IPAddress {
            addr: [127, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
}

/// A simple function which resolves IPv4 queries in our known hosts hashtable
extern "C" fn ipv6_callback(state: *const c_void, string: *const c_char) -> IPAddress {
    let host_name = unsafe { CStr::from_ptr(string).to_str().unwrap() };
    let state_data: &DNState = unsafe { &*(state as *const DNState) };
    let host_data = state_data.hosts.lock().unwrap();

    return match host_data.get(host_name) {
        Some(val) => {
            let split = val.split(":");

            let mut ctr = 0;
            let mut addr: [i32; 16] = [0; 16];
            for s in split {
                let val = s.parse::<i32>().unwrap();
                addr[ctr] = val;
                ctr += 1;
            }

            return IPAddress { addr };
        }
        _ => IPAddress {
            addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        },
    };
}
