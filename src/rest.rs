//! Module that provides the RESTful API for remote host configuration

use rocket;
use rocket::*;

use core::DDOS;

#[get("/query/<host>")]
fn query(host: String, ddos: State<DDOS>) -> String {
  let my_host = ddos.hosts.get(&host).unwrap();

  return format!("Getting host config for {}: '{}'", host, my_host);
}

pub fn initialise(state: DDOS) {
    rocket::ignite()
      .mount("/", routes![query])
      .manage(state)
      .launch();
}
