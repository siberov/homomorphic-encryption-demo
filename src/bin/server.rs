use concrete::{set_server_key, FheUint8};
use std::{net::TcpListener, slice, mem};
use sha2::{Sha256, Digest};

mod common;
 
fn main() -> Result<(), bincode::Error> {
    println!("Waiting for connection ...");
    let (listener, _) = TcpListener::bind("127.0.0.1:50000").unwrap().accept().unwrap();
    println!("Got connection.");
    let payload : common::Payload = bincode::deserialize_from(&listener).unwrap();

    set_server_key(payload.key.to_owned());
    
    let a = payload.a.clone();
    let b = payload.b.clone();

    let a_bytes = unsafe { slice::from_raw_parts(&a as *const _ as *const u8, mem::size_of::<FheUint8>()) }.to_vec();
    let b_bytes = unsafe { slice::from_raw_parts(&b as *const _ as *const u8, mem::size_of::<FheUint8>()) }.to_vec();

    let res : FheUint8 = a + b;

    let res_bytes = unsafe { slice::from_raw_parts(&res as *const _ as *const u8, mem::size_of::<FheUint8>()) }.to_vec();

    let mut hasher = Sha256::new();
    hasher.update(a_bytes);
    let a_hash = hasher.finalize();
    hasher = Sha256::new();
    hasher.update(b_bytes);
    let b_hash = hasher.finalize();
    hasher = Sha256::new();
    hasher.update(res_bytes);
    let res_hash = hasher.finalize();
    println!("Hash of a:\t{:02X?}\nHash of b:\t{:02X?}\nHash of a + b:\t{:02X?}", a_hash, b_hash, res_hash);

    bincode::serialize_into(listener, &res)
}
