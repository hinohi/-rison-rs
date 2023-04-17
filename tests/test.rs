use maplit::{btreemap, hashmap};
use ordered_float::OrderedFloat;
use serde::{ser, Serialize};
use serde_bytes::{ByteBuf, Bytes};
use serde_rison::{de::from_str, ser::to_string};
use std::collections::HashMap;

fn ok<T: ?Sized + ser::Serialize>(value: &T) -> String {
    to_string(value).unwrap()
}

#[test]
fn test_ser_unit() {
    assert_eq!(ok(&()), "!n");
}

#[test]
fn test_bool() {
    assert_eq!(ok(&true), "!t");
    assert_eq!(ok(&false), "!f");
    let b: bool = from_str("!t").unwrap();
    assert_eq!(b, true);
    let b: bool = from_str("!f").unwrap();
    assert_eq!(b, false);
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
    assert_eq!(ok(&1.0_f32), "1.0");
    assert_eq!(ok(&1.0_f64), "1.0");

    assert_eq!(ok(&f32::MAX), "3.4028235e38");
    assert_eq!(ok(&f32::MIN), "-3.4028235e38");
    assert_eq!(ok(&f32::EPSILON), "1.1920929e-7");
    assert_eq!(ok(&f32::INFINITY), "!n");
    assert_eq!(ok(&f32::NEG_INFINITY), "!n");
    assert_eq!(ok(&f32::NAN), "!n");

    assert_eq!(ok(&f64::MAX), "1.7976931348623157e308");
    assert_eq!(ok(&f64::MIN), "-1.7976931348623157e308");
    assert_eq!(ok(&f64::EPSILON), "2.220446049250313e-16");
    assert_eq!(ok(&f64::INFINITY), "!n");
    assert_eq!(ok(&f64::NEG_INFINITY), "!n");
    assert_eq!(ok(&f64::NAN), "!n");
}

#[test]
fn test_ser_str() {
    assert_eq!(ok(&'a'), "a");
    assert_eq!(ok(&' '), "' '");
    assert_eq!(ok(&'!'), "'!!'");
    assert_eq!(ok(&'\''), "'!''");
    assert_eq!(ok(&'\n'), "\n");

    assert_eq!(ok(""), "''");
    assert_eq!(ok("1"), "'1'");
    assert_eq!(ok("a"), "a");
    assert_eq!(ok("abc"), "abc");
    assert_eq!(ok("あ"), "あ");
    assert_eq!(ok("I'm not a JSON!"), "'I!'m not a JSON!!'");
    assert_eq!(ok("_"), "_");
    assert_eq!(ok("\t"), "\t");
    assert_eq!(ok(" "), "' '");
}

#[test]
fn test_ser_seq() {
    assert_eq!(ok::<[i32]>(&[]), "!()");
    assert_eq!(ok(&[1]), "!(1)");
    assert_eq!(ok(&[1, 2]), "!(1,2)");
    assert_eq!(ok(&[&b"ab"[..], &b""[..]]), "!(!(97,98),!())");
    assert_eq!(ok(&vec![Some(1), None]), "!(1,!n)");
    assert_eq!(ok(&vec![0.0, f64::INFINITY, f64::NAN]), "!(0.0,!n,!n)");
    assert_eq!(
        ok(&vec![Some(Some("a!")), Some(None), None]),
        "!('a!!',!n,!n)"
    );
}

#[test]
fn test_ser_bytes() {
    let buf = vec![];
    let bytes = Bytes::new(&buf);
    assert_eq!(ok(&bytes), "!()".to_string());

    let buf = vec![1, 2, 3];
    let bytes = Bytes::new(&buf);
    assert_eq!(ok(&bytes), "!(1,2,3)".to_string());
}

#[test]
fn test_byte_buf_ser() {
    let bytes = ByteBuf::new();
    assert_eq!(ok(&bytes), "!()".to_string());

    let bytes = ByteBuf::from(vec![1, 2, 3]);
    assert_eq!(ok(&bytes), "!(1,2,3)".to_string());
}

#[test]
fn test_ser_tuple() {
    assert_eq!(ok(&(1,)), "!(1)");
    assert_eq!(ok(&("a", 1)), "!(a,1)");
    assert_eq!(ok(&((3,), (2,), 1)), "!(!(3),!(2),1)");
}

