use std::fmt::{Display, Formatter, Result as FmtResult};
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use fastcrypto::vrf::{VRFKeyPair, VRFProof};
use fastcrypto::vrf::ecvrf::{ECVRFKeyPair, ECVRFPrivateKey, ECVRFProof, ECVRFPublicKey};
use rand::thread_rng;
use std::io::{Error, ErrorKind};


struct Keypair {
    public_key: String,
    private_key: String,
}

impl Display for Keypair {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            r#"{{ "publicKey": "{}", "privateKey": "{}" }}"#,
            self.public_key, self.private_key
        )
    }
}

fn keygen() -> Result<Keypair, Error> {
    let keypair = ECVRFKeyPair::generate(&mut thread_rng());
    let sk_string =
        hex::encode(bincode::serialize(&keypair.sk).map_err(|_| {
            Error::new(ErrorKind::Other, "Failed to serialize secret key.")
        })?);
    let pk_string =
        hex::encode(bincode::serialize(&keypair.pk).map_err(|_| {
            Error::new(ErrorKind::Other, "Failed to serialize public key.")
        })?);
    Ok(Keypair {
        private_key: sk_string,
        public_key: pk_string,
    })
}

#[allow(non_snake_case)]
// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_ktfastcrypto_FastCryptoApi_generateKeypair<'local>(mut env: JNIEnv<'local>, class: JClass<'local>) -> jstring {
    let result = keygen();
    match result {
        Ok(res) => {
            let output = env.new_string(format!("{}", res))
                .expect("Couldn't create java string!");
            output.into_raw()
        }
        Err(e) => {
            let output = env.new_string("").expect("Couldn't create java string!");
            output.into_raw()
        }
    }
}

struct Proof {
    str: String,
    hash: String,
}

fn prove(input: String, secret_key: String) -> Result<Proof, Error> {
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
    let proof_string = hex::encode(bincode::serialize(&proof).unwrap());
    let proof_hash = hex::encode(proof.to_hash());

    Ok(Proof {
        str: proof_string,
        hash: proof_hash,
    })
}

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
    let public_key: ECVRFPublicKey =
        bincode::deserialize::<ECVRFPublicKey>(&public_key_bytes)
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid public key."))?;
    let proof: ECVRFProof = bincode::deserialize::<ECVRFProof>(&proof_bytes)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Unable to parse proof."))?;

    if proof
        .verify_output(&alpha_string, &public_key, &output)
        .is_ok()
    {
        return Ok(true);
    }
    Err(Error::new(ErrorKind::Other, "Proof is not correct."))
}