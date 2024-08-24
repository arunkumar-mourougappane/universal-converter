use base64::{prelude::BASE64_STANDARD, Engine};

use super::data_type::DataType;


pub struct Base64Converter;


impl Base64Converter {
   pub fn to_base64_from_any(input_string: String, input_datatype: DataType) -> String {
      return BASE64_STANDARD.encode(input_string.as_str());
   }
}