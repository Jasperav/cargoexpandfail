use std::process::{Command, ExitStatus};

mod expand_me;

fn main() {
    // How do I expand 'expand_me.rs'?

    let output = Command::new("cargo")
        .arg("expand")
        .arg("--expand_me")
        .output()
        .unwrap();

    assert!(output.stderr.is_empty());
}
