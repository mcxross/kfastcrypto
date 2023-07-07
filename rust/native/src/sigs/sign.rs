use crate::model::sign::Sign;
use crate::sigs::sig_scheme::SignatureScheme;
use fastcrypto::ed25519::{Ed25519KeyPair, Ed25519PrivateKey};
use fastcrypto::encoding::{Encoding, Hex};
use fastcrypto::error::FastCryptoError;
use fastcrypto::secp256k1::{Secp256k1KeyPair, Secp256k1PrivateKey};
use fastcrypto::secp256r1::{Secp256r1KeyPair, Secp256r1PrivateKey};
use fastcrypto::traits::{KeyPair, RecoverableSigner, Signer, ToFromBytes};
use std::str::FromStr;


pub(crate) fn sign(
    msg: String,
    secret_key: String,
    scheme: String,
) -> Result<Sign, FastCryptoError> {
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
