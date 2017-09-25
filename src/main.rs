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
                    .about("A Dynamic DOmain nameServer client which is configurable in lua and provides a RESTful API for remote host configuration")
                    .arg(Arg::with_name("port")
                            .short("p")
                            .long("port")
                            .help("Sets a custom port for REST interface")
                            .takes_value(true))
                    .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .help("Provide a custom .lua config file (see README)")
                            .takes_value(true))
                    .arg(Arg::with_name("store")
                            .short("s")
                            .long("store")
                            .help("Custom storage location for known hosts (.json file)")
                            .takes_value(true))
                    .arg(Arg::with_name("keystore")
                            .short("ks")
                            .long("keys")
                            .help("Provide the directory where authorised public keys are stored")
                            .takes_value(true))
                    .subcommand(SubCommand::with_name("run").about("Run ddos as a forground process (mostly for development)"))
                    .subcommand(SubCommand::with_name("start").about("Start ddos as a background process"))
                    .subcommand(SubCommand::with_name("stop").about("Stop ddos background process (if exists)"))
                    .subcommand(SubCommand::with_name("status").about("Get the current ddos process status"))
                    .subcommand(SubCommand::with_name("restart").about("First stop, then restart ddos"))
                    .subcommand(SubCommand::with_name("reload").about("Reload the lua file in case of changes"))
                    .get_matches();

    let port = matches.value_of("port").unwrap_or("8888");
    println!("Port: {}", port);
}