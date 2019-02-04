/// Convert to ascii code. 1 byte will be expressed as 2 bytes ascii code.
/// For example, 0x31 will be converted "49".
pub trait ToAscii {
    fn to_ascii(&self, fill_digits: usize) -> Vec<u8>;
}

impl ToAscii for u8 {
    fn to_ascii(&self, fill_digits: usize) -> Vec<u8> {
        let num_string = self.to_string();
        let num_len = num_string.len();
        let len_diff = if fill_digits < num_len {
            0
        } else {
            fill_digits - num_len
        };

        let mut converted = Vec::new();
        for _ in 0..len_diff {
            let zero = '0' as u8;
            converted.push(zero);
        }

        let mut num_vec = num_string.into_bytes();
        converted.append(&mut num_vec);
        converted
    }
}

impl ToAscii for Vec<u8> {
    fn to_ascii(&self, fill_digits: usize) -> Vec<u8> {
        let mut ascii_converted = Vec::new();
        for num in self {
            let mut ascii = num.to_ascii(fill_digits);
            ascii_converted.append(&mut ascii);
        }
        ascii_converted
    }
}
