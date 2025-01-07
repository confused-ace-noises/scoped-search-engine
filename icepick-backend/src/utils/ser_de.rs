use crate::{main_logic::{eval::ScorePage, scoring::ScoredFlatTree, user_options::{Query, UserModType, UserModifiers}}, utils::searcher::Searcher, W};
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
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

impl<'de> Deserialize<'de> for UserModifiers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct UserModifiersVisitor;
        impl<'de> Visitor<'de> for UserModifiersVisitor {
            type Value = UserModifiers;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting a triplet of values describing a user modifier")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>, 
            {
                let mut mod_type: usize = 0;
                let mut amount: f64 = 0.0;
                let mut search: &str = "";
                let mut search_type: usize = 0;

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "mod_type" => {
                            let value: usize = map.next_value()?;
                            mod_type = value;
                        },
                        "amount" => {
                            let value: f64 = map.next_value()?;
                            amount = value;
                        },
                        "search" => {
                            let value: &str = map.next_value()?;
                            search = value;
                        },
                        "search_type" => {
                            let value: usize = map.next_value()?;
                            search_type = value;
                        },
                        _ => panic!() // TODO: fix this
                    }
                }

                Ok(UserModifiers { 
                    modification_type: {
                        match mod_type {
                            0 => UserModType::Boost(amount),
                            1 => UserModType::Penalize(amount),
                            2 => UserModType::Ban,
                            _ => panic!() // TODO: fix this
                        }
                    }, 
                    searcher: {
                        match search_type {
                            0 => Searcher::SensitiveString(search.to_string()),
                            1 => Searcher::InsensitiveString(search.to_string()),
                            2 => Searcher::from_regex(search).unwrap(),
                            _ => panic!() // TODO: fix this
                        }
                    } 
                })
            }
        }

        deserializer.deserialize_map(UserModifiersVisitor)    
    }
}

impl<'de> Deserialize<'de> for Query {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct QueryVisitor;

        impl<'de> Visitor<'de> for QueryVisitor {
            type Value = Query;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expected 2 values describing query type and query")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>, 
            {
                let mut query_type = 0;
                let mut query = "";

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "query_type" => {
                            let value: usize = map.next_value()?;
                            query_type = value;
                        },

                        "query" => {
                            let value: &str = map.next_value()?;
                            query = value;
                        }

                        _ => panic!() // TODO fix this
                    }
                }

                Ok(match query_type {
                    0 => Query::SensitiveString(query.to_string()),
                    1 => Query::InsensitiveString(query.to_string()),
                    2 => Query::Regex(vec![query.to_string()]),
                    _ => panic!(),
                })
            }
        }

        deserializer.deserialize_map(QueryVisitor)
    }
}

impl Serialize for ScoredFlatTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_map(Some(self.0.len()))?;

        for inner in self.0.iter() {
            state.serialize_entry(&inner.0, &inner.1)?;
        }

        state.end()
    }
}

impl<'de> Deserialize<'de> for ScoredFlatTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct ScoredFlatTreeVisitor;

        impl<'de> Visitor<'de> for ScoredFlatTreeVisitor {
            type Value = ScoredFlatTree;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expected a key value map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>, 
            {
                let mut vec: Vec<(W<Url>, ScorePage)> = Vec::new();

                while let Some(inner) = map.next_entry()? {
                    vec.push(inner);
                }

                Ok(vec.into())
            }
        }
        deserializer.deserialize_map(ScoredFlatTreeVisitor)
    }
}

#[test]
fn test() {
    let json = r#"{"mod_type": 0, "amount": 7.0, "contains": "something", "search_type":0}"#;

    let thing: UserModifiers = serde_json::from_str(&json).unwrap();

    println!("{:#?}", thing)
}