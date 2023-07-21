use wasm_bindgen::prelude::*;
use kfc_core::ecvrf::keygen;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let result = keygen();
    match result {
        Ok(res) => {
            alert(format!("{}", res).as_str());
        }
        Err(e) => {
            alert(e.to_string().as_str());
        }
    }
}