use std::fmt;

pub(crate) struct Sign {
    pub(crate) sig: String,
    pub(crate) pub_key: String,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{ "sig": "{}", "publicKey": "{}" }}"#,
            self.sig, self.pub_key
        )
    }
}
