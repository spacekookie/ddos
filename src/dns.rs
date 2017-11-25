//! Wrapper module around luadns
//!
//! This is pretty suboptimal and should be changed. Maybe for a pure Rust
//!   DNS implementation.


use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::thread;

use std::sync::Mutex;
use std::collections::HashMap;


extern "C" {
    fn ddos_dns_start(port: i32);
    fn ddos_register_state(state: &DNS);
    fn ddos_register_callback(t: i32, cb: extern "C" fn(&DNS, *const c_char) -> [u32; 16]);
}


pub struct DNS<'a> {
    state: &'a Mutex<HashMap<String, String>>,
    thread: Option<thread::JoinHandle<()>>,
}


impl<'a> DNS<'a> {
    pub fn new(state: &'a Mutex<HashMap<String, String>>) -> DNS<'a> {
        return DNS {
            state: state,
            thread: None,
        };
    }

    pub fn start(&mut self, port: i32) {
        unsafe {
            ddos_register_state(self);
            ddos_register_callback(4, cb_a_record);
            ddos_register_callback(6, cb_aaaa_record);
        }

        let child = thread::spawn(move || {
            unsafe {
                ddos_dns_start(port);
            }
        });

        child.join();
    }
}

/// A helper function which 
fn ipv4_to_ipv6(v4: [u32; 4]) -> [u32; 16] {
    let mut v6: [u32; 16] = [0; 16];
    v6[..4].clone_from_slice(&v4);
    println!("{:?}", v6);
    return v6;
}

/// A callback function which fetches the A-record section from a host
extern "C" fn cb_a_record(state: &DNS, host: *const c_char) -> [u32; 16] {
    unsafe {
        let slice = CStr::from_ptr(host);
        println!("string returned: {}", slice.to_str().unwrap());

        // let c_str: &CStr = CStr::from_ptr(host);
        // println!("{:?}", c_str);

        // let str_slice: &str = c_str.to_str().unwrap();
        // println!("{:?}", str_slice);
    }

    let var = ipv4_to_ipv6([123, 123, 123, 123]);
    return var;
}

/// A callback function which fetches the AAAA-record (ipv6) data from a host
extern "C" fn cb_aaaa_record(state: &DNS, host: *const c_char) -> [u32; 16] {
    let var: [u32; 16] = [123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123];
    return var;
}
