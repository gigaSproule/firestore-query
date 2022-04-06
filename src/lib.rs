pub mod google {
    pub mod firestore {
        pub mod v1 {
            tonic::include_proto!("google.firestore.v1");
        }
    }

    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }

    pub mod r#type {
        tonic::include_proto!("google.r#type");
    }
}

use google::firestore::v1::{value::ValueType, ArrayValue, MapValue, Value};
use google::r#type::LatLng;
use prost_types::Timestamp;
use serde::ser::SerializeStruct;
use serde::{
    self,
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::collections::HashMap;
use std::fmt;

impl Serialize for google::firestore::v1::ArrayValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("google::firestore::v1::ArrayValue", 1)?;
        state.serialize_field("values", &self.values)?;
        state.end()
    }
}

impl Serialize for google::firestore::v1::MapValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("google::firestore::v1::MapValue", 1)?;
        state.serialize_field("fields", &self.fields)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for google::firestore::v1::value::ValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            ValueType,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`value_type`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value_type" => Ok(Field::ValueType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = google::firestore::v1::value::ValueType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Value")
            }

            fn visit_seq<V>(
                self,
                mut seq: V,
            ) -> Result<google::firestore::v1::value::ValueType, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let value_type = seq.next_element()?;
                let value = // match std::any::type_name_of_val(&value_type) {
                    // "std::string::String" => {
                        google::firestore::v1::value::ValueType::StringValue(value_type.unwrap());
                // }
                // };
                Ok(value)
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> Result<google::firestore::v1::value::ValueType, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value_type = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ValueType => {
                            if value_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("value_type"));
                            }
                            value_type = Some(map.next_value()?);
                        }
                    }
                }
                let value = // match std::any::type_name_of_val(&value_type) {
                    // "std::string::String" => {
                        google::firestore::v1::value::ValueType::StringValue(value_type.unwrap());
                // }
                // };
                Ok(value)
            }
        }

        const FIELDS: &'static [&'static str] = &["value_type"];
        deserializer.deserialize_struct("Value", FIELDS, ValueVisitor)
    }
}

impl Serialize for google::firestore::v1::value::ValueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            google::firestore::v1::value::ValueType::NullValue(_) => todo!(),
            google::firestore::v1::value::ValueType::BooleanValue(value) => {
                value.serialize(serializer)
            }
            google::firestore::v1::value::ValueType::IntegerValue(value) => {
                value.serialize(serializer)
            }
            google::firestore::v1::value::ValueType::DoubleValue(value) => {
                value.serialize(serializer)
            }
            google::firestore::v1::value::ValueType::TimestampValue(_) => {
                todo!("Need to implement TimestampValue")
            }
            google::firestore::v1::value::ValueType::StringValue(value) => {
                value.serialize(serializer)
            }
            google::firestore::v1::value::ValueType::BytesValue(value) => {
                value.serialize(serializer)
            }
            google::firestore::v1::value::ValueType::ReferenceValue(_) => {
                todo!("Need to implement ReferenceValue")
            }
            google::firestore::v1::value::ValueType::GeoPointValue(_) => {
                todo!("Need to implement GeoPointValue")
            }
            google::firestore::v1::value::ValueType::ArrayValue(value) => {
                value.serialize(serializer)
            }
            google::firestore::v1::value::ValueType::MapValue(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for google::firestore::v1::Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            ValueType,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`value_type`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value_type" => Ok(Field::ValueType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = google::firestore::v1::Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Value")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<google::firestore::v1::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let value_type = seq.next_element()?;
                Ok(google::firestore::v1::Value { value_type })
            }

            fn visit_map<V>(self, mut map: V) -> Result<google::firestore::v1::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value_type = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ValueType => {
                            if value_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("value_type"));
                            }
                            value_type = Some(map.next_value()?);
                        }
                    }
                }
                Ok(google::firestore::v1::Value { value_type })
            }
        }

        const FIELDS: &'static [&'static str] = &["value_type"];
        deserializer.deserialize_struct("Value", FIELDS, ValueVisitor)
    }
}

impl Serialize for google::firestore::v1::Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("google::firestore::v1::Value", 1)?;
        state.serialize_field("value_type", &self.value_type)?;
        state.end()
    }
}

mod opt_value_type {
    use super::google::firestore::v1::value::ValueType;
    use super::ValueTypeDef;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<ValueType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "ValueTypeDef")] &'a ValueType);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ValueType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "ValueTypeDef")] ValueType);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "prost_types::Timestamp")]
pub struct TimestampDef {
    pub seconds: i64,
    pub nanos: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "google::r#type::LatLng")]
pub struct LatLngDef {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "google::firestore::v1::ArrayValue")]
pub struct ArrayValueDef {
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "google::firestore::v1::MapValue")]
pub struct MapValueDef {
    pub fields: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "google::firestore::v1::value::ValueType")]
enum ValueTypeDef {
    NullValue(i32),
    BooleanValue(bool),
    IntegerValue(i64),
    DoubleValue(f64),
    #[serde(with = "TimestampDef")]
    TimestampValue(Timestamp),
    StringValue(String),
    BytesValue(Vec<u8>),
    ReferenceValue(String),
    #[serde(with = "LatLngDef")]
    GeoPointValue(LatLng),
    #[serde(with = "ArrayValueDef")]
    ArrayValue(ArrayValue),
    #[serde(with = "MapValueDef")]
    MapValue(MapValue),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "google::firestore::v1::Value")]
struct ValueDef {
    #[serde(default, with = "opt_value_type")]
    value_type: Option<ValueType>,
}
