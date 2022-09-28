use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use concrete::prelude::*;
use std::net::{TcpStream, SocketAddr};
use bincode::{self, serialize_into};
use std::{env, fmt, error};

mod common;

#[derive(Debug)]
struct ArgumentError {
    details: String
}

impl ArgumentError {
    fn new(msg: &str) -> ArgumentError {
        ArgumentError { details: msg.to_string() }
    }
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for ArgumentError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn get_args() -> Result<(u8, u8, SocketAddr), ArgumentError> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        return Err(ArgumentError::new("Wrong number of arguments"))
    }

    let a: u8 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => return Err(ArgumentError::new("Invalid arguments"))
    };

    let b: u8 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => return Err(ArgumentError::new("Invalid arguments"))
    };

    let addr: SocketAddr;
    if args.len() == 4 {
        addr = match args[2].parse() {
            Ok(sock) => sock,
            Err(_) => return Err(ArgumentError::new("Invalid address"))
        }
    } else {
        addr = common::DEFAULT_ADDRESS.parse().unwrap();
    }

    return Ok((a, b, addr));
}

fn main() {
    let usage = "\
Usage: client <a> <b> <addr>, to calculate <a> + <b> with <addr> as remote host.
Both <a> and <b> must be in the range 0 .. 255.
If <addr> is omitted, it defaults to ".to_owned() + common::DEFAULT_ADDRESS;

    let (clear_a, clear_b, addr) = match get_args() {
        Ok((a, b, addr)) => (a, b, addr),
        Err(err) => {
            println!("{}\n{}", err, usage);
            return
        }
    };

    let stream = match TcpStream::connect(addr) {
        Ok(strm) => strm,
        Err(_) => {
            println!("Error connecting to remote host. Exiting.");
            return
        }
    };

    println!("Connected to {}.", addr);
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

    let payload = common::Payload {
        a,
        b,
        key: server_key_clone
    };

    println!("Sending serialized data to {}.", addr);
    if serialize_into(&stream, &payload).is_err() {
        println!("Error sending to remote host.");
        return
    }

    let result : FheUint8 = match bincode::deserialize_from(&stream) {
        Ok(res) => res,
        Err(_) => {
            println!("Error deserializing answer.");
            return
        }
    };
    let decrypted_result: u8 = result.decrypt(&client_key);
    println!("Decrypted answer: {}", decrypted_result);
}
