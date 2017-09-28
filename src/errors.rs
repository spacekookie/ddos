/// A simple module to help print errors for the user
//
// It provides an easy API to call for error prints which
// will then inform the user (sometimes in colour!) about
// what went wrong! Nice, eh?

use std::process;
use colored::*;


pub struct Errors<'a> {
    pub name: &'a str,
    pub version: &'a str
}

pub enum ErrorType {
    Status,
    Warning,
    Problem
}

impl<'a> Errors<'a> {
    
    pub fn status(&self, msg: &str) {
        self.log(msg, ErrorType::Status);
    }

    pub fn warn(&self, msg: &str) {
        self.log(msg, ErrorType::Warning);
    }

    /// Easily logs a message with a type used as a formatter
    ///
    /// This can and should be used for events that the user should potentially be
    /// informed about, not just for the sake of it. The type can be used to
    /// indicate the context of the logged message.
    ///
    pub fn log(&self, msg: &str, _type: ErrorType) {
        self._log(msg, _type, 0);
    }

    /// Log a message and then exit the application
    pub fn log_and_die(&self, msg: &str, code: i32) -> ! {
        self._log(msg, ErrorType::Problem, code);
        return process::exit(code);
    }

    /// Internal utility function which supports colour printing
    fn _log(&self, msg: &str, _type: ErrorType, code: i32) {
        let cstr = match _type {
            ErrorType::Status => format!("({})", code).green().to_string(),
            ErrorType::Warning => format!("({})", code).yellow().to_string(),
            ErrorType::Problem => format!("({})", code).red().to_string(),
            _ => format!("({})", code).to_string(),
        };

        println!("{} {} {}", cstr, "=>".cyan(), msg);
    }
}