use std::fmt;
use std::fmt::Formatter;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

pub struct Keypair {
    pub public_key: String,
    pub private_key: String,
}

impl fmt::Display for Keypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{ "publicKey": "{}", "privateKey": "{}" }}"#,
            self.public_key, self.private_key
        )
    }
}

pub struct Proof {
    pub proof_str: String,
    pub proof_hash: String,
}

impl fmt::Display for Proof {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{ "proof_str": "{}", "hash": "{}" }}"#,
            self.proof_str, self.proof_hash
        )
    }
}

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
    type Err = Error;

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

pub struct Sign {
    pub sig: String,
    pub pub_key: String,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{ "sig": "{}", "publicKey": "{}" }}"#,
            self.sig, self.pub_key
        )
    }
}
