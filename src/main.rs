use std::process::exit;

use base64::{prelude::BASE64_STANDARD, Engine};
use slint::{ComponentHandle, SharedString};


slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = match AppWindow::new() {
        Ok(app_window) => app_window,
        Err(_) => exit(-2),
    };

    let ui_handle = ui.as_weak();

    ui.on_convert_text(move |text_data| {
        let ui = ui_handle.unwrap();
        let my_data = BASE64_STANDARD.encode(text_data.as_str());
        ui.set_output_text(SharedString::from(my_data.as_str()));
    });

    ui.run()
}