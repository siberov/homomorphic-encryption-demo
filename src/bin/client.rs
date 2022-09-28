use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use concrete::prelude::*;
use std::net::TcpStream;
use bincode::{self, serialize_into};
use std::env;

mod common;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: client <a> <b>, to calculate <a> + <b>.\nBoth <a> and <b> must be in the range 0 .. 255");
        return
    }
    let clear_a: u8 = str::parse(&args[1]).unwrap();
    let clear_b: u8 = str::parse(&args[2]).unwrap();

    let addr = "127.0.0.1:50000";
    let stream = TcpStream::connect(addr).unwrap();
    println!("Encrypting input and generating keys. This may take a while ...");

    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();

    let (client_key, server_key) = generate_keys(config);

    let server_key_clone = server_key.clone();
    set_server_key(server_key);

    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);

    println!("Finished encryption.");

    println!("Serialized sizes\na: {} bytes\nb: {} bytes\nkey: {} bytes",
        bincode::serialized_size(&a).unwrap(),
        bincode::serialized_size(&b).unwrap(),
        bincode::serialized_size(&server_key_clone).unwrap());

    let payload = common::Payload {
        a,
        b,
        key: server_key_clone
    };

    serialize_into(&stream, &payload);

    let result : FheUint8 = bincode::deserialize_from(&stream).unwrap();
    let decrypted_result: u8 = result.decrypt(&client_key);

    println!("Decrypted answer: {}", decrypted_result);
}
