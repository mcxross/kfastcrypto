use fastcrypto::vrf::ecvrf::{ECVRFProof, ECVRFPublicKey};
use fastcrypto::vrf::VRFProof;
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

    if proof
        .verify_output(&alpha_string, &public_key, &output)
        .is_ok()
    {
        return Ok(true);
    }
    Err(Error::new(ErrorKind::Other, "Proof is not correct."))
}