//! Deserializer for P4Data

#![allow(missing_docs)]

use p4runtime::p4::v1::{p4_data, P4Data, P4Varbit};
use serde::de::{self, IntoDeserializer, MapAccess, SeqAccess};

pub fn from_p4data<'de, T>(data: &'de P4Data) -> Result<T, DeserializeP4DataError>
where
    T: de::Deserialize<'de>,
{
    let deserializer = Deserializer::new(data);
    T::deserialize(deserializer)
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializeP4DataError {
    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Expected a bool")]
    ExpectedBool,

    #[error("Expected an i8")]
    ExpectedI8,

    #[error("Expected an i16")]
    ExpectedI16,

    #[error("Expected an i32")]
    ExpectedI32,

    #[error("Expected an i64")]
    ExpectedI64,

    #[error("Expected a u8")]
    ExpectedU8,

    #[error("Expected a u16")]
    ExpectedU16,

    #[error("Expected a u32")]
    ExpectedU32,

    #[error("Expected a u64")]
    ExpectedU64,

    #[error("Expected bytes")]
    ExpectedBytes,

    #[error("Expected a tuple")]
    ExpectedTuple,

    #[error("Expected a struct")]
    ExpectedStruct,
}

impl de::Error for DeserializeP4DataError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        DeserializeP4DataError::Custom(msg.to_string())
    }
}

/// Deserializer
pub struct Deserializer<'de> {
    data: &'de P4Data,
}

impl<'de> Deserializer<'de> {
    /// Create a new Deserializer
    pub fn new(data: &'de P4Data) -> Self {
        Deserializer { data }
    }

    fn parse_bool(&self) -> Result<bool, DeserializeP4DataError> {
        match self.data.data {
            Some(p4_data::Data::Bool(b)) => Ok(b),
            _ => Err(DeserializeP4DataError::ExpectedBool),
        }
    }

    fn parse_i8(&self) -> Result<i8, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if bytes.len() != 1 {
                    return Err(DeserializeP4DataError::ExpectedI8);
                }
                Ok(bytes[0] as i8)
            }
            Some(p4_data::Data::Varbit(P4Varbit {
                bitstring,
                bitwidth,
            })) => {
                if *bitwidth > 8 {
                    return Err(DeserializeP4DataError::ExpectedI8);
                }
                Ok(bitstring[0] as i8)
            }
            _ => Err(DeserializeP4DataError::ExpectedI8),
        }
    }

    fn parse_i16(&self) -> Result<i16, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if bytes.len() > 2 {
                    return Err(DeserializeP4DataError::ExpectedI16);
                }

                let mut buf = [0; 2];
                buf[2 - bytes.len()..].copy_from_slice(bytes);
                Ok(i16::from_be_bytes(buf))
            }
            Some(p4_data::Data::Varbit(P4Varbit {
                bitstring,
                bitwidth,
            })) => {
                if *bitwidth > 16 {
                    return Err(DeserializeP4DataError::ExpectedI16);
                }

                let mut buf = [0; 2];
                buf[2 - (bitwidth / 8) as usize..].copy_from_slice(bitstring);
                Ok(i16::from_be_bytes(buf))
            }
            _ => Err(DeserializeP4DataError::ExpectedI16),
        }
    }

    fn parse_i32(&self) -> Result<i32, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if bytes.len() > 4 {
                    return Err(DeserializeP4DataError::ExpectedI32);
                }

                let mut buf = [0; 4];
                buf[4 - bytes.len()..].copy_from_slice(bytes);
                Ok(i32::from_be_bytes(buf))
            }
            Some(p4_data::Data::Varbit(P4Varbit {
                bitstring,
                bitwidth,
            })) => {
                if *bitwidth > 32 {
                    return Err(DeserializeP4DataError::ExpectedI32);
                }

                let mut buf = [0; 4];
                buf[4 - (bitwidth / 8) as usize..].copy_from_slice(bitstring);
                Ok(i32::from_be_bytes(buf))
            }
            _ => Err(DeserializeP4DataError::ExpectedI32),
        }
    }

    fn parse_i64(&self) -> Result<i64, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if bytes.len() > 8 {
                    return Err(DeserializeP4DataError::ExpectedI64);
                }

                let mut buf = [0; 8];
                buf[8 - bytes.len()..].copy_from_slice(bytes);
                Ok(i64::from_be_bytes(buf))
            }
            Some(p4_data::Data::Varbit(P4Varbit {
                bitstring,
                bitwidth,
            })) => {
                if *bitwidth > 64 {
                    return Err(DeserializeP4DataError::ExpectedI64);
                }

                let mut buf = [0; 8];
                buf[8 - (bitwidth / 8) as usize..].copy_from_slice(bitstring);
                Ok(i64::from_be_bytes(buf))
            }
            _ => Err(DeserializeP4DataError::ExpectedI64),
        }
    }

    fn parse_u8(&self) -> Result<u8, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if bytes.len() != 1 {
                    return Err(DeserializeP4DataError::ExpectedU8);
                }
                Ok(bytes[0])
            }
            Some(p4_data::Data::Varbit(P4Varbit {
                bitstring,
                bitwidth,
            })) => {
                if *bitwidth > 8 {
                    return Err(DeserializeP4DataError::ExpectedU8);
                }
                Ok(bitstring[0])
            }
            _ => Err(DeserializeP4DataError::ExpectedU8),
        }
    }

    fn parse_u16(&self) -> Result<u16, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if bytes.len() > 2 {
                    return Err(DeserializeP4DataError::ExpectedU16);
                }

                let mut buf = [0; 2];
                buf[2 - bytes.len()..].copy_from_slice(bytes);
                Ok(u16::from_be_bytes(buf))
            }
            Some(p4_data::Data::Varbit(P4Varbit {
                bitstring,
                bitwidth,
            })) => {
                if *bitwidth > 16 {
                    return Err(DeserializeP4DataError::ExpectedU16);
                }

                let mut buf = [0; 2];
                buf[2 - (bitwidth / 8) as usize..].copy_from_slice(bitstring);
                Ok(u16::from_be_bytes(buf))
            }
            _ => Err(DeserializeP4DataError::ExpectedU16),
        }
    }

    fn parse_u32(&self) -> Result<u32, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if bytes.len() > 4 {
                    return Err(DeserializeP4DataError::ExpectedU32);
                }

                let mut buf = [0; 4];
                buf[4 - bytes.len()..].copy_from_slice(bytes);
                Ok(u32::from_be_bytes(buf))
            }
            Some(p4_data::Data::Varbit(P4Varbit {
                bitstring,
                bitwidth,
            })) => {
                if *bitwidth > 32 {
                    return Err(DeserializeP4DataError::ExpectedU32);
                }

                let mut buf = [0; 4];
                buf[4 - (bitwidth / 8) as usize..].copy_from_slice(bitstring);
                Ok(u32::from_be_bytes(buf))
            }
            _ => Err(DeserializeP4DataError::ExpectedU32),
        }
    }

    fn parse_u64(&self) -> Result<u64, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if bytes.len() > 8 {
                    return Err(DeserializeP4DataError::ExpectedU64);
                }

                let mut buf = [0; 8];
                buf[8 - bytes.len()..].copy_from_slice(bytes);
                Ok(u64::from_be_bytes(buf))
            }
            Some(p4_data::Data::Varbit(P4Varbit {
                bitstring,
                bitwidth,
            })) => {
                if *bitwidth > 64 {
                    return Err(DeserializeP4DataError::ExpectedU64);
                }

                let mut buf = [0; 8];
                buf[8 - (bitwidth / 8) as usize..].copy_from_slice(bitstring);
                Ok(u64::from_be_bytes(buf))
            }
            _ => Err(DeserializeP4DataError::ExpectedU64),
        }
    }

    fn parse_bytes(&self) -> Result<&[u8], DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => Ok(bytes),
            _ => Err(DeserializeP4DataError::ExpectedBytes),
        }
    }

    fn parse_bytes_buf(&self) -> Result<Vec<u8>, DeserializeP4DataError> {
        match &self.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => Ok(bytes.clone()),
            _ => Err(DeserializeP4DataError::ExpectedBytes),
        }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = DeserializeP4DataError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.parse_bytes_buf()?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.data.data.is_none() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(StructLike {
            de: &self,
            fields: None,
            index: 0,
        })
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(StructLike {
            de: &self,
            fields: None,
            index: 0,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(StructLike {
            de: &self,
            fields: None,
            index: 0,
        })
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(StructLike {
            de: &self,
            fields: Some(fields),
            index: 0,
        })
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct StructLike<'a, 'de: 'a> {
    de: &'a Deserializer<'de>,
    fields: Option<&'a [&'a str]>,
    index: usize,
}

