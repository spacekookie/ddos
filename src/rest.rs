//! Module that provides the RESTful API for remote host configuration

use rocket;
use rocket::State;
use rocket_contrib::Json;
use state::DDOS;
use dns::DNState;

use security::{secret_compare, Signature};

#[derive(Debug, Serialize, Deserialize)]
struct Host {
    auth: Signature,
    name: String,
    ip: String,
}


/// Gets some base data for a registered host. Is considered public 
///   because DNS can return the same information as this API so
///   there isn't really any point to faking privacy
#[get("/host/<host>")]
#[allow(unused_variables)]
fn query(host: String, state: State<DDOS>) -> String {

    println!("Getting queury");

    /* "Find" a host */
    let hosts = state.hosts.lock().unwrap();
    return match hosts.get(&host) {
        Some(val) => format!("{}", val),
        None => format!("Unknown host!"),
    };
}


#[allow(unused_variables)]
#[post("/host/<host>", format = "application/json", data = "<host_data>")]
fn host_update(host: String, host_data: Json<Host>, state: State<DDOS>, dns: State<DNState>) -> String{

    /* First check the key ID is even known */
    let sig = &host_data.auth;

    let keyguard = state.keys.lock();
    let keys = keyguard.unwrap();

    // let keys = state.keys.lock().unwrap();
    if !keys.contains_key(&host_data.auth.key_id) {
        return format!("UNKNOWN KEY ID")
    }

    /* Then compare the actual key secrets */
    if !secret_compare(&host_data.auth.signature, &keys.get(&sig.key_id).unwrap()) {
        return format!("SIGNATURE WRONG")
    }

    let mut m = state.hosts.lock().unwrap();
    m.insert(host_data.name.clone(), host_data.ip.clone());

    let mut dns_hosts = dns.hosts.lock().unwrap();
    dns_hosts.insert(host_data.name.clone(), host_data.ip.clone());

    // Dropping scopes so we can lock it again later
    drop(dns_hosts);
    drop(keys);
    drop(m);

    // Sync the changes immediately
    state.sync();
    return format!("ACK");
}


pub fn initialise(state: DDOS, dns: DNState) {
    rocket::ignite()
        .mount("/", routes![query, host_update])
        .manage(state)
        .manage(dns)
        .launch();
}
