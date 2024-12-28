use crate::html_to_urls::Tree;
use serde::{ser::SerializeMap, Serialize};
use serde_json::Value;
use std::{collections::HashMap, error::Error};
use url::Url;

type GenError = Box<dyn Error>;

#[derive(Debug, Clone)]
pub struct Indexer(pub HashMap<Url, (usize, usize, String)>); // Url: depth, frequency, html
impl Indexer {
    pub async fn new(tree: Vec<&Tree>) -> Result<Indexer, GenError> {
        let mut json: Value = serde_json::from_str(serde_json::to_string_pretty(&tree)?.as_str())?;

        let mut url_map: HashMap<String, (usize, usize, String)> = HashMap::new();

        if let Some(array) = json.as_array_mut() {
            for item in array.iter_mut() {
                item.as_object_mut().map(|obj| {
                    obj.remove("subtree"); // Removes the "subtree" field

                    if let Some(url) = obj.get("url").and_then(Value::as_str) {
                        let html = obj.get("html")
                            .and_then(Value::as_str)
                            .unwrap_or("");
                        let depth =
                            obj.get("depth")
                                .and_then(Value::as_u64)
                                .unwrap_or(usize::MAX as u64) as usize;

                        // Update the entry in the HashMap: Min depth and frequency
                        let entry = url_map.entry(url.to_string()).or_insert((usize::MAX, 0, html.to_string()));
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
        };

        Ok(Indexer(url_map.iter().map(|x| (Url::parse(x.0).unwrap(), (x.1.0, x.1.1, x.1.2.clone()))).collect()))
    }

    // pub async fn new(tree: Vec<&Tree>) -> Result<Indexer, GenError> {
    //     let mut json: Value = serde_json::from_str(serde_json::to_string_pretty(&tree)?.as_str())?;

    //     let mut url_map: HashMap<String, (usize, usize/* , String*/)> = HashMap::new();

    //     if let Some(array) = json.as_array_mut() {
    //         for item in array.iter_mut() {
    //             item.as_object_mut().map(|obj| {
    //                 obj.remove("subtree"); // Removes the "subtree" field

    //                 if let Some(url) = obj.get("url").and_then(Value::as_str) {
    //                     // let html = obj.get("html")
    //                     //     .and_then(Value::as_str)
    //                     //     .unwrap_or("");
    //                     let depth =
    //                         obj.get("depth")
    //                             .and_then(Value::as_u64)
    //                             .unwrap_or(usize::MAX as u64) as usize;

    //                     // Update the entry in the HashMap: Min depth and frequency
    //                     let entry = url_map.entry(url.to_string()).or_insert((usize::MAX, 0, /*html.to_string()*/));
    //                     entry.0 = entry.0.min(depth);
    //                     entry.1 += 1;
    //                 }
    //             });
    //         }
    //     }

    //     let mut result = Vec::new();
    //     for (url, (min_depth, frequency/* , html*/)) in url_map.iter() {
    //         let merged_entry = serde_json::json!({
    //             "url": url,
    //             "depth": min_depth,
    //             "frequency": frequency,
    //             // "html": html
    //         });
    //         result.push(merged_entry);
    //     };

    //     Ok(Indexer(url_map.iter().map(|x| (Url::parse(x.0).unwrap(), (x.1.0, x.1.1/*, x.1.2.clone()*/))).collect()))
    // }
}

impl Serialize for Indexer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        
        for (url, (first, second, html)) in &self.0 {
            // Serialize the value as a temporary struct-like map
            let value = serde_json::json!({
                "depth": first,
                "frequency": second,
                "html": html
            });
            map.serialize_entry(url.as_str(), &value)?;
        }

        map.end()
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