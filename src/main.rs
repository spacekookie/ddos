/// Small dynamic DNS utily with RESTful API for configuration
///
/// 


// External dependencies
extern crate clap;
use clap::{Arg, App, SubCommand};

// stdlib dependencies
use std::env;
use std::path::Path;
use std::ffi::OsStr;


/// Main application entry point
fn main() {
    let name = env::args().next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from).unwrap();

    let matches = App::new(name).version("0.1")
                    .author("Katharina Fey <kookie@spacekookie.de>")
                    .about("Small dynamic DNS server & utility")
                    .arg(Arg::with_name("port")
                            .short("p")
                            .long("port")
                            .help("Sets a custom port for REST interface")
                            .takes_value(true))
                    .get_matches();

    let port = matches.value_of("port").unwrap_or("8888");
    println!("Port: {}", port);
}