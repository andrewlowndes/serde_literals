use core::fmt;
use serde::{
    de::{self, Unexpected, Visitor},
    Deserializer, Serializer,
};

// serde_literals
// deserialise and serialise literal strings, ints, floats, bools and chars into enum unit variants
pub struct LitStr<'a>(&'a str);

impl<'a, 'de> Visitor<'de> for LitStr<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the lit {}", self.0)
    }

    fn visit_str<E>(self, s: &str) -> Result<(), E>
    where
        E: de::Error,
    {
        if s == self.0 {
            Ok(())
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

pub struct LitFloat(f64);

impl<'de> Visitor<'de> for LitFloat {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the lit {}", self.0)
    }

    fn visit_f64<E>(self, v: f64) -> Result<(), E>
    where
        E: de::Error,
    {
        if v == self.0 {
            Ok(())
        } else {
            Err(de::Error::invalid_value(Unexpected::Float(v), &self))
        }
    }
}

pub struct LitInt<const N: i64>;

impl<const N: i64> LitInt<N> {
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(Self)
    }

    pub fn serialize<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(N)
    }
}

impl<'de, const N: i64> Visitor<'de> for LitInt<N> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the lit {}", N)
    }

    fn visit_i64<E>(self, v: i64) -> Result<(), E>
    where
        E: de::Error,
    {
        if v == N {
            Ok(())
        } else {
            Err(de::Error::invalid_value(Unexpected::Signed(v), &self))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<(), E>
    where
        E: de::Error,
    {
        if v as i64 == N {
            Ok(())
        } else {
            Err(de::Error::invalid_value(Unexpected::Unsigned(v), &self))
        }
    }
}

pub struct LitBool<const B: bool>;

impl<const B: bool> LitBool<B> {
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_bool(Self)
    }

    pub fn serialize<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(B)
    }
}

impl<'de, const B: bool> Visitor<'de> for LitBool<B> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the lit {}", B)
    }

    fn visit_bool<E>(self, v: bool) -> Result<(), E>
    where
        E: de::Error,
    {
        if v == B {
            Ok(())
        } else {
            Err(de::Error::invalid_value(Unexpected::Bool(v), &self))
        }
    }
}

pub struct LitChar<const C: char>;

impl<const C: char> LitChar<C> {
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_char(Self)
    }

    pub fn serialize<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_char(C)
    }
}

impl<'de, const C: char> Visitor<'de> for LitChar<C> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the lit {}", C)
    }

    fn visit_str<E>(self, v: &str) -> Result<(), E>
    where
        E: de::Error,
    {
        if v.starts_with(C) {
            Ok(())
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(v), &self))
        }
    }
}

#[macro_export]
macro_rules! lit_str {
    ($struct_name:ident, $val:expr) => {
        pub struct $struct_name;

        impl $struct_name {
            pub fn deserialize<'de, D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<(), D::Error> {
                deserializer.deserialize_str($crate::LitStr($val))
            }

            pub fn serialize<S: serde::Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str($val)
            }
        }
    };
}

#[macro_export]
macro_rules! lit_float {
    ($struct_name:ident, $val:expr) => {
        pub struct $struct_name;

        impl $struct_name {
            pub fn deserialize<'de, D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<(), D::Error> {
                deserializer.deserialize_f64($crate::LitFloat($val as f64))
            }

            pub fn serialize<S: serde::Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_f64($val as f64)
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};

    lit_str!(LitAuto, "auto");
    lit_str!(LitBlah, "blah");
    lit_float!(Lit3_1, 3.1);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(untagged)]
    enum Items {
        #[serde(with = "LitAuto")]
        Auto,
        #[serde(with = "LitBlah")]
        Blah,
        #[serde(with = "LitInt::<123>")]
        Num123,
        #[serde(with = "Lit3_1")]
        Num3Dot1,
        Number(f64),
        #[serde(with = "LitBool::<true>")]
        True,
        #[serde(with = "LitBool::<false>")]
        False,
        #[serde(with = "LitChar::<'z'>")]
        SingleChar,
    }

    #[test]
    fn test_serde() {
        assert_eq!(
            serde_json::to_string_pretty(&Items::Number(4.5)).unwrap(),
            "4.5"
        );
        assert_eq!(
            serde_json::to_string_pretty(&Items::Auto).unwrap(),
            "\"auto\""
        );
        assert_eq!(
            serde_json::to_string_pretty(&Items::Blah).unwrap(),
            "\"blah\""
        );
        assert_eq!(serde_json::to_string_pretty(&Items::Num123).unwrap(), "123");
        assert_eq!(
            serde_json::to_string_pretty(&Items::Num3Dot1).unwrap(),
            "3.1"
        );
        assert_eq!(serde_json::to_string_pretty(&Items::True).unwrap(), "true");
        assert_eq!(
            serde_json::to_string_pretty(&Items::False).unwrap(),
            "false"
        );
        assert_eq!(
            serde_json::to_string_pretty(&Items::SingleChar).unwrap(),
            "\"z\""
        );

        assert_eq!(
            serde_json::from_str::<Items>("2.3").unwrap(),
            Items::Number(2.3)
        );
        assert_eq!(
            serde_json::from_str::<Items>("\"auto\"").unwrap(),
            Items::Auto
        );
        assert_eq!(
            serde_json::from_str::<Items>("\"blah\"").unwrap(),
            Items::Blah
        );
        assert_eq!(serde_json::from_str::<Items>("123").unwrap(), Items::Num123);
        assert_eq!(
            serde_json::from_str::<Items>("3.1").unwrap(),
            Items::Num3Dot1
        );
        assert_eq!(serde_json::from_str::<Items>("true").unwrap(), Items::True);
        assert_eq!(
            serde_json::from_str::<Items>("false").unwrap(),
            Items::False
        );
        assert_eq!(
            serde_json::from_str::<Items>("\"z\"").unwrap(),
            Items::SingleChar
        );
    }
}
