use crate::hex::{ToHexBytes};

#[test]
fn to_hex_test() {
    let hex_string = "1b5f0d";
    let hex = hex_string.to_hex_bytes().unwrap();

    let hex_vec = vec![0x1b, 0x5f, 0x0d];
    assert_eq!(hex, hex_vec);
}