#[derive(serde::Serialize, serde::Deserialize)]
pub struct Payload {
    pub a: concrete::FheUint8,
    pub b: concrete::FheUint8,
    pub key: concrete::ServerKey
}