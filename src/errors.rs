/// A simple module to help print errors for the user
//
// It provides an easy API to call for error prints which
// will then inform the user (sometimes in colour!) about
// what went wrong! Nice, eh?

use std::process;


pub struct Errors<'a> {
    pub name: &'a str,
    pub version: &'a str
}

pub enum ErrorType {

}

impl<'a> Errors<'a> {
    
    /// Easily logs a message with a type used as a formatter
    ///
    /// This can and should be used for events that the user should potentially be
    /// informed about, not just for the sake of it. The type can be used to
    /// indicate the context of the logged message.
    ///
    pub fn log(&self, msg: &str, _type: &ErrorType) {
        println!("{}", msg);
    }

    /// Log a message and then exit the application
    ///
    // TODO: Make this a macro?
    pub fn log_and_die(&self, msg: &str, code: i32) -> ! {
        println!("({}) => {}", code, msg);
        process::exit(code);
    }
}