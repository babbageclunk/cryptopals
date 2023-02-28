use base64::{engine::general_purpose, Engine as _};

pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    general_purpose::STANDARD_NO_PAD.encode(&bytes)
}
