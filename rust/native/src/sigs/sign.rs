use crate::model::scheme::SignatureScheme;
use crate::model::sign::Sign;
use fastcrypto::ed25519::{Ed25519KeyPair, Ed25519PrivateKey};
use fastcrypto::encoding::{Encoding, Hex};
use fastcrypto::error::FastCryptoError;
use fastcrypto::secp256k1::{Secp256k1KeyPair, Secp256k1PrivateKey};
use fastcrypto::secp256r1::{Secp256r1KeyPair, Secp256r1PrivateKey};
use fastcrypto::traits::{KeyPair, RecoverableSigner, Signer, ToFromBytes};
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::str::FromStr;

fn sign(msg: String, secret_key: String, scheme: String) -> Result<Sign, FastCryptoError> {
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_sigsSign<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    msg: JString<'local>,
    secret_key: JString<'local>,
    scheme: JString<'local>,
) -> jstring {
    let msg = env
        .get_string(&msg)
        .expect("Couldn't get java string <msg>!")
        .into();

    let sk = env
        .get_string(&secret_key)
        .expect("Couldn't get java string <secret_key>!")
        .into();

    let scheme = env
        .get_string(&scheme)
        .expect("Couldn't get java string <scheme>!")
        .into();

    let result = sign(msg, sk, scheme);

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
