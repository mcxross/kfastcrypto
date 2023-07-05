use fastcrypto::vrf::ecvrf::{ECVRFProof, ECVRFPublicKey};
use fastcrypto::vrf::VRFProof;
use jni::objects::{JBooleanArray, JClass, JString};
use jni::sys::{jarray, jboolean, jclass, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use std::io::{Error, ErrorKind};

fn verify(output: String, proof: String, input: String, public_key: String) -> Result<bool, Error> {
    // Parse inputs
    let public_key_bytes = hex::decode(public_key)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid public key."))?;
    let alpha_string = hex::decode(input)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid input string."))?;
    let proof_bytes = hex::decode(proof)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid proof string."))?;
    let output_bytes = hex::decode(output)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid output string."))?;
    let output: [u8; 64] = output_bytes
        .try_into()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Output must be 64 bytes."))?;

    // Create public key and proof from parsed bytes
    let public_key: ECVRFPublicKey = bincode::deserialize::<ECVRFPublicKey>(&public_key_bytes)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid public key."))?;
    let proof: ECVRFProof = bincode::deserialize::<ECVRFProof>(&proof_bytes)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Unable to parse proof."))?;

    return if proof
        .verify_output(&alpha_string, &public_key, &output)
        .is_ok()
    {
        Ok(true)
    } else {
        Ok(false)
    };
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_verify<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    output: JString<'local>,
    proof: JString<'local>,
    input: JString<'local>,
    public_key: JString<'local>,
) -> jboolean {
    let output = env
        .get_string(&output)
        .expect("Couldn't get java string <output>!")
        .into();
    let proof = env
        .get_string(&proof)
        .expect("Couldn't get java string <proof>!")
        .into();
    let input = env
        .get_string(&input)
        .expect("Couldn't get java string <input>!")
        .into();
    let public_key = env
        .get_string(&public_key)
        .expect("Couldn't get java string <public_key>!")
        .into();

    let result = verify(output, proof, input, public_key);

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
