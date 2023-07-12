use crate::model::scheme::SignatureScheme;
use fastcrypto::ed25519::{Ed25519PublicKey, Ed25519Signature};
use fastcrypto::encoding::{Encoding, Hex};
use fastcrypto::error::FastCryptoError;
use fastcrypto::secp256k1::recoverable::Secp256k1RecoverableSignature;
use fastcrypto::secp256k1::{Secp256k1PublicKey, Secp256k1Signature};
use fastcrypto::secp256r1::recoverable::Secp256r1RecoverableSignature;
use fastcrypto::secp256r1::{Secp256r1PublicKey, Secp256r1Signature};
use fastcrypto::traits::{ToFromBytes, VerifyRecoverable, VerifyingKey};
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use std::str::FromStr;

fn verify(
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_sigsVerify<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    msg: JString<'local>,
    sig: JString<'local>,
    public_key: JString<'local>,
    scheme: JString<'local>,
) -> jboolean {
    let msg = env
        .get_string(&msg)
        .expect("Couldn't get java string <msg>!")
        .into();

    let sig = env
        .get_string(&sig)
        .expect("Couldn't get java string <sig>!")
        .into();

    let public_key = env
        .get_string(&public_key)
        .expect("Couldn't get java string <public_key>!")
        .into();

    let scheme = env
        .get_string(&scheme)
        .expect("Couldn't get java string <scheme>!")
        .into();

    let result = verify(msg, sig, public_key, scheme);

    match result {
        Ok(res) => {
            if res {
                JNI_TRUE
            } else {
                JNI_FALSE
            }
        }
        Err(_) => JNI_FALSE,
    }
}
