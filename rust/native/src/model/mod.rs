use std::fmt::{Display, Formatter};

pub(crate) struct Keypair {
    pub(crate) public_key: String,
    pub(crate) private_key: String,
}

impl Display for Keypair {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            r#"{{ "publicKey": "{}", "privateKey": "{}" }}"#,
            self.public_key, self.private_key
        )
    }
}