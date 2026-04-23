// SPDX-License-Identifier: GPL-3.0-only

use std::io::Write;
use std::process::{Command, Stdio};

pub fn does_age_exist() -> bool {
    // check to make sure 'age' is installed
    match Command::new("age")
        .arg("--help")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Err(_) => false,
        Ok(v) => v.success(),
    }
}

pub fn decrypt(text: &str, seckey_file: &str) -> Result<String, String> {
    // logic to decrypt
    let mut child = Command::new("age")
        .arg("-d") // decrypt
        .arg("-i") // specify seckey_file
        .arg(seckey_file)
        .stdin(Stdio::piped()) // IO
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() // create
        .map_err(|e| e.to_string())?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| e.to_string())?;
    } // stdin is dropped here so an EOF is sent

    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn encrypt(text: &str, recipient_file: &str) -> Result<String, String> {
    // logic to encrypt
    let mut child = Command::new("age")
        .arg("-a") // ASCI armor the result
        .arg("-R") // specify recipients
        .arg(recipient_file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() // create
        .map_err(|e| e.to_string())?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| e.to_string())?;
    } // stdin is dropped here so an EOF is sent

    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
