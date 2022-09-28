use concrete::{set_server_key, FheUint8};
use std::{net::{TcpListener, SocketAddr}, slice, mem, env};
use sha2::{Sha256, Digest};

mod common;
 
fn main() {
    let socket_addr: SocketAddr;
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        socket_addr = common::DEFAULT_ADDRESS.parse().unwrap();
    } else if args.len() == 2 {
        socket_addr = match ("0.0.0.0:".to_string() + &args[1]).parse() {
            Ok(addr) => addr,
            Err(_) => {
                println!("Invalid port number. Exiting");
                return
            }
        }
    } else {
        println!("Invalid number of arguments. Supply 0 arguments to use the default port, or supply exactly 1 argument defining a port number.");
        return
    }

    let listener = match TcpListener::bind(socket_addr) {
        Ok(l) => l,
        Err(_) => {
            println!("Error binding to port {}. Exiting.", socket_addr);
            return
        }
    };

    println!("Waiting for connection ...");
    let stream = match listener.accept() {
        Ok((s, _)) => s,
        Err(err) => {
            println!("Error establishing connection: {}. Exiting.", err);
            return
        }
    };

    println!("Got connection from {}.", stream.peer_addr().unwrap());
    let payload : common::Payload = match bincode::deserialize_from(&stream) {
        Ok(res) => res,
        Err(_) => {
            println!("Error deserializing payload. Exiting");
            return
        }
    };

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

    if bincode::serialize_into(stream, &res).is_err() {
        println!("Error sending reply. Exiting.");
        return
    }
}
