// SPDX-License-Identifier: GPL-3.0-only

use rfd::FileDialog;
use slint::{self, SharedString};
use std::sync::atomic::{AtomicUsize, Ordering};

mod crypto;
use crypto::{decrypt, encrypt, setup_age};

slint::include_modules!();
static ERRORNUM: AtomicUsize = AtomicUsize::new(0);

fn main() {
    if !setup_age() {
        panic!("'age' is not installed on your system")
    }
    let ui = Main::new().expect("Failed to initalize GUI");

    // setup encrypt callback
    let ui_weak = ui.as_weak();
    ui.on_encrypt_button(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let input = ui.get_input();
            let pub_file = ui.get_pub_file();

            match encrypt(&input, &pub_file) {
                Ok(v) => {
                    ui.set_input(v.into());
                    ui.set_error("".into());
                }
                Err(v) => {
                    ERRORNUM.fetch_add(1, Ordering::SeqCst);
                    let error_num = ERRORNUM.load(Ordering::SeqCst);
                    ui.set_error(format!("[{}]: {}", error_num, v).into());
                }
            }
        }
    });

    // setup decrypt callback
    let ui_weak = ui.as_weak();
    ui.on_decrypt_button(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let input = ui.get_input();
            let sec_file = ui.get_sec_file();

            match decrypt(&input, &sec_file) {
                Ok(v) => {
                    ui.set_input(v.into());
                    ui.set_error("".into());
                }
                Err(v) => {
                    ERRORNUM.fetch_add(1, Ordering::SeqCst);
                    let error_num = ERRORNUM.load(Ordering::SeqCst);
                    ui.set_error(format!("[{}]: {}", error_num, v).into());
                }
            }
        }
    });

    // setup request_open_seckey callback
    let ui_weak = ui.as_weak();
    ui.on_request_open_seckey(move || {
        if let Some(ui) = ui_weak.upgrade() {
            if let Some(path) = FileDialog::new().pick_file() {
                let path = SharedString::from(path.to_string_lossy().to_string());
                ui.set_sec_file(path);
            }
        }
    });

    // setup request_open_pubkey callback
    let ui_weak = ui.as_weak();
    ui.on_request_open_pubkey(move || {
        if let Some(ui) = ui_weak.upgrade() {
            if let Some(path) = FileDialog::new().pick_file() {
                let path = SharedString::from(path.to_string_lossy().to_string());
                ui.set_pub_file(path);
            }
        }
    });

    ui.run().expect("Failed to run GUI");
}
