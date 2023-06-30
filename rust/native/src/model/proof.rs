use std::fmt;
use std::fmt::Formatter;

pub(crate) struct Proof {
    pub(crate) proof_str: String,
    pub(crate) proof_hash: String,
}

impl fmt::Display for Proof {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{ "proof_str": "{}", "hash": "{}" }}"#,
            self.proof_str, self.proof_hash
        )
    }
}
