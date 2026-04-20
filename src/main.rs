use slint;
slint::include_modules!();

mod crypto;
use crypto::{decrypt, encrypt};

fn main() {
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
                Err(v) => ui.set_error(v.into()),
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
                Err(v) => ui.set_error(v.into()),
            }
        }
    });

    ui.run().expect("Failed to run GUI");
}
