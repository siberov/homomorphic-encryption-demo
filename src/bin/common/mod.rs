#[derive(serde::Serialize, serde::Deserialize)]
pub struct Payload {
    pub a: concrete::FheUint8,
    pub b: concrete::FheUint8,
    pub key: concrete::ServerKey
}

pub const DEFAULT_ADDRESS: &str = "127.0.0.1:50000";