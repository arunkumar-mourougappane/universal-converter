use std::fmt::format;

use itertools::Itertools;

use super::data_type::{self, ConverterError, DataType};

pub struct HexConverter;

impl HexConverter {
    pub fn to_hex_from_any(input_string: String, input_datatype: DataType) -> Result<String,ConverterError> {
        match input_datatype {
            DataType::Unknown => todo!(),
            DataType::Ascii => {
                let converted_data = hex::encode(input_string)
                    .as_str()
                    .chars()
                    .chunks(2)
                    .into_iter()
                    .map(|chunk| chunk.collect::<String>())
                    .collect::<Vec<_>>();

                return Ok(converted_data.join(" "));
            }
            DataType::Base64 => {
               let error_name = format!("{}", input_datatype);
               return  Err(ConverterError::ConverterParseFromError(error_name));
            },
            DataType::Hex => {
               let converted_data = hex::encode(input_string)
               .as_str()
               .chars()
               .chunks(2)
               .into_iter()
               .map(|chunk| chunk.collect::<String>())
               .collect::<Vec<_>>();

               return Ok(converted_data.join(" "));
            },
            DataType::Decimal => todo!(),
            DataType::Binary => todo!(),
            DataType::Rot13 => todo!(),
        }
    }
}
