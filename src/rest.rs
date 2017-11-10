//! Module that provides the RESTful API for remote host configuration

use rocket;
use rocket::State;
use rocket_contrib::Json;
use core::DDOS;

use security::secret_compare;

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct Signature {
    signature: String,
    key_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Host {
    auth: Signature,
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

    /* First check the key ID is even known */
    let sig = &host_data.auth;
    let keys = state.keys.lock().unwrap();
    if !keys.contains_key(&host_data.auth.key_id) {
        return format!("Provided signature key ID not known to the system. Bugger off...")
    }

    /* Then compare the actual key secrets */
    if !secret_compare(&sig.key_id, &keys.get(&sig.key_id).unwrap()) {
        return format!("Your keys didn't match...fuck off")
    }

    let mut m = state.hosts.lock().unwrap();
    m.insert(host_data.name.clone(), host_data.ip.clone());

    // Sync the changes immediately
    state.sync();
}


pub fn initialise(state: DDOS) {
    rocket::ignite()
        .mount("/", routes![query, host_update])
        .manage(state)
        .launch();
}
