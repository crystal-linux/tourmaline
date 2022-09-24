use std::{collections::HashMap, mem};

use thiserror::Error;

use super::record::RecordValue;

#[derive(Default)]
pub struct RecordSerializer {
    list_serializer: Option<RecordListSerializer>,
    map_serializer: Option<RecordMapSerializer>,
}
pub struct RecordListSerializer {
    serializer: Option<Box<RecordSerializer>>,
    entries: Vec<RecordValue>,
}
pub struct RecordMapSerializer {
    serializer: Option<Box<RecordSerializer>>,
    last_key: Option<RecordValue>,
    entries: Vec<(RecordValue, RecordValue)>,
}

impl RecordSerializer {
    pub fn list_serializer<'a>(&'a mut self, cap: Option<usize>) -> &'a mut RecordListSerializer {
        if self.list_serializer.is_none() {
            self.list_serializer = Some(RecordListSerializer::with_capacity_opt(cap));
        }
        self.list_serializer.as_mut().unwrap()
    }

    pub fn map_serializer(&mut self, cap: Option<usize>) -> &mut RecordMapSerializer {
        if self.map_serializer.is_none() {
            self.map_serializer = Some(RecordMapSerializer::with_capacity_opt(cap));
        }
        self.map_serializer.as_mut().unwrap()
    }
}

impl RecordListSerializer {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            serializer: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            serializer: None,
        }
    }

    pub fn with_capacity_opt(capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(cap),
            None => Self::new(),
        }
    }

    pub fn serializer(&mut self) -> &mut RecordSerializer {
        if self.serializer.is_none() {
            self.serializer = Some(Box::new(RecordSerializer::default()));
        }
        self.serializer.as_mut().unwrap()
    }
}

impl RecordMapSerializer {
    pub fn new() -> Self {
        Self {
            last_key: None,
            entries: Vec::new(),
            serializer: None,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            last_key: None,
            entries: Vec::with_capacity(capacity),
            serializer: None,
        }
    }
    pub fn with_capacity_opt(capacity: Option<usize>) -> Self {
        match capacity {
            Some(cap) => Self::with_capacity(cap),
            None => Self::new(),
        }
    }
    pub fn serializer(&mut self) -> &mut RecordSerializer {
        if self.serializer.is_none() {
            self.serializer = Some(Box::new(RecordSerializer::default()));
        }
        self.serializer.as_mut().unwrap()
    }
}

#[derive(Error, Debug)]
pub enum RecordSerError {
    #[error("{0}")]
    Msg(String),

    #[error("Encountered map value before map key")]
    ValueBeforeKey,
}

impl serde::ser::Error for RecordSerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Msg(msg.to_string())
    }
}

impl<'a> serde::ser::Serializer for &'a mut RecordSerializer {
    type Ok = RecordValue;

    type Error = RecordSerError;

    type SerializeSeq = &'a mut RecordListSerializer;

    type SerializeTuple = &'a mut RecordListSerializer;

    type SerializeTupleStruct = &'a mut RecordListSerializer;

    type SerializeTupleVariant = &'a mut RecordListSerializer;

    type SerializeMap = &'a mut RecordMapSerializer;

    type SerializeStruct = &'a mut RecordMapSerializer;

    type SerializeStructVariant = &'a mut RecordMapSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok((v as i64).into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok((v as i64).into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok((v as i64).into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok((v as i64).into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok((v as i64).into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok((v as i64).into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok((v as i64).into())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok((v as f64).into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string().into())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_vec().into())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(RecordValue::Null)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        let mut map = HashMap::new();
        map.insert(variant.to_string(), value.serialize(self)?);
        Ok(map.into())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self.list_serializer(len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self.list_serializer(Some(len)))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self.list_serializer(Some(len)))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self.list_serializer(Some(len)))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self.map_serializer(len))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self.map_serializer(Some(len)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self.map_serializer(Some(len)))
    }
}

impl<'a> serde::ser::SerializeTuple for &'a mut RecordListSerializer {
    type Ok = RecordValue;

    type Error = RecordSerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let val = value.serialize(self.serializer())?;
        self.entries.push(val);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(mem::take(&mut self.entries).into())
    }
}

impl<'a> serde::ser::SerializeSeq for &'a mut RecordListSerializer {
    type Ok = RecordValue;

    type Error = RecordSerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let val = value.serialize(self.serializer())?;
        self.entries.push(val);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(mem::take(&mut self.entries).into())
    }
}

impl<'a> serde::ser::SerializeTupleStruct for &'a mut RecordListSerializer {
    type Ok = RecordValue;

    type Error = RecordSerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let val = value.serialize(self.serializer())?;
        self.entries.push(val);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(mem::take(&mut self.entries).into())
    }
}

impl<'a> serde::ser::SerializeTupleVariant for &'a mut RecordListSerializer {
    type Ok = RecordValue;

    type Error = RecordSerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let val = value.serialize(self.serializer())?;
        self.entries.push(val);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(mem::take(&mut self.entries).into())
    }
}

impl<'a> serde::ser::SerializeMap for &'a mut RecordMapSerializer {
    type Ok = RecordValue;

    type Error = RecordSerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let val = key.serialize(self.serializer())?;
        self.last_key = Some(val);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if let Some(last_key) = self.last_key.take() {
            let val = value.serialize(self.serializer())?;
            self.entries.push((last_key, val));

            Ok(())
        } else {
            Err(RecordSerError::ValueBeforeKey)
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(RecordValue::Map(mem::take(&mut self.entries)))
    }
}

impl<'a> serde::ser::SerializeStruct for &'a mut RecordMapSerializer {
    type Ok = RecordValue;

    type Error = RecordSerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let val = value.serialize(self.serializer())?;
        self.entries.push((key.into(), val));

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(RecordValue::Map(mem::take(&mut self.entries)))
    }
}

impl<'a> serde::ser::SerializeStructVariant for &'a mut RecordMapSerializer {
    type Ok = RecordValue;

    type Error = RecordSerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let val = value.serialize(self.serializer())?;
        self.entries.push((key.into(), val));

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(RecordValue::Map(mem::take(&mut self.entries)))
    }
}
