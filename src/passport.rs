use nom::{
    IResult,
    bytes::complete::{take_until, take_till, tag},
};
use serde::{de, Serialize, Deserialize};
use serde::de::{Error, Visitor};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    pub fn parse(input: &str) -> IResult<&str, Passport, de::value::Error> {
        let mut deserializer = PassportDeserializer{
            input,
            value: None,
        };
        let map_access = de::value::MapAccessDeserializer::new(&mut deserializer);
        let t = Passport::deserialize(map_access).map_err(nom::Err::Error)?;
        Ok((deserializer.input, t))
    }

    fn validate_height(h: &str) -> anyhow::Result<()> {
        let idx = h.find(|c: char| !c.is_digit(10))
            .ok_or_else(|| anyhow::Error::msg("height must have unit"))?;
        let (num, unit) = h.split_at(idx);
        let num = i32::from_str(num)?;

        match unit {
            "cm" => {
                if num < 150 || num > 193 {
                    Err(anyhow::Error::msg("invalid height num"))?;
                }
            },
            "in" => {
                if num < 59 || num > 76 {
                    Err(anyhow::Error::msg("invalid height num"))?;
                }
            },
            x => Err(anyhow::Error::msg(format!("unexpected unit {}", x)))?,
        };

        Ok(())
    }

    fn validate_hair_colour(h: &str) -> anyhow::Result<()> {
        if h.len() != 7 {
            Err(anyhow::Error::msg("hcl wrong length"))?;
        }

        let mut chars = h.chars();
        if chars.next() != Some('#') {
            Err(anyhow::Error::msg("hcl must start with a"))
        } else {
            for char in chars {
                if ('0'..='9').contains(&char) {
                    continue
                }

                if ('a'..='f').contains(&char) {
                    continue
                }

                Err(anyhow::Error::msg("invalid hcl"))?;
            }

            Ok(())
        }
    }

    fn validate_eye_colour(e: &str) -> anyhow::Result<()> {
        match e {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
            c => Err(anyhow::Error::msg(format!("invalid ecl: {}", c)))
        }
    }

    fn validate_passport_id(i: &str) -> anyhow::Result<()> {
        if i.len() != 9 {
            Err(anyhow::Error::msg("pid must be 9 characters"))?;
        }

        if !i.chars().all(|c| c.is_digit(10)) {
            Err(anyhow::Error::msg("pid must be numbers"))?;
        }

        Ok(())
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.byr < 1920 || self.byr > 2002 {
            Err(anyhow::Error::msg("byr must be between 1920 and 2002"))
        } else if self.iyr < 2010 || self.iyr > 2020 {
            Err(anyhow::Error::msg("iyr must be between 2010 and 2020"))
        } else if self.eyr < 2020 || self.eyr > 2030 {
            Err(anyhow::Error::msg("eyr is invalid"))
        } else {
            Passport::validate_height(&self.hgt)?;
            Passport::validate_hair_colour(&self.hcl)?;
            Passport::validate_eye_colour(&self.ecl)?;
            Passport::validate_passport_id(&self.pid)?;
            Ok(())
        }
    }
}

struct FieldDeserializer<'de> {
    input: &'de str,
}

impl<'de> de::Deserializer<'de> for FieldDeserializer<'de> {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_str(self.input)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_bool(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_i8(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_i16(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_i32(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_i64(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_u8(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_u16(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_u32(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_u64(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_f32(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_f64(FromStr::from_str(self.input).map_err(|e| de::value::Error::custom(&e))?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if self.input.len() != 1 {
            Err(de::value::Error::custom("char must be length 1"))
        } else {
            visitor.visit_char(self.input.as_bytes()[0] as char)
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_str(self.input)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_string(String::from(self.input))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_bytes(self.input.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_byte_buf(self.input.as_bytes().to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        Err(de::value::Error::custom("seq not supported"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        Err(de::value::Error::custom("tuple not supported"))
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        Err(de::value::Error::custom("tuple not supported"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        Err(de::value::Error::custom("map not supported"))
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        Err(de::value::Error::custom("struct not supported"))
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        Err(de::value::Error::custom("enum not supported"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_str(self.input)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_str(self.input)
    }
}

struct PassportDeserializer<'de> {
    input: &'de str,
    value: Option<&'de str>,
}

impl<'de> de::MapAccess<'de> for PassportDeserializer<'de> {
    type Error = de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as de::DeserializeSeed<'de>>::Value>, Self::Error> where
        K: de::DeserializeSeed<'de> {
        if self.input.is_empty() {
            return Ok(None);
        }

        let (input, (key, value)) = parse_key_pair(self.input).map_err(Error::custom)?;
        self.input = input;
        self.value = Some(value);
        seed.deserialize(FieldDeserializer{input: key}).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<<V as de::DeserializeSeed<'de>>::Value, Self::Error> where
        V: de::DeserializeSeed<'de> {
        if let Some(value) = self.value.take() {
            seed.deserialize(FieldDeserializer{input: value})
        } else {
            Err(de::value::Error::custom("called next_value without next_key"))
        }
    }
}

fn skip_whitespace(input: &str) -> &str {
    input.trim_start()
}

fn parse_key_pair(input: &str) -> IResult<&str, (&str, &str)> {
    let input = skip_whitespace(input);
    let (input, key) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, value) = take_till(|c: char| c.is_whitespace())(input)?;
    let input = skip_whitespace(input);
    Ok((input, (key, value)))
}

pub fn split_blocks(input: &str) -> impl Iterator<Item=&str> {
    input.split("\n\n")
}

pub fn parse_list(input: &str) -> Result<Vec<Passport>, nom::Err<de::value::Error>> {
    split_blocks(input)
        .map(Passport::parse)
        .map(|r| r.and_then(|(i, p)| {
            if i.is_empty() {
                Ok(p)
            } else {
                Err(nom::Err::Error(de::value::Error::custom("extra input after passport")))
            }
        }))
        .collect()
}
