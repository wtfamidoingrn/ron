use serde::de::{self, Visitor};

use super::{Deserializer, Error, Result};

pub struct IdDeserializer<'a, 'b: 'a> {
    d: &'a mut Deserializer<'b>,
    map_as_struct: bool,
}

impl<'a, 'b: 'a> IdDeserializer<'a, 'b> {
    pub fn new(map_as_struct: bool, d: &'a mut Deserializer<'b>) -> Self {
        IdDeserializer { d, map_as_struct }
    }
}

impl<'a, 'b: 'a, 'c> de::Deserializer<'b> for &'c mut IdDeserializer<'a, 'b> {
    type Error = Error;

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        if self.map_as_struct {
            self.d.bytes.expect_byte(b'"', Error::ExpectedString)?;
        }
        let result = self.d.deserialize_identifier(visitor);
        if self.map_as_struct {
            self.d.bytes.expect_byte(b'"', Error::ExpectedStringEnd)?;
        }
        result
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        self.deserialize_identifier(visitor)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        self.deserialize_identifier(visitor)
    }

    fn deserialize_bool<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    #[cfg(feature = "integer128")]
    fn deserialize_i128<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_u8<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    #[cfg(feature = "integer128")]
    fn deserialize_u128<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_f32<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_f64<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_string<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_option<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_tuple_struct<V>(self, _: &'static str, _: usize, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        Err(Error::ExpectedIdentifier)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'b>,
    {
        self.deserialize_any(visitor)
    }
}
