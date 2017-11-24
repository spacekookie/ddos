//! Wrapper module around luadns
//!
//! This is pretty suboptimal and should be changed. Maybe for a pure Rust
//!   DNS implementation.


use std::ffi::CString;
use std::os::raw::c_char;
use std::thread;

use std::sync::Mutex;
use std::collections::HashMap;


extern "C" {
    fn ddos_dns_start(port: i32);
    fn ddos_register_state(state: &DNS);
    fn ddos_register_callback(cb: extern "C" fn(&DNS) -> *const c_char);
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
            ddos_register_callback(cb_a_record);
            ddos_register_callback(cb_aaaa_record);
        }

        self.thread = Some(thread::spawn(move || {
            unsafe {
                ddos_dns_start(port);
            }
        }));
    }
}


/// A callback function which fetches the A-record section from a host
extern "C" fn cb_a_record(state: &DNS) -> *const c_char {
    return CString::new("hurray").unwrap().as_ptr();
}

/// A callback function which fetches the AAAA-record (ipv6) data from a host
extern "C" fn cb_aaaa_record(state: &DNS) -> *const c_char {
    return CString::new("hurray").unwrap().as_ptr();
}