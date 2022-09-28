use concrete::{set_server_key, FheUint8};
use std::net::TcpListener;

mod common;
 
fn main() -> Result<(), bincode::Error> {
    let (listener, _) = TcpListener::bind("127.0.0.1:50000").unwrap().accept().unwrap();
    println!("Got connection!");
    let payload : common::Payload = bincode::deserialize_from(&listener).unwrap();

    set_server_key(payload.key.to_owned());
    
    let a = payload.a.clone();
    let b = payload.b.clone();

    let res : FheUint8 = a + b;

    bincode::serialize_into(listener, &res)
}