#[test]
fn test_ser_map() {
    assert_eq!(ok(&HashMap::<String, String>::new()), "()");
    assert_eq!(
        ok(&hashmap! {
            "I'm a key!" => "I'm a value!",
        }),
        "('I!'m a key!!':'I!'m a value!!')"
    );
    assert_eq!(
        ok(&hashmap! {
            '!' => 1,
            '\'' => 2,
            'a' => 3,
        }),
        "('!!':1,'!'':2,a:3)"
    );
    assert_eq!(
        ok(&hashmap! {
            "b" => 1,
            "a" => 2,
        }),
        "(a:2,b:1)"
    );
    assert_eq!(
        ok(&hashmap! {
            1_u64 => 10_u128,
            2_u64 => 20_u128,
            3_u64 => 30_u128,
        }),
        "(1:10,2:20,3:30)"
    );

    assert_eq!(ok(&hashmap! {u8::MAX => 1}), format!("({}:1)", u8::MAX));
    assert_eq!(ok(&hashmap! {i8::MIN => 1}), format!("({}:1)", i8::MIN));
    assert_eq!(ok(&hashmap! {u16::MAX => 1}), format!("({}:1)", u16::MAX));
    assert_eq!(ok(&hashmap! {i16::MIN => 1}), format!("({}:1)", i16::MIN));
    assert_eq!(ok(&hashmap! {u32::MAX => 1}), format!("({}:1)", u32::MAX));
    assert_eq!(ok(&hashmap! {i32::MIN => 1}), format!("({}:1)", i32::MIN));
    assert_eq!(ok(&hashmap! {u64::MAX => 1}), format!("({}:1)", u64::MAX));
    assert_eq!(ok(&hashmap! {i64::MIN => 1}), format!("({}:1)", i64::MIN));
    assert_eq!(ok(&hashmap! {u128::MAX => 1}), format!("({}:1)", u128::MAX));
    assert_eq!(ok(&hashmap! {i128::MIN => 1}), format!("({}:1)", i128::MIN));

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
        }),
        "(key1:(1:!((a:A))),key2:())"
    );
    assert_eq!(
        ok(&hashmap! {
            "B" => None,
            "A" => Some("a"),
        }),
        "(A:a,B:!n)"
    );
}

#[test]
fn test_ser_map_err() {
    assert!(to_string(&hashmap! {
        () => 1
    })
    .is_err());
    assert!(to_string(&hashmap! {
        [1] => 1
    })
    .is_err());
    assert!(to_string(&hashmap! {
        vec![1] => 1
    })
    .is_err());
    assert!(to_string(&hashmap! {
        b"a" => 1
    })
    .is_err());
    assert!(to_string(&hashmap! {
        Some(1) => 1,
    })
    .is_err());
    assert!(to_string(&hashmap! {
        Option::<i32>::None => 1,
    })
    .is_err());
    assert!(to_string(&hashmap! {
        ByteBuf::new() => 1
    })
    .is_err());
    assert!(to_string(&hashmap! {
        true => 1
    })
    .is_err());
    assert!(to_string(&hashmap! {
        OrderedFloat(1.0_f32) => 1
    })
    .is_err());
    assert!(to_string(&hashmap! {
        OrderedFloat(1.0_f64) => 1
    })
    .is_err());
    assert!(to_string(&hashmap! {
        btreemap! { 1 => 2 } => 3
    })
    .is_err());
}

#[test]
fn test_unit_struct() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    struct Unit;
    assert_eq!(ok(&Unit), "!n");
    assert_eq!(
        ok(&hashmap! {
            1 => Unit,
        }),
        "(1:!n)"
    );
    assert!(to_string(&hashmap! {
        Unit => 1,
    })
    .is_err());
}

#[test]
fn test_unit_variant() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    enum E {
        A,
        #[serde(rename = "b!")]
        B,
    }
    assert_eq!(ok(&E::A), "A");
    assert_eq!(ok(&E::B), "'b!!'");
    assert_eq!(ok(&[E::B, E::A]), "!('b!!',A)");
    assert_eq!(
        ok(&hashmap! {
            E::A => 1,
            E::B => 2,
        }),
        "('b!!':2,A:1)"
    );
}

#[test]
fn test_newtype_struct() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    struct N(u8);
    assert_eq!(ok(&N(0)), "0");
    assert_eq!(ok(&[N(1)]), "!(1)");
    assert_eq!(
        ok(&hashmap! {
            N(3) => 1,
            N(20) => 2,
            N(100) => 3,
        }),
        "(100:3,20:2,3:1)"
    )
}

