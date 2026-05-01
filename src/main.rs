// SPDX-License-Identifier: GPL-3.0-only

use rfd::FileDialog;
use slint::{self, SharedString};
use std::env;
use std::process::exit;
use std::sync::atomic::{AtomicUsize, Ordering};
use webbrowser;

mod crypto;
use crypto::{decrypt, encrypt, setup_age};

slint::include_modules!();
static ERRORNUM: AtomicUsize = AtomicUsize::new(0);

// #region: macros
macro_rules! url_open {
    ($url:expr) => {{
        let url_ref = $url;
        println!("Opening {}", &url_ref);
        webbrowser::open(&url_ref) // NOTE: returned
    }};
}

macro_rules! report_error {
    ($ui:expr, $val:expr) => {{
        ERRORNUM.fetch_add(1, Ordering::SeqCst);
        let error_num = ERRORNUM.load(Ordering::SeqCst);
        $ui.set_error(format!("[{}]: {}", error_num, $val).into());
    }};
}
// #endregion

fn install_gui() {
    let ui = GetAge::new().expect("Failed to initalize GUI");

    ui.on_open_url(|url| {
        url_open!(url).expect("Failed to open URL");
    });
    ui.on_quit(|| exit(1)); // tried having it return, did not work

    ui.run().expect("Failed to run GUI");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // handle --get-age
    let trigger_get_age = args
        .iter()
        .any(|arg| matches!(arg.as_str(), "-ga" | "--get-age"));

    // prompt to install 'age' GUI
    if !setup_age() || trigger_get_age {
        install_gui();
        exit(1);
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
                Err(v) => report_error!(ui, v),
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
                Err(v) => report_error!(ui, v),
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

    // setup open_url callback
    let ui_weak = ui.as_weak();
    ui.on_open_url(move |url| {
        if let Some(ui) = ui_weak.upgrade() {
            if let Err(e) = url_open!(url) {
                report_error!(ui, e);
            };
        }
    });

    ui.run().expect("Failed to run GUI");
}
