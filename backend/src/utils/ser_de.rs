use crate::W;
use serde::Serialize;
use url::Url;

impl Serialize for W<Url> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
