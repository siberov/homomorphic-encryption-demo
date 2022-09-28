use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use concrete::prelude::*;
use std::net::TcpStream;
use bincode::{self, serialize_into};
// use serde_json::{Serializer, Deserializer};
// use reqwest::blocking::Client;

mod common;

fn main() {
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();
    println!("Config built");

    let (client_key, server_key) = generate_keys(config);
    println!("Keys generated");

    let server_key_clone = server_key.clone();
    set_server_key(server_key);

    let clear_a = 27u8;
    let clear_b = 128u8;

    println!("Encrypting ...");
    let a = FheUint8::encrypt(clear_a, &client_key);
    println!("Encrypted a!");
    let b = FheUint8::encrypt(clear_b, &client_key);
    println!("Encrypted b!");

    let addr = "127.0.0.1:50000";
    let stream = TcpStream::connect(addr).unwrap();

    let payload = common::Payload {
        a,
        b,
        key: server_key_clone
    };

    serialize_into(&stream, &payload);

    let result : FheUint8 = bincode::deserialize_from(&stream).unwrap();

    let decrypted_result: u8 = result.decrypt(&client_key);

    println!("Answer: {}", decrypted_result);
}
