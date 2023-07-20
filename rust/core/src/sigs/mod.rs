use crate::model::{Keypair, Sign, SignatureScheme};
use fastcrypto::ed25519::{Ed25519KeyPair, Ed25519PrivateKey, Ed25519PublicKey, Ed25519Signature};
use fastcrypto::encoding::{Encoding, Hex};
use fastcrypto::error::FastCryptoError;
use fastcrypto::secp256k1::recoverable::Secp256k1RecoverableSignature;
use fastcrypto::secp256k1::{
    Secp256k1KeyPair, Secp256k1PrivateKey, Secp256k1PublicKey, Secp256k1Signature,
};
use fastcrypto::secp256r1::recoverable::Secp256r1RecoverableSignature;
use fastcrypto::secp256r1::{
    Secp256r1KeyPair, Secp256r1PrivateKey, Secp256r1PublicKey, Secp256r1Signature,
};
use fastcrypto::traits::{
    KeyPair, RecoverableSigner, Signer, ToFromBytes, VerifyRecoverable, VerifyingKey,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::str::FromStr;

pub fn keygen(scheme: String, seed: String) -> Result<Keypair, FastCryptoError> {
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

pub fn sign(msg: String, secret_key: String, scheme: String) -> Result<Sign, FastCryptoError> {
    let sk = Hex::decode(&secret_key).map_err(|_| FastCryptoError::InvalidInput)?;
    let msg = Hex::decode(&msg).map_err(|_| FastCryptoError::InvalidInput)?;

    let (pk, sig) = match SignatureScheme::from_str(&scheme) {
        Ok(SignatureScheme::Ed25519) => {
            let kp = Ed25519KeyPair::from(Ed25519PrivateKey::from_bytes(&sk)?);
            (
                Hex::encode(kp.public()),
                Hex::encode(kp.sign(&msg).as_ref()),
            )
        }
        Ok(SignatureScheme::Secp256k1) => {
            let kp = Secp256k1KeyPair::from(Secp256k1PrivateKey::from_bytes(&sk)?);
            (
                Hex::encode(kp.public()),
                Hex::encode(kp.sign(&msg).as_ref()),
            )
        }
        Ok(SignatureScheme::Secp256k1Recoverable) => {
            let kp = Secp256k1KeyPair::from(Secp256k1PrivateKey::from_bytes(&sk)?);
            (
                Hex::encode(kp.public()),
                Hex::encode(kp.sign_recoverable(&msg).as_ref()),
            )
        }
        Ok(SignatureScheme::Secp256r1) => {
            let kp = Secp256r1KeyPair::from(Secp256r1PrivateKey::from_bytes(&sk)?);
            (
                Hex::encode(kp.public()),
                Hex::encode(kp.sign(&msg).as_ref()),
            )
        }
        Ok(SignatureScheme::Secp256r1Recoverable) => {
            let kp = Secp256r1KeyPair::from(Secp256r1PrivateKey::from_bytes(&sk)?);
            (
                Hex::encode(kp.public()),
                Hex::encode(kp.sign_recoverable(&msg).as_ref()),
            )
        }
        Ok(SignatureScheme::BLS12381MinSig) => {
            let kp = fastcrypto::bls12381::min_sig::BLS12381KeyPair::from(
                fastcrypto::bls12381::min_sig::BLS12381PrivateKey::from_bytes(&sk)?,
            );
            (
                Hex::encode(kp.public()),
                Hex::encode(kp.sign(&msg).as_ref()),
            )
        }
        Ok(SignatureScheme::BLS12381MinPk) => {
            let kp = fastcrypto::bls12381::min_pk::BLS12381KeyPair::from(
                fastcrypto::bls12381::min_pk::BLS12381PrivateKey::from_bytes(&sk)?,
            );
            (
                Hex::encode(kp.public()),
                Hex::encode(kp.sign(&msg).as_ref()),
            )
        }
        Err(_) => return Err(FastCryptoError::InvalidInput),
    };
    Ok(Sign { sig, pub_key: pk })
}

pub fn verify(
    msg: String,
    signature: String,
    public_key: String,
    scheme: String,
) -> Result<bool, FastCryptoError> {
    let msg = Hex::decode(&msg).map_err(|_| FastCryptoError::InvalidInput)?;
    let sig = Hex::decode(&signature).map_err(|_| FastCryptoError::InvalidInput)?;
    let pk = Hex::decode(&public_key).map_err(|_| FastCryptoError::InvalidInput)?;

    let res = match SignatureScheme::from_str(&scheme) {
        Ok(SignatureScheme::Ed25519) => {
            let pk =
                Ed25519PublicKey::from_bytes(&pk).map_err(|_| FastCryptoError::InvalidInput)?;
            pk.verify(&msg, &Ed25519Signature::from_bytes(&sig)?)
        }
        Ok(SignatureScheme::Secp256k1) => {
            let pk =
                Secp256k1PublicKey::from_bytes(&pk).map_err(|_| FastCryptoError::InvalidInput)?;
            pk.verify(&msg, &Secp256k1Signature::from_bytes(&sig)?)
        }
        Ok(SignatureScheme::Secp256k1Recoverable) => {
            let pk =
                Secp256k1PublicKey::from_bytes(&pk).map_err(|_| FastCryptoError::InvalidInput)?;
            pk.verify_recoverable(&msg, &Secp256k1RecoverableSignature::from_bytes(&sig)?)
        }
        Ok(SignatureScheme::Secp256r1) => {
            let pk =
                Secp256r1PublicKey::from_bytes(&pk).map_err(|_| FastCryptoError::InvalidInput)?;
            pk.verify(&msg, &Secp256r1Signature::from_bytes(&sig)?)
        }
        Ok(SignatureScheme::Secp256r1Recoverable) => {
            let pk =
                Secp256r1PublicKey::from_bytes(&pk).map_err(|_| FastCryptoError::InvalidInput)?;
            pk.verify_recoverable(&msg, &Secp256r1RecoverableSignature::from_bytes(&sig)?)
        }
        Ok(SignatureScheme::BLS12381MinSig) => {
            let pk = fastcrypto::bls12381::min_sig::BLS12381PublicKey::from_bytes(&pk)
                .map_err(|_| FastCryptoError::InvalidInput)?;
            pk.verify(
                &msg,
                &fastcrypto::bls12381::min_sig::BLS12381Signature::from_bytes(&sig)?,
            )
        }
        Ok(SignatureScheme::BLS12381MinPk) => {
            let pk = fastcrypto::bls12381::min_pk::BLS12381PublicKey::from_bytes(&pk)
                .map_err(|_| FastCryptoError::InvalidInput)?;
            pk.verify(
                &msg,
                &fastcrypto::bls12381::min_pk::BLS12381Signature::from_bytes(&sig)?,
            )
        }
        Err(_) => return Err(FastCryptoError::InvalidInput),
    };
    Ok(res.is_ok())
}
