//! Module that provides the RESTful API for remote host configuration

use rocket;
use rocket::State;
use rocket::response::content::Json;

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
fn query(host: String, payload: Option<Signature>, ddos: State<DDOS>) -> String {
  
    /* "Find" a host */
    let hq = ddos.hosts.get(&host);

    match (hq, &payload) {
        (Some(h), &Some(ref sig)) => {
            return format!("Host: {}\nSignature: {:?}", h, sig.signature);
        },
        (_, _) => return format!("INVALID PARAMETERS {}, {:?}!", host, &payload)
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    complete: bool
}

#[post("/todo", data = "<task>")]
fn test_function(task: Json<Task>) -> String { 
    return format!("Foooo");
}


pub fn initialise(state: DDOS) {
    rocket::ignite()
        .mount("/", routes![query])
        .manage(state)
        .launch();
}
