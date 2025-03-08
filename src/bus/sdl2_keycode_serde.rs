use std::fmt;

use sdl2::keyboard::Keycode;
use serde::{
    de::{self, Visitor},
    Deserializer, Serializer,
};

pub fn serialize<S>(kc: &Keycode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&kc.to_string())
}

pub struct KeycodeVisitor;

impl Visitor<'_> for KeycodeVisitor {
    type Value = Keycode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid SDL2 Scancode integer")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
            Keycode::from_i32(value as i32)
                .ok_or(E::custom("keycode not recognized by SDL2".to_string()))
        } else {
            Err(E::custom(format!("i32 out of range: {}", value)))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::convert::TryFrom;
        i64::try_from(v)
            .map_err(|_| E::custom(format!("out of range: {}", v)))
            .and_then(|v| self.visit_i64(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Keycode::from_name(v).ok_or(E::custom("Keycode not recognized by SDL2".to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Keycode, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_i64(KeycodeVisitor)
}