#[test]
fn test_newtype_variant() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    enum E {
        #[serde(rename = "a!")]
        A(u8),
        B(Option<i32>),
    }
    assert_eq!(ok(&E::A(0)), "('a!!':0)");
    assert_eq!(ok(&E::B(None)), "(B:!n)");
    assert_eq!(ok(&[E::A(1), E::B(Some(2))]), "!(('a!!':1),(B:2))");
    assert_eq!(
        ok(&hashmap! {
            1 => E::A(100),
        }),
        "(1:('a!!':100))"
    );
    assert!(to_string(&hashmap! {
        E::A(0) => 0
    })
    .is_err());
}

#[test]
fn test_newtype_variant_err() {
    #[derive(Serialize)]
    enum E {
        A(HashMap<Vec<i32>, i32>),
    }
    let e = E::A(hashmap! {vec![1, 2] => 3});
    assert!(to_string(&e).is_err());
    assert!(to_string(&[&e]).is_err());
    assert!(to_string(&hashmap! {1 => &e}).is_err());
}

#[test]
fn test_tuple_struct() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    struct Abc(u8, u16, u32);

    assert_eq!(ok(&Abc(2, 1, 0)), "!(2,1,0)");
    assert_eq!(ok(&[Abc(2, 1, 0)]), "!(!(2,1,0))");
    assert_eq!(
        ok(&hashmap! {
            1 => Abc(1, 2, 3),
        }),
        "(1:!(1,2,3))"
    );
    assert!(to_string(&hashmap! {
        Abc(1, 2, 3) => 1
    })
    .is_err());
}

#[test]
fn test_tuple_variant() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    enum E {
        #[serde(rename = "a!")]
        A(i32, u32),
        B(u8, u8),
    }
    assert_eq!(ok(&E::A(-1, 1)), "('a!!':!(-1,1))");
    assert_eq!(ok(&E::B(0, 1)), "(B:!(0,1))");
    assert_eq!(ok(&[E::A(-1, 1)]), "!(('a!!':!(-1,1)))");
    assert_eq!(
        ok(&hashmap! {
            1 => E::A(1, 2),
            2 => E::B(10, 20),
        }),
        "(1:('a!!':!(1,2)),2:(B:!(10,20)))"
    );
    assert!(to_string(&hashmap! {
       E::A(1, 2) => 1,
       E::B(10, 20) => 2,
    })
    .is_err());
}

#[test]
fn test_struct() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    struct S {
        #[serde(rename = "a!")]
        a: u32,
        b: u8,
    }
    assert_eq!(ok(&S { a: 1, b: 2 }), "('a!!':1,b:2)");
    assert_eq!(ok(&[S { a: 1, b: 2 }]), "!(('a!!':1,b:2))");
    assert_eq!(
        ok(&hashmap! {
            1 => S { a: 1, b: 2 },
        }),
        "(1:('a!!':1,b:2))"
    );
    assert!(to_string(&hashmap! {
       S { a: 1, b: 2 } => 1,
    })
    .is_err());
}

#[test]
fn test_struct_variant() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    enum E {
        A { a: u8 },
        B { a: u8, b: u32 },
    }
    assert_eq!(ok(&E::A { a: 1 }), "(A:(a:1))");
    assert_eq!(ok(&E::B { a: 2, b: 3 }), "(B:(a:2,b:3))");
    assert_eq!(ok(&[E::A { a: 1 }]), "!((A:(a:1)))");
    assert_eq!(
        ok(&hashmap! {
            1 => E::A { a: 1 },
        }),
        "(1:(A:(a:1)))"
    );
    assert!(to_string(&hashmap! {
       E::A { a: 1 } => 1,
    })
    .is_err());
}

#[test]
fn test_key_error() {
    #[derive(Serialize, Hash, Eq, PartialEq)]
    struct S {
        a: u32,
    }
    let err = to_string(&hashmap! {
        S { a: 0 } => 1,
    })
    .expect_err("Not Error");
    assert_eq!(err.to_string(), "key must be a string");
    assert_eq!(format!("{:?}", err), "key must be a string");
}

#[test]
fn test_custom_error() {
    struct P;

    impl Serialize for P {
        fn serialize<S>(&self, _s: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            Err(ser::Error::custom("foo!"))
        }
    }

    let err = to_string(&P).expect_err("Not Err");
    assert_eq!(err.to_string(), "foo!");
    assert_eq!(format!("{:?}", err), "foo!");
}
