use fastcrypto::vrf::ecvrf::{ECVRFKeyPair, ECVRFPrivateKey};
use fastcrypto::vrf::{VRFKeyPair, VRFProof};
use std::io::{Error, ErrorKind};

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
