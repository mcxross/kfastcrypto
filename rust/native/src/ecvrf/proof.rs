use crate::model::proof::Proof;
use fastcrypto::vrf::ecvrf::{ECVRFKeyPair, ECVRFPrivateKey};
use fastcrypto::vrf::{VRFKeyPair, VRFProof};
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::io::{Error, ErrorKind};

fn prove(input: String, secret_key: String) -> Result<Proof, Error> {
    // Parse inputs
    let secret_key_bytes = hex::decode(secret_key)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid private key."))?;
    let alpha_string = hex::decode(input)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid input string."))?;

    // Create keypair from the secret key bytes
    let secret_key = bincode::deserialize::<ECVRFPrivateKey>(&secret_key_bytes)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Failed to parse private key."))?;
    let kp = ECVRFKeyPair::from(secret_key);

    // Generate proof
    let proof = kp.prove(&alpha_string);
    let proof_str = hex::encode(bincode::serialize(&proof).unwrap());
    let proof_hash = hex::encode(proof.to_hash());

    Ok(Proof {
        proof_str,
        proof_hash,
    })
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_prove<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    input: JString<'local>,
    secretkey: JString<'local>,
) -> jstring {
    let input = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();
    let secrete_key = env
        .get_string(&secretkey)
        .expect("Couldn't get java string!")
        .into();

    let result = prove(input, secrete_key);

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
                .expect("Couldn't ceate java string");
            output.into_raw()
        }
    }
}
