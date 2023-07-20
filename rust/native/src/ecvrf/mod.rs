use jni::objects::JClass;
use jni::objects::JString;
use jni::sys::jstring;
use jni::sys::{jboolean, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use kfc_core::ecvrf;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_generateKeypair<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    let result = ecvrf::keygen();
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
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_prove<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
    secret_key: JString<'local>,
) -> jstring {
    let input = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();
    let secrete_key = env
        .get_string(&secret_key)
        .expect("Couldn't get java string!")
        .into();

    let result = ecvrf::prove(input, secrete_key);

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
                .expect("Couldn't ceate java string");
            output.into_raw()
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_verify<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
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

    let result = ecvrf::verify(output, proof, input, public_key);

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
