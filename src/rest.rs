//! Module that provides the RESTful API for remote host configuration

use rocket;
use rocket::{Request, Route, Data};
use rocket::handler::Outcome;
use rocket::http::Method;
use rocket::*;
use rocket::request::*;

use core::DDOS;

#[derive(FromForm)]
#[derive(Debug)]
struct Signature {
    signature: String,
    key_id: String,
}

#[get("/query/<host>?<payload>")]
fn query(host: String, payload: Option<Signature>, ddos: State<DDOS>) -> String {
  
  /* "Find" a host */
  let hq = ddos.hosts.get(&host);

  match (hq, &payload) {
    (Some(h), &Some(ref sig)) => {
      return format!("Host: {}\nSignature: {:?}", h, sig.signature);
    },
    (_, _) => return format!("INVALID PARAMETERS {}, {:?}!", host, &payload)
  }

  return format!("INVALID PARAMETERS!");

  // match pl {
  //     Some(sig) => ,
  //     _ => return format!("INVALID PARAMETERS!"),
  // }

  // let my_host = ddos.hosts.get(&host).unwrap();

  // return format!("Getting host config for {}: '{}'. Signature: {}", 
    // host, my_host, pl.unwrap().signature);
}

pub fn initialise(state: DDOS) {
    rocket::ignite()
      .mount("/", routes![query])
      .manage(state)
      .launch();
}
