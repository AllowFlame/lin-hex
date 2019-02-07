use std::error::Error;
use std::fmt;

/// Convert to hex values.
/// It is similar to into_bytes() in String. However, this trait forces creating Vec<u8>.
pub trait ToHexBytes {
    fn to_hex_bytes(&self) -> Result<Vec<u8>, HexConvertingError>;
}

impl ToHexBytes for str {
    fn to_hex_bytes(&self) -> Result<Vec<u8>, HexConvertingError> {
        let hex_string_len = self.len();
        if hex_string_len % 2 != 0 {
            return Result::Err(HexConvertingError::InvalidLength);
        }

        let hex = self.to_owned();
        let hex_array = hex.into_bytes();

        let byte_len = hex_string_len / 2;
        let mut bytes: Vec<u8> = Vec::<u8>::new();
        for index in 0..byte_len {
            let start_index = index * 2;
            let end_index = start_index + 1;

            let mut digit_string = String::new();
            digit_string.push(hex_array[start_index] as char);
            digit_string.push(hex_array[end_index] as char);
            let ascii_digits = digit_string.as_str();

            let digit = u8::from_str_radix(ascii_digits, 16).map_err(|_err| {
                HexConvertingError::InvalidCharacter
            })?;
            bytes.push(digit);
        }
        Result::Ok(bytes)
    }
}

pub trait ToHexString {
    fn to_hex_string(&self) -> Result<String, HexConvertingError>;
}

impl ToHexString for Vec<u8> {
    fn to_hex_string(&self) -> Result<String, HexConvertingError> {
        let hex_string = String::from_hex(self.as_slice());
        Ok(hex_string)
    }
}

impl ToHexString for [u8] {
    fn to_hex_string(&self) -> Result<String, HexConvertingError> {
        let hex_string = String::from_hex(self);
        Ok(hex_string)
    }
}

/// Convert from hex values.
/// This is for deserializing.
pub trait FromHex {
    fn from_hex(bytes: &[u8]) -> Self;
}

impl FromHex for String {
    fn from_hex(bytes: &[u8]) -> String {
        use std::fmt::Write;

        let mut hex_string = String::new();
        for &byte in bytes {
            write!(&mut hex_string, "{:02X}", byte).expect("Unable to write");
        }
        hex_string
    }
}

#[derive(Debug, PartialEq)]
pub enum HexConvertingError {
    InvalidCharacter,
    InvalidLength,
}

impl fmt::Display for HexConvertingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for HexConvertingError {
    fn description(&self) -> &str {
        match *self {
            HexConvertingError::InvalidCharacter => "InvalidCharacterError",
            HexConvertingError::InvalidLength => "InvalidLengthError",
        }
    }
}
