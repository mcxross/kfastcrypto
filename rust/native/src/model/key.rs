use std::fmt;

pub(crate) struct Keypair {
    pub(crate) public_key: String,
    pub(crate) private_key: String,
}

impl fmt::Display for Keypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{ "publicKey": "{}", "privateKey": "{}" }}"#,
            self.public_key, self.private_key
        )
    }
}
