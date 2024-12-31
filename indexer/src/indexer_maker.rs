use crate::html_to_urls::Tree;
use serde::{
    de::{self, MapAccess, Visitor}, ser::{SerializeMap, SerializeSeq}, Deserialize, Deserializer, Serialize, Serializer
};
use serde_json::{json, Value};
use std::{
    collections::{
        hash_map::{IntoIter, Iter},
        HashMap,
    },
    error::Error,
    fmt,
};
use url::Url;

type GenError = Box<dyn Error>;

#[derive(Debug, Clone)]
pub struct Indexer(pub HashMap<Url, (usize, usize, String)>); // Url: depth, frequency, html
impl Indexer {
    pub async fn new(tree: Vec<&Tree>) -> Result<Indexer, GenError> {
        let bind = serde_json::to_string_pretty(&tree)?;
        std::fs::write("lkdajghfuy", bind.as_str()).unwrap();
        let mut json: Value = serde_json::from_str(bind.as_str())?;
        std::fs::write("somethiung", format!("{}", json)).unwrap();
        let mut url_map: HashMap<String, (usize, usize, String)> = HashMap::new();

        if let Some(array) = json.as_array_mut() {
            for item in array.iter_mut() {
                item.as_object_mut().map(|obj| {
                    obj.remove("subtree"); // Removes the "subtree" field

                    if let Some(url) = obj.get("url").and_then(Value::as_str) {
                        let html = obj
                            .get("html")
                            .and_then(|x| {
                                let y = format!("{}", x);
                                Some(y)
                            })
                            .unwrap_or("".to_string());
                        let depth =
                            obj.get("depth")
                                .and_then(Value::as_u64)
                                .unwrap_or(usize::MAX as u64) as usize;

                        // Update the entry in the HashMap: Min depth and frequency
                        let entry = url_map.entry(url.to_string()).or_insert((
                            usize::MAX,
                            0,
                            html.to_string(),
                        ));
                        entry.0 = entry.0.min(depth);
                        entry.1 += 1;
                    }
                });
            }
        }

        let mut result = Vec::new();
        for (url, (min_depth, frequency, html)) in url_map.iter() {
            let merged_entry = serde_json::json!({
                "url": url,
                "depth": min_depth,
                "frequency": frequency,
                "html": html
            });
            result.push(merged_entry);
        }

        Ok(Indexer(
            url_map
                .iter()
                .map(|x| (Url::parse(x.0).unwrap(), (x.1 .0, x.1 .1, x.1 .2.clone())))
                .collect(),
        ))
    }

    pub fn iter(&self) -> Iter<Url, (usize, usize, String)> {
        self.0.iter()
    }
}

impl IntoIterator for Indexer {
    type Item = (Url, (usize, usize, String));

    type IntoIter = IntoIter<Url, (usize, usize, String)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Serialize for Indexer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_seq(Some(self.0.len()))?;

        // let mut max_depth = 0;

        let data: Vec<_> = self
            .0
            .iter()
            .map(|(url, (first, second, html))| {
                // if *first > max_depth {
                //     max_depth = *first
                // }
                let url = url.to_string();
                serde_json::json!({
                    "url": url,
                    "depth": first,
                    "frequency": second,
                    "html": html
                })
            })
            .collect();

        // let meta = serde_json::json!(
        //     {"depth": max_depth}
        // );

        map.serialize_element(&data)?;
        // map.serialize_entry("meta", &meta)?;
        map.end()
    }
}


// impl Serialize for Indexer {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(2))?; // We are going to add "data" and "meta"
//         let mut max_depth = 0;
//         // Serialize the "data" object
//         let mut data_map = serializer.serialize_map(Some(self.0.len()))?;
//         for (url, (depth, frequency, html)) in &self.0 {
//             if max_depth < *depth {
//                 max_depth = *depth
//             }
//             data_map.serialize_entry(url.as_str(), &json!({
//                 "depth": depth,
//                 "frequency": frequency,
//                 "html": html
//             }))?;
//         }
//         data_map.end()?;

//         // Serialize the "meta" object
//         let meta = json!({"depth": max_depth});

//         map.serialize_entry("meta", &meta)?;

//         map.end()
//     }
// }

// impl Serialize for Indexer {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // Create a map for the final structure
//         let mut map = serializer.serialize_map(Some(2))?; // We have "data" and "meta"

//         // Serialize the "data" object
//         {
//             let mut data_map = serializer.serialize_map(Some(self.0.len()))?;
//             for (url, (depth, frequency, html)) in &self.0 {
//                 data_map.serialize_entry(url.as_str(), &json!({
//                     "depth": depth,
//                     "frequency": frequency,
//                     "html": html
//                 }))?;
//             }
//             data_map.end()?;
//         }

//         // // Serialize the "meta" object
//         // let meta = Meta {
//         //     version: "1.0".to_string(),
//         //     timestamp: "2024-12-29T00:00:00Z".to_string(),
//         // };

//         // map.serialize_entry("meta", &meta)?;

//         // End serialization
//         map.end()
//     }
// }

// impl<'de> serde::Deserialize<'de> for Indexer {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct IndexerVisitor;

//         impl<'de> Visitor<'de> for IndexerVisitor {
//             type Value = Indexer;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("a map where keys are URLs and values are maps with 'depth', 'frequency', and 'html' fields")
//             }

