use std::{process::exit, str::FromStr};

use ascii_converter::{string_to_binary, string_to_decimals};
use slint::{ComponentHandle, SharedString};
use universal_converter::converter_api::{
    base64_converter::Base64Converter, data_type::DataType, hex_connverter::HexConverter,
};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = match AppWindow::new() {
        Ok(app_window) => app_window,
        Err(_) => exit(-2),
    };

    let ui_handle = ui.as_weak();

    ui.on_convert_text(move |text_data, input_type, output_type| {
        let ui = ui_handle.unwrap();
        ui.set_info_text(SharedString::from(""));
        let input_data_type = match DataType::from_str(&input_type) {
            Ok(input_data_type) => input_data_type,
            Err(_) => {
                ui.set_info_text(SharedString::from("Cannot get input data's type"));
                return;
            },
        };
        let output_data_type = DataType::from_str(&output_type).unwrap();
        match output_data_type {
            DataType::Unknown => {
                ui.set_info_text(SharedString::from("Error: Bad Output Data Type!"))
            }
            DataType::Ascii => todo!(),
            DataType::Base64 => {
                ui.set_output_text(SharedString::from(Base64Converter::to_base64_from_any(
                    text_data.to_string(),
                    input_data_type,
                )));
            }
            DataType::Hex => {
                let hex_ouput = HexConverter::to_hex_from_any(
                    text_data.to_string(),
                    input_data_type,
                );
                match hex_ouput {
                    Ok(hex_ouput) => {
                        ui.set_output_text(SharedString::from(hex_ouput));
                    },
                    Err(error) => {
                        ui.set_info_text(SharedString::from(error.to_string()))
                    },
                }
            }
            DataType::Decimal => {
                match string_to_decimals(&text_data) {
                    Ok(decimal_data) => {
                        let data_string = decimal_data.iter()
                        .map(|num| num.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");

                        ui.set_output_text(SharedString::from(data_string));
                    },
                    Err(_) => ui.set_info_text(SharedString::from(SharedString::from("Error: Invalid Input"))),
                };
            },
            DataType::Binary => {
                match string_to_binary(&text_data) {
                    Ok(decimal_data) => {
                        let data_string = decimal_data.iter()
                        .map(|num| num.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");

                        ui.set_output_text(SharedString::from(data_string));
                    },
                    Err(_) => ui.set_info_text(SharedString::from(SharedString::from("Error: Invalid Input"))),
                };
            },
            DataType::Rot13 => {
                let mut result = String::new();
                for char in text_data.chars() {
                    if char.is_alphabetic() {
                        let is_uppercase = char.is_uppercase();
                        let base = if is_uppercase { 'A' as u8 } else { 'a' as u8 };
                        let shifted_char = ((char as u8 - base + 13) % 26 + base) as char;
                        result.push(shifted_char);
                    } else {
                        result.push(char);
                    }
                }
                ui.set_output_text(SharedString::from(result));
            }
        }
    });

    ui.run()
}
