use std::process::{ Command, Output };
use std::borrow::Cow;

pub fn execute_command(cmd: String) -> String {
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd")
        .args(["/C", &cmd])
        .output()
        .unwrap();

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh")
        .args(["-c", &cmd])
        .output()
        .unwrap();

    let stdout: Cow<str> = String::from_utf8_lossy(&output.stdout);
    let stderr: Cow<str> = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        stdout.to_string()
    } else {
        stderr.to_string()
    }
}