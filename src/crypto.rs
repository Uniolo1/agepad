// SPDX-License-Identifier: GPL-3.0-only

use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

static USE_RAGE: AtomicBool = AtomicBool::new(false);
pub fn setup_age() -> bool {
    // check to make sure 'age' is installed
    match Command::new("age")
        .arg("--help")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Err(_) => {
            // try 'rage'
            match Command::new("rage")
                .arg("--help")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
            {
                Err(_) => false,
                Ok(v) => {
                    USE_RAGE.store(true, Ordering::Relaxed);
                    v.success()
                }
            }
        }
        Ok(v) => v.success(),
    }
}

fn age_or_rage() -> Command {
    match USE_RAGE.load(Ordering::Relaxed) {
        true => Command::new("rage"),
        false => Command::new("age"),
    }
}

pub fn decrypt(text: &str, seckey_file: &str) -> Result<String, String> {
    // logic to decrypt
    let mut child = age_or_rage()
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
    let mut child = age_or_rage()
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

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn decrypt_text() {
        // uses testdata/enc/test1.age
        let encrypted =
            fs::read_to_string("./testdata/enc/test1.age").expect("Failed to read file");
        let decrypted =
            fs::read_to_string("./testdata/enc/test1.txt").expect("Failed to read file");
        assert_eq!(
            decrypted.trim_end(),
            decrypt(encrypted.trim_end(), "./testdata/testkey.sec.txt").expect("Decrypt failed")
        );
    }

    fn encrypt_text() {
        let decrypted = "Why do we even use Lorem Ipsum? What is Lorem Ipsum?";
        let encrypted =
            encrypt(decrypted, "./testdata/testkey.pub.txt").expect("Failed to encrypt");
        assert_eq!(
            decrypted,
            decrypt(encrypted.as_str(), "./testdata/testkey.sec.txt").expect("Failed to encrypt")
        );
    }

    #[test]
    fn decrypt_text_age() {
        USE_RAGE.store(false, Ordering::Relaxed);
        decrypt_text()
    }

    #[test]
    fn encrypt_text_age() {
        USE_RAGE.store(false, Ordering::Relaxed);
        encrypt_text()
    }

    #[test]
    fn decrypt_text_rage() {
        USE_RAGE.store(true, Ordering::Relaxed);
        decrypt_text()
    }

    #[test]
    fn encrypt_text_rage() {
        USE_RAGE.store(true, Ordering::Relaxed);
        encrypt_text()
    }
}
