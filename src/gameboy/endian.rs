pub fn u16_to_big_endian(val: u16) -> [u8; 2] {
    let mut result = [0, 0];

    result[0] = (val >> 8) as u8;
    result[1] = val as u8;

    result
}

pub fn be_to_u16(vals: &[u8]) -> u16 {
    let mut result;
    result = vals[1] as u16;
    result += (vals[0] as u16) << 8;
    result
}

pub fn le_to_u16(vals: &[u8]) -> u16 {
    let mut result;
    result = vals[0] as u16;
    result += (vals[1] as u16) << 8;
    result
}


#[test]
fn u16_test() {
    assert_eq!(u16_to_big_endian(0xffff), [0xff, 0xff]);
    assert_eq!(u16_to_big_endian(0x0000), [0x00, 0x00]);
    assert_eq!(u16_to_big_endian(0x00ff), [0x00, 0xff]);
    assert_eq!(u16_to_big_endian(0xff00), [0xff, 0x00]);
    assert_eq!(u16_to_big_endian(0x1234), [0x12, 0x34]);
}

#[test]
fn big_endian_test() {
    assert_eq!(be_to_u16(&[0xff, 0xff]), 0xffff);
    assert_eq!(be_to_u16(&[0x00, 0x00]), 0x0000);
    assert_eq!(be_to_u16(&[0x00, 0xff]), 0x00ff);
    assert_eq!(be_to_u16(&[0xff, 0x00]), 0xff00);
    assert_eq!(be_to_u16(&[0x12, 0x34]), 0x1234);
}


#[test]
fn little_endian_test() {
    assert_eq!(le_to_u16(&[0xff, 0xff]), 0xffff);
    assert_eq!(le_to_u16(&[0x00, 0x00]), 0x0000);
    assert_eq!(le_to_u16(&[0x00, 0xff]), 0xff00);
    assert_eq!(le_to_u16(&[0xff, 0x00]), 0x00ff);
    assert_eq!(le_to_u16(&[0x12, 0x34]), 0x3412);
}