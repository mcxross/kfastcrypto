use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::sys::{jboolean, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use kfc_core::sigs;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_sigsGenerateKeypair<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    scheme: JString<'local>,
    seed: JString<'local>,
) -> jstring {
    let scheme = env
        .get_string(&scheme)
        .expect("Couldn't get java string <scheme>!")
        .into();

    let seed = env
        .get_string(&seed)
        .expect("Couldn't get java string <seed>!")
        .into();

    let result = sigs::keygen(scheme, seed);

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
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_sigsSign<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    msg: JString<'local>,
    secret_key: JString<'local>,
    scheme: JString<'local>,
) -> jstring {
    let msg = env
        .get_string(&msg)
        .expect("Couldn't get java string <msg>!")
        .into();

    let sk = env
        .get_string(&secret_key)
        .expect("Couldn't get java string <secret_key>!")
        .into();

    let scheme = env
        .get_string(&scheme)
        .expect("Couldn't get java string <scheme>!")
        .into();

    let result = sigs::sign(msg, sk, scheme);

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
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_sigsVerify<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    msg: JString<'local>,
    sig: JString<'local>,
    public_key: JString<'local>,
    scheme: JString<'local>,
) -> jboolean {
    let msg = env
        .get_string(&msg)
        .expect("Couldn't get java string <msg>!")
        .into();

    let sig = env
        .get_string(&sig)
        .expect("Couldn't get java string <sig>!")
        .into();

    let public_key = env
        .get_string(&public_key)
        .expect("Couldn't get java string <public_key>!")
        .into();

    let scheme = env
        .get_string(&scheme)
        .expect("Couldn't get java string <scheme>!")
        .into();

    let result = sigs::verify(msg, sig, public_key, scheme);

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
