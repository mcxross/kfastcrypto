use fastcrypto::encoding::{Base64, Encoding, Hex};
use std::io::{Error, ErrorKind};

pub fn base64_to_hex(byte: String) -> Result<String, Error> {
    let val = Base64::decode(&byte)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid base64 string"))?;
    Ok(Hex::encode(val))
}

pub fn hex_to_base64(byte: String) -> Result<String, Error> {
    let val = Hex::decode(&byte)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid hex string"))?;
    Ok(Base64::encode(val))
}
