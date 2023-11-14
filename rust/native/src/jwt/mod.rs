use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfc_FastCryptoApi_parse_and_validate_jwt<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    token: JString<'local>,
) -> jstring {

    let token_str = env
        .get_string(&token)
        .expect("Couldn't get java string!");

    let token = token_str.to_str().expect("Couldn't convert java string to rust string");


    match fastcrypto::jwt_utils::parse_and_validate_jwt(token) {
        Ok((sub, aud)) => {
            let json = format!(r#"{{"sub":"{}","aud":"{}"}}"#, sub, aud);
            let output = env
                .new_string(format!("{}", json))
                .expect("Couldn't create java string!");
            output.into_raw()
        }
        Err(_) => {
            let output = env
                .new_string("Invalid JWT token")
                .expect("Couldn't create java string!");
            output.into_raw()
        }
    }
}