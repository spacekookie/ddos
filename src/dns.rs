//! Wrapper module around luadns
//!
//! This is pretty suboptimal and should be changed. Maybe for a pure Rust
//!   DNS implementation.


use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_void, c_char};
use std::thread;

use std::collections::HashMap;
use std::sync::Mutex;

/// The main state held (and given to C) for DNS resolution
pub struct DNState<'a> {
    val: &'a Mutex<HashMap<String, String>>
}

/// A struct that represents an IP address between Rust and C.
/// Has always 16 places for compatibility with IPv6
#[repr(C)]
pub struct IPAddress {
    addr: [c_int; 16]
}

extern {
    fn start_dns_server(port: i32);
    fn set_state(state: &DNState);
    fn set_callback(_type: i32, cb: extern "C" fn(*const c_void, *const c_char) -> IPAddress);
    fn do_fun_stuff();
}


/// A simple function which resolves IPv4 queries in our known hosts hashtable
extern "C" fn ipv4_callback(state: *const c_void, string: *const c_char) -> IPAddress {
    let host_name = unsafe { CStr::from_ptr(string).to_str().unwrap() };
    let state_data: &DNState = unsafe { &*(state as *const DNState) };
    let host_data = state_data.val.lock().unwrap();
    drop(state_data.val);

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

            return IPAddress { addr: addr };
        },
        _ => IPAddress { addr: [127, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
    };
}

impl<'a> DNState<'a> {
    pub fn new(state: &'a Mutex<HashMap<String, String>>) -> DNState {
        return DNState {
            val: state
        };
    }

    pub fn start(&self, port: i32) {
        unsafe {
            set_state(self);
            set_callback(4, ipv4_callback);
            // do_fun_stuff();
            start_dns_server(port);
        }

        // self.val.lock().unwrap().get("kookiejar.tech2").unwrap();
        
        // let child = thread::spawn(move || {
        //     unsafe {
        //         start_dns_server(port);
        //     }
        // });
        // child.join();
    }
}


// extern "C" {
//     fn ddos_dns_start(port: i32);
//     fn ddos_register_state(state: &DNS);
//     fn ddos_register_callback(t: i32, cb: extern "C" fn(*const c_void, *const c_char) -> [u32; 16]);
//     fn do_fun_stuff();
// }


// pub struct DNS<'a> {
//     state: &'a Mutex<HashMap<String, String>>,
//     thread: Option<thread::JoinHandle<()>>,
// }


// impl<'a> DNS<'a> {
//     pub fn new(state: &'a Mutex<HashMap<String, String>>) -> DNS<'a> {
//         return DNS {
//             state: state,
//             thread: None,
//         };
//     }

//     pub fn start(&mut self, port: i32) {
//         unsafe {
//             ddos_register_state(self);
//             ddos_register_callback(4, cb_a_record);
//             ddos_register_callback(6, cb_aaaa_record);
//         }

//         let child = thread::spawn(move || {
//             unsafe {
//                 ddos_dns_start(port);
//             }
//         });

//         child.join();
//     }
// }

// /// A helper function which 
// fn ipv4_to_ipv6(v4: [u32; 4]) -> [u32; 16] {
//     let mut v6: [u32; 16] = [0; 16];
//     v6[..4].clone_from_slice(&v4);
//     println!("{:?}", v6);
//     return v6;
// }

// /// A callback function which fetches the A-record section from a host
// extern "C" fn cb_a_record(state: *const c_void, string: *const c_char) -> [u32; 16] {
//     println!("CALLBACK!");
    
//     // let state_data: &Mutex<HashMap<String, String>> = unsafe { &*(state as *const Mutex<HashMap<String, String>>) };

//     unsafe {
//         println!("{:?}", state);
//     }

//     // let host_name = unsafe { CStr::from_ptr(host) };

//     /* Find out if we know this host  */
//     // let mut known_hosts: &HashMap<String, String> = &*state_data.lock().unwrap();

//     let var = ipv4_to_ipv6([123, 123, 123, 123]);
//     return var;
// }

// /// A callback function which fetches the AAAA-record (ipv6) data from a host
// extern "C" fn cb_aaaa_record(state: *const c_void, string: *const c_char) -> [u32; 16] {
//     let h = unsafe { CStr::from_ptr(string) };

//     let var: [u32; 16] = [123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123];
//     return var;
// }