//             fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
//             where
//                 M: MapAccess<'de>,
//             {
//                 let mut index = HashMap::new();

//                 while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
//                     // Extract fields from the value
//                     let depth = value
//                         .get("depth")
//                         .ok_or_else(|| de::Error::missing_field("depth"))?
//                         .as_u64()
//                         .ok_or_else(|| de::Error::custom("depth must be a positive integer"))?
//                         as u32;

//                     let frequency = value
//                         .get("frequency")
//                         .ok_or_else(|| de::Error::missing_field("frequency"))?
//                         .as_u64()
//                         .ok_or_else(|| de::Error::custom("frequency must be a positive integer"))?
//                         as u32;

//                     let html = value
//                         .get("html")
//                         .ok_or_else(|| de::Error::missing_field("html"))?
//                         .as_str()
//                         .ok_or_else(|| de::Error::custom("html must be a string"))?
//                         .to_string();

//                     index.insert(
//                         Url::parse(key.as_str()).unwrap(),
//                         (depth as usize, frequency as usize, html),
//                     );
//                 }

//                 Ok(Indexer(index))
//             }

//             fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::SeqAccess<'de>,
//             {
//                 let seq = seq;
//                 // Err(serde::de::Error::invalid_type(serde::de::Unexpected::Seq, &self))

                
//             }

//             // fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Bool(v), &self))
//             // }

//             // fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_i64(v as i64)
//             // }

//             // fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_i64(v as i64)
//             // }

//             // fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_i64(v as i64)
//             // }

//             // fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Signed(v), &self))
//             // }

//             // fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     let mut buf = [0u8; 58];
//             //     let mut writer = serde::format::Buf::new(&mut buf);
//             //     fmt::Write::write_fmt(&mut writer, format_args!("integer `{}` as i128", v)).unwrap();
//             //     Err(serde::de::Error::invalid_type(
//             //         serde::de::Unexpected::Other(writer.as_str()),
//             //         &self,
//             //     ))
//             // }

//             // fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_u64(v as u64)
//             // }

//             // fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_u64(v as u64)
//             // }

//             // fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_u64(v as u64)
//             // }

//             // fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Unsigned(v), &self))
//             // }

//             // fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     let mut buf = [0u8; 57];
//             //     let mut writer = serde::format::Buf::new(&mut buf);
//             //     fmt::Write::write_fmt(&mut writer, format_args!("integer `{}` as u128", v)).unwrap();
//             //     Err(serde::de::Error::invalid_type(
//             //         serde::de::Unexpected::Other(writer.as_str()),
//             //         &self,
//             //     ))
//             // }

//             // fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_f64(v as f64)
//             // }

//             // fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Float(v), &self))
//             // }

//             // fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_str(v.encode_utf8(&mut [0u8; 4]))
//             // }

//             // fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Str(v), &self))
//             // }

//             // fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_str(v)
//             // }

//             // fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_str(&v)
//             // }

//             // fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Bytes(v), &self))
//             // }

//             // fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_bytes(v)
//             // }

//             // fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     self.visit_bytes(&v)
//             // }

//             // fn visit_none<E>(self) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Option, &self))
//             // }

//             // fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//             // where
//             //     D: Deserializer<'de>,
//             // {
//             //     let _ = deserializer;
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Option, &self))
//             // }

//             // fn visit_unit<E>(self) -> Result<Self::Value, E>
//             // where
//             //     E: serde::de::Error,
//             // {
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Unit, &self))
//             // }

//             // fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//             // where
//             //     D: Deserializer<'de>,
//             // {
//             //     let _ = deserializer;
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::NewtypeStruct, &self))
//             // }

//             // fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
//             // where
//             //     A: serde::de::EnumAccess<'de>,
//             // {
//             //     let _ = data;
//             //     Err(serde::de::Error::invalid_type(serde::de::Unexpected::Enum, &self))
//             // }
//         }

//         deserializer.deserialize_map(IndexerVisitor)
//     }
// }

impl<'de> Deserialize<'de> for Indexer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> 
    {
        // Deserializing the array of objects into the HashMap<Url, (usize, usize, String)>
        let vec: Vec<HashMap<String, Value>> = Deserialize::deserialize(deserializer)?;

        let mut map: HashMap<Url, (usize, usize, String)> = HashMap::new();

        for entry in vec {
            if let (Some(url_str), Some(depth), Some(frequency), Some(html)) = (
                entry.get("url").and_then(|v| v.as_str()),
                entry.get("depth").and_then(|v| v.as_u64()),
                entry.get("frequency").and_then(|v| v.as_u64()),
                entry.get("html").and_then(|v| v.as_str()),
            ) {
                let url = Url::parse(url_str).unwrap();
                map.insert(
                    url,
                    (depth as usize, frequency as usize, html.to_string()),
                );
            }
        }

        Ok(Indexer(map))
    }
}

impl FromIterator<(Url, (usize, usize, String))> for Indexer {
    fn from_iter<T: IntoIterator<Item = (Url, (usize, usize, String))>>(iter: T) -> Self {
        Indexer(iter.into_iter().collect::<HashMap<Url, (usize, usize, String)>>())
    }
}

#[test]
fn test() {
    let mut x = HashMap::new();
    x.insert("string", (5, 5));
    x.insert("eepy", (123, 456));

    let json = serde_json::to_string(&x).unwrap();

    println!("{}", json);
}

#[test]
fn test2() {}
