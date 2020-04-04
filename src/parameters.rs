//! External dependencies

use clap::{App, Arg, SubCommand}; // ArgMatches

pub fn initialise<'a>(name: &'a str, version: &'a str) -> App<'a, 'a> {
    App::new(name)
        .version(version)
        .author("Katharina Fey <kookie@spacekookie.de>")
        .about("A Dynamic DOmain nameServer client which provides a RESTful API for remote host configuration")
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .help("Override the API port setting from the config")
             .takes_value(true))
        .arg(Arg::with_name("dns-port")
             .short("d")
             .long("dns-port")
             .help("Override the DNS port setting from the config")
             .takes_value(true))
        .arg(Arg::with_name("config") // TODO: Rename this to "lua"?
             .short("c")
             .long("config")
             .help("Provide a DDOS config file (default: ddos.toml)")
             .takes_value(true))
        .subcommand(SubCommand::with_name("run").about("Run ddos as a forground process"))
        .subcommand(SubCommand::with_name("register")
                    .about("Register a public key as authorised")
                    .arg(Arg::with_name("key_id")
                         .takes_value(true)
                         .long("id")
                         .help("Provide a key ID"))
                    .arg(Arg::with_name("secret")
                         .takes_value(true)
                         .long("secret")
                         .help("A cryptographic random secret")))
}