impl<'de, 'a> SeqAccess<'de> for StructLike<'a, 'de> {
    type Error = DeserializeP4DataError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match &self.de.data.data {
            Some(p4_data::Data::Bitstring(bytes)) => {
                if self.index >= bytes.len() {
                    return Ok(None);
                }

                let byte = bytes[self.index];
                self.index += 1;
                seed.deserialize(byte.into_deserializer()).map(Some)
            }
            Some(p4_data::Data::Tuple(tuple)) => {
                if self.index >= tuple.members.len() {
                    return Ok(None);
                }

                let de = Deserializer::new(&tuple.members[self.index]);
                let value = seed.deserialize(de)?;
                self.index += 1;

                Ok(Some(value))
            }
            _ => Err(DeserializeP4DataError::ExpectedTuple),
        }
    }
}

impl<'de, 'a> MapAccess<'de> for StructLike<'a, 'de> {
    type Error = DeserializeP4DataError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.index >= self.fields.as_ref().unwrap().len() {
            return Ok(None);
        }
        let field_name = self.fields.as_ref().unwrap()[self.index];
        seed.deserialize(field_name.into_deserializer()).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match &self.de.data.data {
            Some(p4_data::Data::Struct(structure)) => {
                let value = seed.deserialize(Deserializer::new(&structure.members[self.index]))?;
                self.index += 1;
                Ok(value)
            }
            _ => Err(DeserializeP4DataError::ExpectedStruct),
        }
    }
}

#[cfg(test)]
mod tests {
    use p4runtime::p4::v1::P4StructLike;

    use super::*;

    #[test]
    fn deserialize_struct() {
        #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
        struct DigestA {
            dst_addr: u64,
            port: u16,
            src_addr: u64,
        }

        let p4_data = P4Data {
            data: Some(p4_data::Data::Struct(P4StructLike {
                members: vec![
                    P4Data {
                        data: Some(p4_data::Data::Bitstring(vec![0, 0, 0, 0, 0, 0, 0, 1])),
                    },
                    P4Data {
                        data: Some(p4_data::Data::Bitstring(vec![0, 1])),
                    },
                    P4Data {
                        data: Some(p4_data::Data::Bitstring(vec![0, 0, 0, 0, 0, 0, 0, 2])),
                    },
                ],
            })),
        };

        let result: DigestA = from_p4data(&p4_data).unwrap();
        assert_eq!(result.dst_addr, 1);
        assert_eq!(result.port, 1);
        assert_eq!(result.src_addr, 2);
    }
}
