use crate::W;
use serde::{de::Visitor, Deserialize, Serialize};
use url::Url;

impl Serialize for W<Url> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for W<Url> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct WUrlVisitor;

        impl<'de> Visitor<'de> for WUrlVisitor {
            type Value = W<Url>;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error, 
            {
                Ok(
                    W(
                        Url::parse(v).map_err(|e| E::custom(e.to_string()))?
                    )
                )
            }
        }

        deserializer.deserialize_str(WUrlVisitor)
    }
}