use crate::model::Keypair;
use fastcrypto::vrf::ecvrf::ECVRFKeyPair;
use fastcrypto::vrf::VRFKeyPair;
use jni::objects::JClass;
use jni::sys::jstring;
use jni::JNIEnv;
use rand::thread_rng;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};

pub(crate) fn keygen() -> Result<Keypair, Error> {
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_xyz_mcxross_kfastcrypto_FastCryptoApi_generateKeypair<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
) -> jstring {
    let result = keygen();
    match result {
        Ok(res) => {
            let output = env
                .new_string(format!("{}", res))
                .expect("Couldn't create java string!");
            output.into_raw()
        }
        Err(e) => {
            let output = env.new_string("").expect("Couldn't create java string!");
            output.into_raw()
        }
    }
}
