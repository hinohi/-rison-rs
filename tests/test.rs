use serde_rison::ser::to_string;

#[test]
fn test_ser_unit() {
    assert_eq!(to_string(&()).as_str(), "!n");
}

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

#[test]
fn test_ser_float() {
    assert_eq!(to_string(&1.0_f32).as_str(), "1.0");
    assert_eq!(to_string(&1.0_f64).as_str(), "1.0");

    assert_eq!(to_string(&f32::MAX).as_str(), "3.4028235e38");
    assert_eq!(to_string(&f32::MIN).as_str(), "-3.4028235e38");
    assert_eq!(to_string(&f32::EPSILON).as_str(), "1.1920929e-7");
    assert_eq!(to_string(&f32::INFINITY).as_str(), "!n");
    assert_eq!(to_string(&f32::NEG_INFINITY).as_str(), "!n");
    assert_eq!(to_string(&f32::NAN).as_str(), "!n");

    assert_eq!(to_string(&f64::MAX).as_str(), "1.7976931348623157e308");
    assert_eq!(to_string(&f64::MIN).as_str(), "-1.7976931348623157e308");
    assert_eq!(to_string(&f64::EPSILON).as_str(), "2.220446049250313e-16");
    assert_eq!(to_string(&f64::INFINITY).as_str(), "!n");
    assert_eq!(to_string(&f64::NEG_INFINITY).as_str(), "!n");
    assert_eq!(to_string(&f64::NAN).as_str(), "!n");
}

#[test]
fn test_ser_str() {
    assert_eq!(to_string(&'a').as_str(), "a");
    assert_eq!(to_string(&' ').as_str(), "' '");
    assert_eq!(to_string(&'!').as_str(), "'!!'");
    assert_eq!(to_string(&'\'').as_str(), "'!''");
    assert_eq!(to_string(&'\n').as_str(), "\n");

    assert_eq!(to_string("").as_str(), "''");
    assert_eq!(to_string("1").as_str(), "'1'");
    assert_eq!(to_string("a").as_str(), "a");
    assert_eq!(to_string("abc").as_str(), "abc");
    assert_eq!(to_string("あ").as_str(), "あ");
    assert_eq!(to_string("I'm not a JSON!").as_str(), "'I!'m not a JSON!!'");
    assert_eq!(to_string("_").as_str(), "_");
    assert_eq!(to_string("\t").as_str(), "\t");
    assert_eq!(to_string(" ").as_str(), "' '");
}

#[test]
fn test_seq() {
    assert_eq!(to_string::<[i32]>(&[]).as_str(), "!()");
    assert_eq!(to_string(&[1]).as_str(), "!(1)");
    assert_eq!(to_string(&[1, 2]).as_str(), "!(1,2)");
    assert_eq!(
        to_string(&[&b"ab"[..], &b""[..]]).as_str(),
        "!(!(97,98),!())"
    );
}
