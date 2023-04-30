use crate::read;
use alloc::{boxed::Box, string::ToString};
use core::{fmt, result};
use serde::de::{self, Deserialize, Expected, Unexpected, Visitor};

pub struct Error {
    code: ErrorCode,
    position: usize,
}

enum ErrorCode {
    Message(Box<str>),
    InvalidChar(u8, u8),
    InvalidEscape(u8),
    EofWhileParsingValue,
    InvalidNumber,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::InvalidChar(unexp, exp) => {
                write!(f, "invalid char: {}, expected {}", unexp, exp)
            }
            ErrorCode::InvalidEscape(unexp) => {
                write!(f, "invalid escape char: {}", unexp)
            }
            ErrorCode::EofWhileParsingValue => f.write_str("EOF while parsing a value"),
            ErrorCode::InvalidNumber => f.write_str("invalid number"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at position {}", self.code, self.position)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({}, position: {})", self.code, self.position)
    }
}

impl de::StdError for Error {}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error {
            code: ErrorCode::Message(msg.to_string().into_boxed_str()),
            position: 0,
        }
    }

    fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
        if let Unexpected::Unit = unexp {
            Error::custom(format_args!("invalid type: null, expected {}", exp))
        } else {
            Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
        }
    }
}

pub struct Deserializer<R> {
    read: R,
}

impl<R> Deserializer<R> {
    pub fn new(read: R) -> Self {
        Deserializer { read }
    }
}

impl<'a> Deserializer<read::SliceRead<'a>> {
    /// Creates a JSON deserializer from a `&[u8]`.
    pub fn from_slice(bytes: &'a [u8]) -> Self {
        Deserializer::new(read::SliceRead::new(bytes))
    }
}

impl<R: read::Read> Deserializer<R> {
    #[cold]
    fn error(&self, code: ErrorCode) -> Error {
        let position = self.read.position();
        Error { code, position }
    }

    #[cold]
    fn invalid_escaped_type(&mut self, b: Option<u8>, exp: &dyn Expected) -> Error {
        let err = match b.unwrap_or(b'\x00') {
            b'n' => de::Error::invalid_type(Unexpected::Unit, exp),
            b't' => de::Error::invalid_type(Unexpected::Bool(true), exp),
            b'f' => de::Error::invalid_type(Unexpected::Bool(false), exp),
            b'(' => de::Error::invalid_type(Unexpected::Seq, exp),
            b => self.error(ErrorCode::InvalidEscape(b)),
        };
        self.fix_position(err)
    }

    #[cold]
    fn invalid_type(&mut self, b: Option<u8>, exp: &dyn Expected) -> Error {
        let err = match b.unwrap_or(b'\x00') {
            b'!' => self.invalid_escaped_type(self.read.next(), exp),
            b'(' => de::Error::invalid_type(Unexpected::Map, exp),
            _ => todo!(),
        };
        self.fix_position(err)
    }

    #[cold]
    fn fix_position(&self, err: Error) -> Error {
        self.error(err.code)
    }

    fn deserialize_number<'de, V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match self.read.peak() {
            Some(b) => b,
            None => return Err(self.error(ErrorCode::EofWhileParsingValue)),
        };
        let f = match peek {
            b'-' => {
                self.read.eat_char();
                self.parse_significand(false)?
            }
            b'0'..=b'9' => self.parse_significand(true)?,
            _ => {
                self.read.eat_char();
                return Err(self.invalid_type(Some(peek), &visitor));
            }
        };
        visitor.visit_f64(f)
    }

    fn parse_significand(&mut self, positive: bool) -> Result<f64> {
        let next = match self.read.next() {
            Some(b) => b,
            None => return Err(self.error(ErrorCode::EofWhileParsingValue)),
        };
        match next {
            b'0' => match self.read.peek_or_null() {
                b'0'..=b'9' => return Err(self.error(ErrorCode::InvalidNumber)),
                _ => self.parse_number(positive, 0),
            },
            c @ b'1'..=b'9' => {
                let mut significand = (c - b'0') as f64;
                loop {
                    match self.read.peek_or_null() {
                        c @ b'0'..=b'9' => {
                            significand = significand * 10.0 + (c - b'0') as f64;
                        }
                        _ => self.parse_number(positive, significand),
                    }
                }
            }
        }
    }

    fn parse_number(&mut self, positive: bool, significand: f64) -> Result<ParserNumber> {
        match self.read.peek_or_null() {
            b'.' => self.parse_decimal(positive, significand, 0),
            b'e' => self.parse_exponent(positive, significand, 0),
            _ => Ok(if positive { significand } else { -significand }),
        }
    }

    fn parse_decimal(
        &mut self,
        positive: bool,
        mut significand: f64,
        exponent_before_decimal_point: i32,
    ) -> Result<f64> {
        self.read.eat_char();

        let mut exponent_after_decimal_point = 0;
        while let c @ b'0'..=b'9' = self.read.peek_or_null() {
            self.eat_char();
            significand = significand * 10 + (c - b'0') as f64;
            exponent_after_decimal_point -= 1;
        }

        if exponent_after_decimal_point == 0 {
            return match self.read.peak() {
                Some(_) => Err(self.error(ErrorCode::InvalidNumber)),
                None => Err(self.error(ErrorCode::EofWhileParsingValue)),
            };
        }

        let exponent = exponent_before_decimal_point + exponent_after_decimal_point;
        match self.read.peek_or_null() {
            b'e' => self.parse_exponent(positive, significand, exponent),
            _ => self.f64_from_parts(positive, significand, exponent),
        }
    }

    fn parse_exponent(
        &mut self,
        positive: bool,
        significand: f64,
        starting_exp: i32,
    ) -> Result<f64> {
        todo!()
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: read::Read,
{
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.read.next() {
            Some(b'!') => (),
            Some(b) => return Err(self.error(ErrorCode::InvalidChar(b, b'!'))),
            None => return Err(self.error(ErrorCode::EofWhileParsingValue)),
        };
        match self.read.next() {
            Some(b't') => visitor.visit_bool(true),
            Some(b'f') => visitor.visit_bool(false),
            b => Err(self.invalid_escaped_type(b, &visitor)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_number(visitor)
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i128<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u128<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

pub fn from_slice<'de, V>(s: &'de [u8]) -> Result<V>
where
    V: Deserialize<'de>,
{
    let mut de = Deserializer::from_slice(s);
    V::deserialize(&mut de)
}

pub fn from_str<'de, V>(s: &'de str) -> Result<V>
where
    V: Deserialize<'de>,
{
    from_slice(s.as_bytes())
}
