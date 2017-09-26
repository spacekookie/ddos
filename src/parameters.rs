// External dependencies

use clap::{Arg, App, SubCommand, ArgMatches};


pub fn initialise<'a>(name: String, version: &'a str) -> App<'a, 'a> {
    let mut app = App::new(name)
        .version(version)
        .author("Katharina Fey <kookie@spacekookie.de>")
        .about("A Dynamic DOmain nameServer client which is configurable in lua \n  and provides a RESTful API for remote host configuration")

    /* Define argument handlers */
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Override the API port setting from the config")
            .takes_value(true))
        .arg(Arg::with_name("config") // TODO: Rename this to "lua"?
            .short("c")
            .long("config")
            .help("Provide a DDOS config file (default: ddos.toml)")
            .takes_value(true))
        // .arg(Arg::with_name("hoststore")
        // .short("s")
        // .long("hosts")
        // .help("Override host-store setting from the config")
        // .takes_value(true))
        // .arg(Arg::with_name("keystore")
        // .short("k")
        // .long("keys")
        // .help("Override key-store setting from the config")
        // .takes_value(true))
                    
    /* Mostly used for development */
        .subcommand(SubCommand::with_name("run").about("Run ddos as a forground process"))
    
    /* Basic daemon control flow */
        .subcommand(SubCommand::with_name("start").about("Start ddos as a background process"))
        .subcommand(SubCommand::with_name("stop").about("Stop ddos background process (if exists)"))
        .subcommand(SubCommand::with_name("status").about("Get the current ddos process status"))
        .subcommand(SubCommand::with_name("restart").about("First stop, then restart ddos"))

    /* Utility to register/unregister pubkeys */
        .subcommand(SubCommand::with_name("register").about("Register a public key as authorised"))
        .subcommand(SubCommand::with_name("unregister").about("Unregister an authorised key"));

    return app;
}