use maplit::hashmap;
use serde::ser::Serialize;
use serde_rison::ser::to_string;
use std::collections::HashMap;

fn ok<T: ?Sized + Serialize>(value: &T) -> String {
    to_string(value).unwrap()
}

#[test]
fn test_ser_unit() {
    assert_eq!(ok(&()).as_str(), "!n");
}

#[test]
fn test_ser_bool() {
    assert_eq!(ok(&true).as_str(), "!t");
    assert_eq!(ok(&false).as_str(), "!f");
}

#[test]
fn test_ser_integer() {
    assert_eq!(ok(&i8::MIN), i8::MIN.to_string());
    assert_eq!(ok(&i8::MAX), i8::MAX.to_string());
    assert_eq!(ok(&i16::MIN), i16::MIN.to_string());
    assert_eq!(ok(&i16::MAX), i16::MAX.to_string());
    assert_eq!(ok(&i32::MIN), i32::MIN.to_string());
    assert_eq!(ok(&i32::MAX), i32::MAX.to_string());
    assert_eq!(ok(&i64::MIN), i64::MIN.to_string());
    assert_eq!(ok(&i64::MAX), i64::MAX.to_string());
    assert_eq!(ok(&i128::MIN), i128::MIN.to_string());
    assert_eq!(ok(&i128::MAX), i128::MAX.to_string());
    assert_eq!(ok(&u8::MIN), u8::MIN.to_string());
    assert_eq!(ok(&u8::MAX), u8::MAX.to_string());
    assert_eq!(ok(&u16::MIN), u16::MIN.to_string());
    assert_eq!(ok(&u16::MAX), u16::MAX.to_string());
    assert_eq!(ok(&u32::MIN), u32::MIN.to_string());
    assert_eq!(ok(&u32::MAX), u32::MAX.to_string());
    assert_eq!(ok(&u64::MIN), u64::MIN.to_string());
    assert_eq!(ok(&u64::MAX), u64::MAX.to_string());
    assert_eq!(ok(&u128::MIN), u128::MIN.to_string());
    assert_eq!(ok(&u128::MAX), u128::MAX.to_string());
}

#[test]
fn test_ser_float() {
    assert_eq!(ok(&1.0_f32).as_str(), "1.0");
    assert_eq!(ok(&1.0_f64).as_str(), "1.0");

    assert_eq!(ok(&f32::MAX).as_str(), "3.4028235e38");
    assert_eq!(ok(&f32::MIN).as_str(), "-3.4028235e38");
    assert_eq!(ok(&f32::EPSILON).as_str(), "1.1920929e-7");
    assert_eq!(ok(&f32::INFINITY).as_str(), "!n");
    assert_eq!(ok(&f32::NEG_INFINITY).as_str(), "!n");
    assert_eq!(ok(&f32::NAN).as_str(), "!n");

    assert_eq!(ok(&f64::MAX).as_str(), "1.7976931348623157e308");
    assert_eq!(ok(&f64::MIN).as_str(), "-1.7976931348623157e308");
    assert_eq!(ok(&f64::EPSILON).as_str(), "2.220446049250313e-16");
    assert_eq!(ok(&f64::INFINITY).as_str(), "!n");
    assert_eq!(ok(&f64::NEG_INFINITY).as_str(), "!n");
    assert_eq!(ok(&f64::NAN).as_str(), "!n");
}

#[test]
fn test_ser_str() {
    assert_eq!(ok(&'a').as_str(), "a");
    assert_eq!(ok(&' ').as_str(), "' '");
    assert_eq!(ok(&'!').as_str(), "'!!'");
    assert_eq!(ok(&'\'').as_str(), "'!''");
    assert_eq!(ok(&'\n').as_str(), "\n");

    assert_eq!(ok("").as_str(), "''");
    assert_eq!(ok("1").as_str(), "'1'");
    assert_eq!(ok("a").as_str(), "a");
    assert_eq!(ok("abc").as_str(), "abc");
    assert_eq!(ok("あ").as_str(), "あ");
    assert_eq!(ok("I'm not a JSON!").as_str(), "'I!'m not a JSON!!'");
    assert_eq!(ok("_").as_str(), "_");
    assert_eq!(ok("\t").as_str(), "\t");
    assert_eq!(ok(" ").as_str(), "' '");
}

#[test]
fn test_seq() {
    assert_eq!(ok::<[i32]>(&[]).as_str(), "!()");
    assert_eq!(ok(&[1]).as_str(), "!(1)");
    assert_eq!(ok(&[1, 2]).as_str(), "!(1,2)");
    assert_eq!(ok(&[&b"ab"[..], &b""[..]]).as_str(), "!(!(97,98),!())");
    assert_eq!(ok(&vec![Some(1), None]).as_str(), "!(1,!n)");
    assert_eq!(
        ok(&vec![Some(Some("a!")), Some(None), None]).as_str(),
        "!('a!!',!n,!n)"
    );
}

#[test]
fn test_map() {
    assert_eq!(ok(&HashMap::<String, String>::new()).as_str(), "()");
    assert_eq!(
        ok(&hashmap! {
            "I'm a key!" => "I'm a value!",
        })
        .as_str(),
        "('I!'m a key!!':'I!'m a value!!')"
    );
    assert_eq!(
        ok(&hashmap! {
            "b" => 1,
            "a" => 2,
        })
        .as_str(),
        "(a:2,b:1)"
    );
    assert_eq!(
        ok(&hashmap! {
            1_u64 => 10_u128,
            2_u64 => 20_u128,
            3_u64 => 30_u128,
        })
        .as_str(),
        "(1:10,2:20,3:30)"
    );
    assert_eq!(
        ok(&hashmap! {
            "key1" => hashmap! {
                1 => vec![
                    hashmap! {
                        "a" => "A",
                    },
                ],
            },
            "key2" => hashmap! {},
        })
        .as_str(),
        "(key1:(1:!((a:A))),key2:())"
    );
    assert_eq!(
        ok(&hashmap! {
            "B" => None,
            "A" => Some("a"),
        })
        .as_str(),
        "(A:a,B:!n)"
    )
}
