use std::fmt;
use std::marker::PhantomData;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeTupleStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Color, Float};

impl<SPACE> Serialize for Color<SPACE> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple_struct("Vec3", 3)?;
        let (v1, v2, v3) = self.tuple();
        tup.serialize_field(&v1)?;
        tup.serialize_field(&v2)?;
        tup.serialize_field(&v3)?;
        tup.end()
    }
}

impl<'de, SPACE> Deserialize<'de> for Color<SPACE> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple_struct("Vec3", 3, Vec3Visitor(PhantomData))
    }
}

struct Vec3Visitor<SPACE>(PhantomData<SPACE>);

impl<'de, SPACE> Visitor<'de> for Vec3Visitor<SPACE> {
    type Value = Color<SPACE>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Formatter::write_str(f, "tuple struct Vec3")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        const EXP: &str = "tuple struct Vec3 with 3 elements";

        let field0 = SeqAccess::next_element::<Float>(&mut seq)?
            .ok_or_else(|| Error::invalid_length(0, &EXP))?;
        let field1 = SeqAccess::next_element::<Float>(&mut seq)?
            .ok_or_else(|| Error::invalid_length(1, &EXP))?;
        let field2 = SeqAccess::next_element::<Float>(&mut seq)?
            .ok_or_else(|| Error::invalid_length(2, &EXP))?;

        Ok(Color::new(field0, field1, field2))
    }
}
