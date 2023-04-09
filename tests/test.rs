use serde::ser::{self, Serializer};

use serde_rison::ser::to_string;

#[test]
fn test_ser_bool() {
    assert_eq!(to_string(&true).as_str(), "!t");
    assert_eq!(to_string(&false).as_str(), "!f");
}

#[test]
fn test_ser_integer() {
    assert_eq!(to_string(&i8::MIN), i8::MIN.to_string());
    assert_eq!(to_string(&i8::MAX), i8::MAX.to_string());
    assert_eq!(to_string(&i16::MIN), i16::MIN.to_string());
    assert_eq!(to_string(&i16::MAX), i16::MAX.to_string());
    assert_eq!(to_string(&i32::MIN), i32::MIN.to_string());
    assert_eq!(to_string(&i32::MAX), i32::MAX.to_string());
    assert_eq!(to_string(&i64::MIN), i64::MIN.to_string());
    assert_eq!(to_string(&i64::MAX), i64::MAX.to_string());
    assert_eq!(to_string(&i128::MIN), i128::MIN.to_string());
    assert_eq!(to_string(&i128::MAX), i128::MAX.to_string());
    assert_eq!(to_string(&u8::MIN), u8::MIN.to_string());
    assert_eq!(to_string(&u8::MAX), u8::MAX.to_string());
    assert_eq!(to_string(&u16::MIN), u16::MIN.to_string());
    assert_eq!(to_string(&u16::MAX), u16::MAX.to_string());
    assert_eq!(to_string(&u32::MIN), u32::MIN.to_string());
    assert_eq!(to_string(&u32::MAX), u32::MAX.to_string());
    assert_eq!(to_string(&u64::MIN), u64::MIN.to_string());
    assert_eq!(to_string(&u64::MAX), u64::MAX.to_string());
    assert_eq!(to_string(&u128::MIN), u128::MIN.to_string());
    assert_eq!(to_string(&u128::MAX), u128::MAX.to_string());
}
