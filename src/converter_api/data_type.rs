use core::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConverterError {
    #[error("Cannot parse the string to a data type")]
    DataTypeParseError,
    #[error("Cannot parse from data type {0}")]
    ConverterParseFromError(String),
    #[error("Cannot parse to data type {0}")]
    ConverterParseToError(String),
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Debug)]
pub enum DataType {
    Unknown,
    Ascii,
    Base64,
    Hex,
    Decimal,
    Binary,
    Rot13,
}

impl FromStr for DataType {
    type Err = ConverterError;

    fn from_str(data_type: &str) -> Result<Self, Self::Err> {
        match data_type {
            "ascii" => Ok(Self::Ascii),
            "base64" => Ok(Self::Base64),
            "Hex" => Ok(Self::Hex),
            "Decimal" => Ok(Self::Decimal),
            "Binary" => Ok(Self::Binary),
            "ROT13" => Ok(Self::Rot13),
            _ => Err(ConverterError::DataTypeParseError),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Unknown => write!(f, "Unknown"),
            DataType::Ascii => write!(f, "ASCII"),
            DataType::Base64 => write!(f, "base64"),
            DataType::Hex => write!(f, "Hex"),
            DataType::Decimal => write!(f, "Decimal"),
            DataType::Binary => write!(f, "Binary"),
            DataType::Rot13 => write!(f, "ROT13"),
        }
    }
}


