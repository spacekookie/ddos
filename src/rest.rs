//! Module that provides the RESTful API for remote host configuration

use rocket;
use rocket::State;
use rocket_contrib::Json;
use core::DDOS;


#[derive(Debug, FromForm)]
struct Signature {
    signature: String,
    key_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Host {
    name: String,
    ip: String,
}


/// Gets some base data for a registered host. Is considered public 
///   because DNS can return the same information as this API so
///   there isn't really any point to faking privacy
#[get("/host/<host>?<payload>")]
#[allow(unused_variables)]
fn query(host: String, payload: Option<Signature>, state: State<DDOS>) -> String {

    /* "Find" a host */
    let hosts = state.hosts.lock().unwrap();
    let hq = hosts.get(&host);

    match (hq, &payload) {
        (Some(h), &Some(ref sig)) => {
            return format!("Host: {}\nSignature: {:?}", h, sig.signature);
        },
        (_, _) => return format!("INVALID PARAMETERS {}, {:?}!", host, &payload)
    }
}


#[allow(unused_variables)]
#[post("/host/<host>", format = "application/json", data = "<host_data>")]
fn host_update(host: String, host_data: Json<Host>, state: State<DDOS>) {
    // TODO: Check authentication

    let mut m = state.hosts.lock().unwrap();
    m.insert(host_data.name.clone(), host_data.ip.clone());
}


pub fn initialise(state: DDOS) {
    rocket::ignite()
        .mount("/", routes![query, host_update])
        .manage(state)
        .launch();
}
