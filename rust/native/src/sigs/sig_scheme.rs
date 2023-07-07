use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

pub enum SignatureScheme {
    Ed25519,
    Secp256k1,
    Secp256k1Recoverable,
    Secp256r1,
    Secp256r1Recoverable,
    BLS12381MinSig,
    BLS12381MinPk,
}

impl FromStr for SignatureScheme {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ed25519" => Ok(SignatureScheme::Ed25519),
            "secp256k1" => Ok(SignatureScheme::Secp256k1),
            "secp256k1-rec" => Ok(SignatureScheme::Secp256k1Recoverable),
            "secp256r1" => Ok(SignatureScheme::Secp256r1),
            "secp256r1-rec" => Ok(SignatureScheme::Secp256r1Recoverable),
            "bls12381-minsig" => Ok(SignatureScheme::BLS12381MinSig),
            "bls12381-minpk" => Ok(SignatureScheme::BLS12381MinPk),
            _ => Err(Error::new(ErrorKind::Other, "Invalid signature scheme.")),
        }
    }
}
