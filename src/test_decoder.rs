use decoder::{decode, MAX_ARRAY_SIZE};
use std::collections::BTreeMap;
use {CborError, CborType};

// First test all the basic types
fn test_decoder(bytes: Vec<u8>, expected: CborType) {
    let result = decode(&bytes);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

fn test_decoder_error(bytes: Vec<u8>, expected_error: CborError) {
    let result = decode(&bytes);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), expected_error);
}

fn test_integer(bytes: Vec<u8>, expected: u64) {
    let decoded = decode(&bytes).unwrap();
    match decoded {
        CborType::Integer(val) => assert_eq!(val, expected),
        _ => assert_eq!(1, 0),
    }
}

fn test_integer_all(bytes: Vec<u8>, expected_value: u64) {
    let expected = CborType::Integer(expected_value);
    test_decoder(bytes.clone(), expected);
    test_integer(bytes, expected_value);
}

#[test]
fn test_integer_objects() {
    let bytes: Vec<u8> = vec![0x00];
    test_integer_all(bytes, 0);

    let bytes = vec![0x01];
    test_integer_all(bytes, 1);

    let bytes = vec![0x0A];
    test_integer_all(bytes, 10);

    let bytes = vec![0x17];
    test_integer_all(bytes, 23);

    let bytes = vec![0x18, 0x18];
    test_integer_all(bytes, 24);

    let bytes = vec![0x18, 0x19];
    test_integer_all(bytes, 25);

    let bytes = vec![0x18, 0x64];
    test_integer_all(bytes, 100);

    let bytes = vec![0x19, 0x03, 0xe8];
    test_integer_all(bytes, 1000);

    let bytes = vec![0x1a, 0x00, 0x0f, 0x42, 0x40];
    test_integer_all(bytes, 1000000);

    let bytes = vec![0x1b, 0x00, 0x00, 0x00, 0xe8, 0xd4, 0xa5, 0x10, 0x00];
    test_integer_all(bytes, 1000000000000);

    let bytes = vec![0x1b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    test_integer_all(bytes, 18446744073709551615);
}

#[cfg(test)]
fn test_tag(bytes: Vec<u8>, expected_tag: u64, expected_value: CborType) {
    let decoded = decode(&bytes).unwrap();
    match decoded {
        CborType::Tag(tag, value) => {
            assert_eq!(expected_tag, tag);
            assert_eq!(expected_value, *value);
        }
        _ => assert_eq!(1, 0),
    }
}

#[test]
fn test_tagged_objects() {
    let bytes: Vec<u8> = vec![0xD2, 0x02];
    let expected_tag_value = 0x12;
    let expected_value = CborType::Integer(2);
    let expected = CborType::Tag(expected_tag_value, Box::new(expected_value.clone()));
    test_decoder(bytes.clone(), expected);
    test_tag(bytes, expected_tag_value, expected_value);
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_arrays() {
    // []
    let bytes: Vec<u8> = vec![0x80];
    let expected = CborType::Array(vec![]);
    test_decoder(bytes, expected);

    // [1, 2, 3]
    let bytes: Vec<u8> = vec![0x83, 0x01, 0x02, 0x03];
    let tmp = vec![
        CborType::Integer(1),
        CborType::Integer(2),
        CborType::Integer(3),
    ];
    let expected = CborType::Array(tmp);
    test_decoder(bytes, expected);

    // [1, [2, 3], [4, 5]]
    let bytes: Vec<u8> = vec![0x83, 0x01, 0x82, 0x02, 0x03, 0x82, 0x04, 0x05];
    let tmp1 = vec![CborType::Integer(2), CborType::Integer(3)];
    let tmp2 = vec![CborType::Integer(4), CborType::Integer(5)];
    let tmp = vec![
        CborType::Integer(1),
        CborType::Array(tmp1),
        CborType::Array(tmp2),
    ];
    let expected = CborType::Array(tmp);
    test_decoder(bytes, expected);

    // [1, [[[[1]]]], [1]]
    let bytes: Vec<u8> = vec![0x83, 0x01, 0x81, 0x81, 0x81, 0x81, 0x01, 0x81, 0x02];
    let tmp = vec![
        CborType::Integer(1),
        CborType::Array(vec![
            CborType::Array(vec![
                CborType::Array(vec![
                    CborType::Array(vec![
                        CborType::Integer(1)])])])]),
        CborType::Array(vec![CborType::Integer(2)]),
    ];
    let expected = CborType::Array(tmp);
    test_decoder(bytes, expected);

    let bytes: Vec<u8> = vec![0x98, 0x1A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                              0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                              0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
                              0x17, 0x18, 0x18, 0x18, 0x19, 0x82, 0x81, 0x81,
                              0x81, 0x05, 0x81, 0x1A, 0x49, 0x96, 0x02, 0xD2];
    // [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    //  21, 22, 23, 24, 25, [[[[5]]], [1234567890]]]
    let tmp = vec![
        CborType::Integer(1),
        CborType::Integer(2),
        CborType::Integer(3),
        CborType::Integer(4),
        CborType::Integer(5),
        CborType::Integer(6),
        CborType::Integer(7),
        CborType::Integer(8),
        CborType::Integer(9),
        CborType::Integer(10),
        CborType::Integer(11),
        CborType::Integer(12),
        CborType::Integer(13),
        CborType::Integer(14),
        CborType::Integer(15),
        CborType::Integer(16),
        CborType::Integer(17),
        CborType::Integer(18),
        CborType::Integer(19),
        CborType::Integer(20),
        CborType::Integer(21),
        CborType::Integer(22),
        CborType::Integer(23),
        CborType::Integer(24),
        CborType::Integer(25),
        CborType::Array(vec![
            CborType::Array(vec![
                CborType::Array(vec![
                    CborType::Array(vec![
                        CborType::Integer(5)])])]),
            CborType::Array(vec![CborType::Integer(1234567890)])])
    ];
    let expected = CborType::Array(tmp);
    test_decoder(bytes, expected);
}

#[test]
fn test_signed_integer() {
    let bytes: Vec<u8> = vec![0x20];
    let expected = CborType::SignedInteger(-1);
    test_decoder(bytes, expected);

    let bytes = vec![0x29];
    let expected = CborType::SignedInteger(-10);
    test_decoder(bytes, expected);

    let bytes = vec![0x38, 0x63];
    let expected = CborType::SignedInteger(-100);
    test_decoder(bytes, expected);

    let bytes = vec![0x39, 0x03, 0xe7];
    let expected = CborType::SignedInteger(-1000);
    test_decoder(bytes, expected);

    let bytes = vec![0x39, 0x27, 0x0F];
    let expected = CborType::SignedInteger(-10000);
    test_decoder(bytes, expected);

    let bytes = vec![0x3A, 0x00, 0x01, 0x86, 0x9F];
    let expected = CborType::SignedInteger(-100000);
    test_decoder(bytes, expected);

    let bytes = vec![0x3B, 0x00, 0x00, 0x00, 0xE8, 0xD4, 0xA5, 0x0F, 0xFF];
    let expected = CborType::SignedInteger(-1000000000000);
    test_decoder(bytes, expected);
}

#[test]
fn test_byte_strings() {
    let bytes: Vec<u8> = vec![0x40];
    let expected = CborType::Bytes(vec![]);
    test_decoder(bytes, expected);

    // 01020304
    let bytes: Vec<u8> = vec![0x44, 0x01, 0x02, 0x03, 0x04];
    let expected = CborType::Bytes(vec![0x01, 0x02, 0x03, 0x04]);
    test_decoder(bytes, expected);

    // 0102030405060708090A0B0C0D0E0F10203040506070
    let bytes: Vec<u8> = vec![
        0x56, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70,
    ];
    let expected = CborType::Bytes(vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70,
    ]);
    test_decoder(bytes, expected);

    let bytes: Vec<u8> = vec![
        0x59, 0x01, 0x0E, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF,
    ];
    let expected = CborType::Bytes(vec![
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ]);
    test_decoder(bytes, expected);
}

#[test]
fn test_maps() {
    // {}
    let bytes: Vec<u8> = vec![0xa0];
    let expected: BTreeMap<CborType, CborType> = BTreeMap::new();
    test_decoder(bytes, CborType::Map(expected));

    // {1: 2, 3: 4}
    let bytes: Vec<u8> = vec![0xa2, 0x01, 0x02, 0x03, 0x04];
    let mut expected: BTreeMap<CborType, CborType> = BTreeMap::new();
    expected.insert(CborType::Integer(1), CborType::Integer(2));
    expected.insert(CborType::Integer(3), CborType::Integer(4));
    test_decoder(bytes, CborType::Map(expected));

    // TODO: strings aren't properly supported as keys yet.
    // {"a": 1, "b": [2, 3]}
    // let bytes: Vec<u8> = vec![0xa2, 0x61, 0x61, 0x01, 0x61, 0x62, 0x82, 0x02, 0x03];
    // let expected =
    //     CborType::Map(vec![
    //         CborMap{key: CborType::Integer(1), value: CborType::Integer(2)},
    //         CborMap{key: CborType::Integer(3), value: CborType::Integer(4)}]);
    // test_decoder(bytes, expected);

    // let bytes: Vec<u8> = vec![0x82, 0x61, 0x61, 0xa1, 0x61, 0x62, 0x61, 0x63];
    // test_decoder(bytes, "[a, {b: c}]");

    // let bytes: Vec<u8> = vec![0xa5, 0x61, 0x61, 0x61, 0x41, 0x61, 0x62, 0x61,
    //                           0x42, 0x61, 0x63, 0x61, 0x43, 0x61, 0x64, 0x61,
    //                           0x44, 0x61, 0x65, 0x61, 0x45];
    // test_decoder(bytes, "{a: A, b: B, c: C, d: D, e: E}");
}

#[test]
fn test_map_duplicate_keys() {
    let bytes: Vec<u8> = vec![0xa4, 0x01, 0x02, 0x02, 0x03, 0x01, 0x03, 0x04, 0x04];
    test_decoder_error(bytes, CborError::DuplicateMapKey);
}

#[test]
fn test_tag_with_no_value() {
    let bytes: Vec<u8> = vec![0xc0];
    test_decoder_error(bytes, CborError::TruncatedInput);
}

#[test]
fn test_truncated_int() {
    let bytes: Vec<u8> = vec![0x19, 0x03];
    test_decoder_error(bytes, CborError::TruncatedInput);
}

#[test]
fn test_truncated_array() {
    let bytes: Vec<u8> = vec![0x83, 0x01, 0x02];
    test_decoder_error(bytes, CborError::TruncatedInput);
}

#[test]
fn test_truncated_map() {
    let bytes: Vec<u8> = vec![0xa2, 0x01, 0x02, 0x00];
    test_decoder_error(bytes, CborError::TruncatedInput);
}

#[test]
fn test_malformed_integer() {
    let bytes: Vec<u8> = vec![0x1c];
    test_decoder_error(bytes, CborError::MalformedInput);
}

#[test]
fn test_signed_integer_too_large() {
    let bytes = vec![0x3b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    test_decoder_error(bytes, CborError::InputValueOutOfRange);
}

#[test]
fn test_null() {
    let bytes = vec![0xf6];
    test_decoder(bytes, CborType::Null);
}

#[test]
fn test_null_in_array() {
    let bytes = vec![0x82, 0xf6, 0xf6];
    test_decoder(bytes, CborType::Array(vec![CborType::Null, CborType::Null]));
}

#[test]
fn test_major_type_7() {
    for i in 0..0x20 {
        if i != 22 {
            let bytes = vec![0xe0 | i];
            test_decoder_error(bytes, CborError::UnsupportedType);
        }
    }
}

#[test]
fn test_large_input() {
    let len = MAX_ARRAY_SIZE;
    let array = vec![0xFF; len];
    let expected = CborType::Bytes(array.clone());
    let mut bytes = vec![0x5A];
    bytes.extend_from_slice(&(len as u32).to_be_bytes());
    bytes.extend_from_slice(&array);
    test_decoder(bytes, expected);
}

#[test]
fn test_too_large_input() {
    let len = MAX_ARRAY_SIZE + 1;
    let array = vec![0xFF; len];
    let mut bytes = vec![0x5A];
    bytes.extend_from_slice(&(len as u32).to_be_bytes());
    bytes.extend_from_slice(&array);
    test_decoder_error(bytes, CborError::InputTooLarge);
}

// We currently don't support CBOR strings (issue #39).
#[test]
fn test_invalid_input() {
    let bytes = vec![0x60];
    test_decoder_error(bytes, CborError::UnsupportedType);
}

#[test]
fn test_avoid_stack_exhaustion_with_arrays() {
    let mut bytes: Vec<u8> = Vec::new();
    // Create a payload representing Array(Array(Array(Array(...(Array(0))))))
    // If the implementation is not careful, this will exhaust the stack.
    for _ in 1..10000 {
        bytes.push(0b1000_0001);
    }
    bytes.push(0);
    test_decoder_error(bytes, CborError::MalformedInput);
}

#[test]
fn test_avoid_stack_exhaustion_with_maps_1() {
    let mut bytes: Vec<u8> = Vec::new();
    // Create a payload representing Map(0: Map(0: Map(0: Map(...Map()))))
    // If the implementation is not careful, this will exhaust the stack.
    for _ in 1..10000 {
        bytes.push(0b1010_0001);
        bytes.push(0);
    }
    bytes.push(0b1010_0000);
    test_decoder_error(bytes, CborError::MalformedInput);
}

#[test]
fn test_avoid_stack_exhaustion_with_maps_2() {
    let mut bytes: Vec<u8> = Vec::new();
    // Create a payload representing Map(Map(Map(...(Map(): 0): 0): 0): 0)
    // If the implementation is not careful, this will exhaust the stack.
    for _ in 1..10000 {
        bytes.push(0b1010_0001);
    }
    bytes.push(0b1010_0000);
    for _ in 1..9999 {
        bytes.push(0);
    }
    test_decoder_error(bytes, CborError::MalformedInput);
}

#[test]
fn test_avoid_stack_exhaustion_with_tags() {
    let mut bytes: Vec<u8> = Vec::new();
    // Create a payload representing Tag(6: Tag(6: Tag(6: Tag(...Tag(0)))))
    // If the implementation is not careful, this will exhaust the stack.
    for _ in 1..10000 {
        bytes.push(0b1100_0110);
    }
    bytes.push(0);
    test_decoder_error(bytes, CborError::MalformedInput);
}
