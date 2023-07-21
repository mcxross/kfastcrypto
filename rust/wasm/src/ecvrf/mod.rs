use kfc_core::ecvrf;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn keygen() -> Result<String, JsValue> {
    let keypair = match ecvrf::keygen() {
        Ok(pair) => pair,
        Err(e) => return Err(JsValue::from_str(&e.to_string())),
    };
    Ok(format!("{}", keypair))
}

#[wasm_bindgen]
pub fn prove(input: &str, secret_key: &str) -> Result<String, JsValue> {
    let proof = match ecvrf::prove(input.to_string(), secret_key.to_string()) {
        Ok(proof) => proof,
        Err(e) => return Err(JsValue::from_str(&e.to_string())),
    };
    Ok(format!("{}", proof))
}

#[wasm_bindgen]
pub fn verify(output: &str, proof: &str, input: &str, pub_key: &str) -> Result<bool, JsValue> {
    let result = ecvrf::verify(
        output.to_string(),
        proof.to_string(),
        input.to_string(),
        pub_key.to_string(),
    );
    if result.is_ok() {
        Ok(true)
    } else {
        Err(JsValue::from_str("Verification failed"))
    }
}