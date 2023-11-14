use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use fc_core::encode;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfc_FastCryptoApi_base64ToHex<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    byte: JString<'local>,
) -> jstring {
    let byte = env
        .get_string(&byte)
        .expect("Couldn't get java string <byte>!")
        .into();

    let result = encode::base64_to_hex(byte);

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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfc_FastCryptoApi_hexToBase64<'local>(
    mut env: JNIEnv<'local>,
    _jobject: JClass<'local>,
    byte: JString<'local>,
) -> jstring {
    let byte = env
        .get_string(&byte)
        .expect("Couldn't get java string <byte>!")
        .into();

    let result = encode::hex_to_base64(byte);

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
    use fc_core::encode::{base64_to_hex, hex_to_base64};

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
