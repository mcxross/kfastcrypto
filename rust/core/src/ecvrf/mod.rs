use crate::model::{Keypair, Proof};
use fastcrypto::vrf::ecvrf::{ECVRFKeyPair, ECVRFPrivateKey, ECVRFProof, ECVRFPublicKey};
use fastcrypto::vrf::{VRFKeyPair, VRFProof};
use rand::thread_rng;
use std::io::{Error, ErrorKind};

pub fn keygen() -> Result<Keypair, Error> {
    let keypair = ECVRFKeyPair::generate(&mut thread_rng());
    let sk_string = hex::encode(
        bincode::serialize(&keypair.sk)
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to serialize secret key."))?,
    );
    let pk_string = hex::encode(
        bincode::serialize(&keypair.pk)
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to serialize public key."))?,
    );
    Ok(Keypair {
        private_key: sk_string,
        public_key: pk_string,
    })
}

pub fn prove(input: String, secret_key: String) -> Result<Proof, Error> {
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

pub fn verify(
    output: String,
    proof: String,
    input: String,
    public_key: String,
) -> Result<bool, Error> {
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
