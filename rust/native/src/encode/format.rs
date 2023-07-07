use fastcrypto::encoding::{Base64, Encoding, Hex};
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::io::{Error, ErrorKind};

pub(crate) fn base64_to_hex(byte: String) -> Result<String, Error> {
    let val = Base64::decode(&byte)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid base64 string"))?;
    Ok(Hex::encode(val))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_base64ToHex<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    byte: JString<'local>,
) -> jstring {
    let byte = env
        .get_string(&byte)
        .expect("Couldn't get java string <byte>!")
        .into();

    let result = base64_to_hex(byte);

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

pub(crate) fn hex_to_base64(byte: String) -> Result<String, Error> {
    let val = Hex::decode(&byte)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid hex string"))?;
    Ok(Base64::encode(val))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcryto_FastCryptoApi_hexToBase64<'local>(
    mut env: JNIEnv<'local>,
    _jobject: JClass<'local>,
    byte: JString<'local>,
) -> jstring {
    let byte = env
        .get_string(&byte)
        .expect("Couldn't get java string <byte>!")
        .into();

    let result = hex_to_base64(byte);

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

#[cfg(test)]
mod tests {
    use crate::encode::format::{base64_to_hex, hex_to_base64};

    #[test]
    fn test_encode_base64_to_hex() {
        // The correctness of the output is tested in the integration tests in fastcrypto-cli/tests/encode_cli.rs.
        let base64 = "SGVsbG8gV29ybGQh";
        assert!(base64_to_hex(base64.to_string()).is_ok());

        let invalid_base64 = "SGVsbG8gV29ybGQ";
        assert!(base64_to_hex(invalid_base64.to_string()).is_err());
    }

    #[test]
    fn test_encode_hex_to_base64() {
        // The correctness of the output is tested in the integration tests in fastcrypto-cli/tests/encode_cli.rs.
        let hex = "48656c6c6f20576f726c6421";
        assert!(hex_to_base64(hex.to_string()).is_ok());

        let invalid_hex = "48656c6c6f20576f726c642";
        assert!(hex_to_base64(invalid_hex.to_string()).is_err());
    }
}
