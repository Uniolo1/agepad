// SPDX-License-Identifier: GPL-3.0-only

use rfd::FileDialog;
use slint::{self, ToSharedString};
use std::env::var;
use std::process::exit;
use std::sync::atomic::{AtomicUsize, Ordering};

mod crypto;
use crypto::{decrypt, encrypt, setup_age};

slint::include_modules!();
static ERRORNUM: AtomicUsize = AtomicUsize::new(0);

macro_rules! report_error {
    ($ui:expr, $val:expr) => {{
        ERRORNUM.fetch_add(1, Ordering::SeqCst);
        let error_num = ERRORNUM.load(Ordering::SeqCst);
        $ui.set_error(format!("[{}]: {}", error_num, $val).into());
    }};
}

fn main() {
    if !setup_age() {
        // the documentation for the 'age' crate was too hard to parse for what I wanted to do
        // hence I use the system age
        eprintln!("'age' is not installed, please install it to use this program");
        eprintln!("https://github.com/FiloSottile/age");
        exit(1);
    }

    let ui = Main::new().expect("Failed to initalize GUI");

    // setup starting variables based on env variables
    if let Ok(default_secfile) = var("AGEPAD_SECFILE") {
        ui.set_sec_file(default_secfile.to_shared_string());
    }
    if let Ok(default_pubfile) = var("AGEPAD_PUBFILE") {
        ui.set_pub_file(default_pubfile.to_shared_string());
    }

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
                    report_error!(ui, v);
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
                    report_error!(ui, v);
                }
            }
        }
    });

    // setup request_open_seckey callback
    let ui_weak = ui.as_weak();
    ui.on_request_open_seckey(move || {
        if let Some(ui) = ui_weak.upgrade() {
            if let Some(path) = FileDialog::new().pick_file() {
                let path = path.to_string_lossy().to_shared_string();
                ui.set_sec_file(path);
            }
        }
    });

    // setup request_open_pubkey callback
    let ui_weak = ui.as_weak();
    ui.on_request_open_pubkey(move || {
        if let Some(ui) = ui_weak.upgrade() {
            if let Some(path) = FileDialog::new().pick_file() {
                let path = path.to_string_lossy().to_shared_string();
                ui.set_pub_file(path);
            }
        }
    });

    // set about button callback
    let ui_weak = ui.as_weak();
    ui.on_about_button(move || {
        if let Some(ui) = ui_weak.upgrade() {
            match PopupWindow::new() {
                Ok(v) => {
                    let run = v.run();
                    if let Err(v) = run {
                        report_error!(ui, v);
                    }
                }
                Err(v) => {
                    report_error!(ui, v);
                }
            };
        }
    });

    ui.run().expect("Failed to run GUI");
}
