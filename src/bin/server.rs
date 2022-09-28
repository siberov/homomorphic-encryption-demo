#[macro_use] extern crate rocket;

use concrete::{set_server_key};
use rocket::data::{ToByteUnit, Limits};
use rocket::serde::json::{Json, json, Value};

mod common;
 
#[post("/add", format = "application/json", data = "<dat>")]
fn add(dat: &[u8]) -> Value {
    println!("Got connection!");
    let payload : common::Payload = bincode::deserialize(dat).unwrap();
    set_server_key(payload.key.to_owned());
    
    let a = payload.a.clone();
    let b = payload.b.clone();

    let res = a + b;

    let response = bincode::serialize(&res).unwrap();

    response
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("limits", Limits::new().limit("json", 512.mebibytes())));

    rocket::custom(figment).mount("/", routes![add])
}
