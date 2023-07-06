use crate::model::key_pair::Keypair;
use fastcrypto::{
    ed25519::Ed25519KeyPair,
    encoding::{Encoding, Hex},
    error::FastCryptoError,
    secp256k1::Secp256k1KeyPair,
    secp256r1::Secp256r1KeyPair,
    traits::KeyPair,
};
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

enum SignatureScheme {
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

pub(crate) fn keygen(scheme: String, seed: String) -> Result<Keypair, FastCryptoError> {
    // Check if the seed string is empty
    let seed_bytes: [u8; 32];
    if seed.is_empty() {
        // Generate a random seed if the seed string is empty
        let mut rng = StdRng::from_entropy();
        seed_bytes = rng.gen()
    } else {
        // Convert the seed from hex-encoded string to a byte array
        let arr = Hex::decode(&seed).map_err(|_| FastCryptoError::InvalidInput)?;

        // Convert the byte array into a fixed-size array of length 32
        seed_bytes = arr.try_into().map_err(|_| FastCryptoError::InvalidInput)?;
    }

    let rng = &mut StdRng::from_seed(seed_bytes);

    let (sk, pk) = match SignatureScheme::from_str(&scheme) {
        Ok(SignatureScheme::Ed25519) => {
            let kp = Ed25519KeyPair::generate(rng);
            (
                Hex::encode(kp.copy().private().as_ref()),
                Hex::encode(kp.public().as_ref()),
            )
        }
        Ok(SignatureScheme::Secp256k1) | Ok(SignatureScheme::Secp256k1Recoverable) => {
            let kp = Secp256k1KeyPair::generate(rng);
            (
                Hex::encode(kp.copy().private().as_ref()),
                Hex::encode(kp.public().as_ref()),
            )
        }
        Ok(SignatureScheme::Secp256r1) | Ok(SignatureScheme::Secp256r1Recoverable) => {
            let kp = Secp256r1KeyPair::generate(rng);
            (
                Hex::encode(kp.copy().private().as_ref()),
                Hex::encode(kp.public().as_ref()),
            )
        }
        Ok(SignatureScheme::BLS12381MinSig) => {
            let kp = fastcrypto::bls12381::min_sig::BLS12381KeyPair::generate(rng);
            (
                Hex::encode(kp.copy().private().as_ref()),
                Hex::encode(kp.public().as_ref()),
            )
        }
        Ok(SignatureScheme::BLS12381MinPk) => {
            let kp = fastcrypto::bls12381::min_pk::BLS12381KeyPair::generate(rng);
            (
                Hex::encode(kp.copy().private().as_ref()),
                Hex::encode(kp.public().as_ref()),
            )
        }
        Err(_) => return Err(FastCryptoError::InvalidInput),
    };

    Ok(Keypair {
        private_key: sk,
        public_key: pk,
    })
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_sigsGenerateKeypair<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    scheme: JString<'local>,
    seed: JString<'local>,
) -> jstring {
    let scheme = env
        .get_string(&scheme)
        .expect("Couldn't get java string <scheme>!")
        .into();

    let seed = env
        .get_string(&seed)
        .expect("Couldn't get java string <seed>!")
        .into();

    let result = keygen(scheme, seed);

    match result {
        Ok(res) => {
            let output = env
                .new_string(format!("{}", res))
                .expect("Couldn't create java string!");
            output.into_raw()
        }
        Err(e) => {
            let output = env
                .new_string(e.to_string())
                .expect("Couldn't create java string!");
            output.into_raw()
        }
    }
}
