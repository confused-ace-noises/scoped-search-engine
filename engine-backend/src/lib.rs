use rocket::figment::util::vec_tuple_map::deserialize;
use rocket::{post, serde::json::Json};
use rocket::form::Form;
use indexer::{html_to_urls::Tree, indexer_maker::Indexer};
use regex::{self, Regex};
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{self, Deserialize, Deserializer, Serialize};
use serde_json::{self, json};
use sort_results::matches::{Matches, Page, Sorter};
use sort_results::{Pages, UserMod, UserModifier, UserParams};
use std::collections::HashMap;
use std::fmt;
use std::{error::Error, ops::Index, slice::SliceIndex};
use tokio;
use url::{self, Url};

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaData {
    max_depth: usize,
}

#[derive(Debug)]
pub struct Data {
    data: Indexer,
    meta: MetaData,
}
impl Data {
    pub fn weed_out_too_deep(self, depth: usize) -> Indexer {
        let max_depth = self.meta.max_depth;
        if depth == max_depth {
            self.data
        } else {
            self.data
                .into_iter()
                .filter(|x| !(x.1 .0 > depth))
                .collect::<Indexer>()
        }
    }
}
impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_map(Some(2))?;
        state.serialize_entry("data", &self.data.0.clone().into_iter().map(|s| {
            json! {
                {
                    "depth": s.1.0,
                    "frequency": s.1.1,
                    "html": s.1.2,
                    "url": s.0.to_string()
                }
            }
        }).collect::<Vec<_>>())?;
        state.serialize_entry("meta", &self.meta)?;
        state.end()
    }
}

// impl<'de> Deserialize<'de> for Data {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         struct RawData {
//             data: Vec<Vec<RawDataItem>>,
//             meta: MetaData,
//         }

//         #[derive(Deserialize)]
//         struct RawDataItem {
//             depth: usize,
//             frequency: usize,
//             html: String,
//             url: String,
//         }

//         let raw: RawData = RawData::deserialize(deserializer)?;

//         // Flatten the nested data array
//         let flattened_data: Vec<RawDataItem> = raw
//             .data
//             .into_iter()
//             .flat_map(|inner| inner.into_iter())
//             .collect();

//         // Convert the flattened data into the format `Indexer` expects
//         let indexer_data: Vec<(String, (usize, usize, String))> = flattened_data
//             .into_iter()
//             .map(|item| (item.url, (item.depth, item.frequency, item.html)))
//             .collect();

//         // Create the Indexer
//         let indexer = Indexer(
//             indexer_data
//                 .into_iter()
//                 .map(|x| (Url::parse(&x.0).unwrap(),(x.1.0, x.1.1, x.1.2)))
//                 .collect::<std::collections::HashMap<_, _>>(),
//         );

//         Ok(Data {
//             data: indexer,
//             meta: raw.meta,
//         })
//     }
// }

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataVisitor;

        impl<'de> Visitor<'de> for DataVisitor {
            type Value = Data;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map containing 'data' and 'meta'")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut data = None;
                let mut meta = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "data" => {
                            let raw_data: Vec<serde_json::Value> = map.next_value()?;
                            let mut indexer_data = HashMap::new();

                            for item in raw_data {
                                if let (Some(url), Some(depth), Some(frequency), Some(html)) = (
                                    item.get("url").and_then(|v| v.as_str()),
                                    item.get("depth").and_then(|v| v.as_u64()),
                                    item.get("frequency").and_then(|v| v.as_u64()),
                                    item.get("html").and_then(|v| v.as_str()),
                                ) {
                                    indexer_data.insert(
                                        url.to_string(),
                                        (depth as usize, frequency as usize, html.to_string()),
                                    );
                                } else {
                                    return Err(de::Error::custom(
                                        "Invalid structure for 'data' item",
                                    ));
                                }
                            }

                            data = Some(Indexer(indexer_data.into_iter().map(|x| (Url::parse(&x.0).unwrap(), (x.1.0, x.1.1, x.1.2))).collect::<HashMap<_, _>>()));
                        }
                        "meta" => {
                            meta = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(&key, &["data", "meta"])),
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let meta = meta.ok_or_else(|| de::Error::missing_field("meta"))?;

                Ok(Data { data, meta })
            }
        }

        deserializer.deserialize_map(DataVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct SearchQuery {
    pub starting_url: String,
    pub query: Vec<String>, // Use appropriate data types for the query, e.g., String for simple text search
    pub modifiers: Vec<(String, i8, f64, u8)>, // url, (-1: Penal, 0: Ban, 1: Boost), value, (0: string, 1: regex)
    pub params: (f64, f64, f64),
    pub max_depth: usize,
    pub query_type: u8, // 0: strings sensitive, 1: strings insensitive, 2: regexes
}

#[test]
fn test2() {
    let json = r#"{
    "starting_url": "https://askiiart.net/",
    "query": [
        "askiiart"
    ],
    "modifiers": [],
    "params": [
        -0.7,
        1.7,
        2.5
    ],
    "max_depth": 1,
    "query_type": 0
}"#;
    let deserde: Result<SearchQuery, serde_json::Error> = serde_json::from_str(json);
    println!("{:#?}", deserde);
}

#[test]
fn test1() {
    let json = r#"
        {
            "data": [
                {
                    "depth": 1,
                    "frequency": 1,
                    "html": "",
                    "url": "https://example.org/"
                }
            ],

            "meta": {
                "max_depth": 1
            }
        }
    "#;

    let data: Data = serde_json::from_str(&json).unwrap();
    println!("{data:#?}");
}

// #[post("/api/search_any", format = "application/json", data = "<query>")]
// pub async fn search_any(query: Json<SearchQuery>) -> Json<Pages> {
#[post("/api/search_any", format = "application/json", data = "<query>")]
pub async fn search_any(query: String) -> Json<Pages> {
    
    println!("{}", query);

    let query: SearchQuery = serde_json::from_str(&query).unwrap();

    println!("{:#?}", query);

    let starting_url = query.starting_url;
    let query_query = query.query;
    let userparams = UserParams {
        depth_w: query.params.0,
        frequency_w: query.params.1,
        n_matches_w: query.params.2,
    };
    let mods = fix_modifiers(query.modifiers);
    let max_depth = query.max_depth;

    let result = match query.query_type {
        0 => search_normal_case_sensitive(
            starting_url,
            query_query,
            userparams,
            mods,
            max_depth,
        ).await.unwrap(),
        1 => search_normal_case_insensitive(
            starting_url,
            query_query,
            userparams,
            mods,
            max_depth,
        ).await.unwrap(),
        2 => search_regex(
            starting_url,
            query_query.iter().map(|x| Regex::new(&x)).filter(|x| x.is_ok()).map(|x| x.unwrap()).collect(),
            userparams,
            mods,
            max_depth,
        ).await.unwrap(),
        _ => unimplemented!(),
    };

    Json(Pages(result))
}

fn fix_modifiers(input: Vec<(String, i8, f64, u8)>) -> Vec<UserModifier> {
    input.iter().map(|modifier| -> Result<UserModifier, Box<dyn Error>>{
        match modifier.1 {
            -1 => match modifier.3 {
                0 => Ok(UserModifier { value_modifier: UserMod::Penal(modifier.2), pattern: sort_results::Patt::String(modifier.0.clone()) }),
                1 => Ok(UserModifier { value_modifier: UserMod::Penal(modifier.2), pattern: sort_results::Patt::Regex(Regex::new(&modifier.0)?) }),
                _ => unimplemented!()
            },
            0 => match modifier.3 {
                0 => Ok(UserModifier { value_modifier: UserMod::Ban, pattern: sort_results::Patt::String(modifier.0.clone()) }),
                1 => Ok(UserModifier { value_modifier: UserMod::Ban, pattern: sort_results::Patt::Regex(Regex::new(&modifier.0)?) }),
                _ => unimplemented!()
            },
            1 => match modifier.3 {
                0 => Ok(UserModifier { value_modifier: UserMod::Boost(modifier.2), pattern: sort_results::Patt::String(modifier.0.clone()) }),
                1 => Ok(UserModifier { value_modifier: UserMod::Boost(modifier.2), pattern: sort_results::Patt::Regex(Regex::new(&modifier.0)?) }),
                _ => unimplemented!()
            },
            _ => unimplemented!()
        }
    }).filter(|x| !x.is_err()).map(|x| {x.unwrap()}).collect() 
}
pub async fn make_index(starting_url: String, depth: usize) -> Result<(), Box<dyn Error>> {
    let start = Url::parse(&starting_url)?;

    let raw_tree = Tree::new(start, depth).await?;

    let mut tree = Vec::new();
    raw_tree.flatten(&mut tree);

    let indexed = Indexer::new(tree).await?;

    std::fs::write(
        // format!("url_map_icepick_json_{starting_url}.txt"),
        // "url_map_icepick_json_test.txt",
        "test",
        serde_json::to_string_pretty(&indexed)?.as_str(),
    )?;

    Ok(())
}

pub async fn search_regex(
    starting_url: String,
    query: Vec<Regex>,
    params: UserParams,
    modifiers: Vec<UserModifier>,
    depth: usize,
) -> Result<Vec<Page>, Box<dyn Error>> {
    // get data
    let x = {
        if let Ok(inner) =
            // std::fs::read(format!("url_map_icepick_json_{starting_url}.txt")) {
            std::fs::read("url_map_icepick_json_test.txt")
        {
            let x: Data = serde_json::from_str(
                inner
                    .into_iter()
                    .map(|x| x as char)
                    .collect::<String>()
                    .as_str(),
            )?;

            if x.meta.max_depth <= depth
            /*x.depth <= depth*/
            {
                x
            } else {
                let path = format!("url_map_icepick_json_test.txt");
                make_index(starting_url, depth).await?;
                let x: Data = serde_json::from_str(
                    std::fs::read(path)?
                        .iter()
                        .map(|x| *x as char)
                        .collect::<String>()
                        .as_str(),
                )?;
                x
            }
        } else {
            let path = format!("url_map_icepick_json_{starting_url}.txt");
            make_index(starting_url, depth).await?;
            let x: Data = serde_json::from_str(
                std::fs::read(path)?
                    .iter()
                    .map(|x| *x as char)
                    .collect::<String>()
                    .as_str(),
            )?;
            x
        }
    };

    let indexer = x.weed_out_too_deep(depth);

    let pages = indexer.sort_regex(query, params, modifiers);

    Ok(pages)
}

pub async fn search_normal_case_sensitive(
    starting_url: String,
    query: Vec<String>,
    params: UserParams,
    modifiers: Vec<UserModifier>,
    depth: usize,
) -> Result<Vec<Page>, Box<dyn Error>> {
    let x = {
        if let Ok(inner) =
            // std::fs::read(format!("url_map_icepick_json_{starting_url}.txt")) {
            std::fs::read_to_string("test")
        {
            let data_test = serde_json::to_string_pretty(&Data {
                data: Indexer({let mut x = HashMap::new(); x.insert(Url::parse("https://example.org/").unwrap(), (1 as usize,2 as usize,String::new())); x}),
                meta: MetaData { max_depth: 1 },
            }).unwrap();

            println!("12: {data_test}");

            let x: Data = serde_json::from_str(&inner)?;

            println!("123: {x:#?}");

            if x.meta.max_depth <= depth
            /*x.depth <= depth*/
            {
                x
            } else {
                let path = format!("test");
                make_index(starting_url, depth).await?;
                let x: Data = serde_json::from_str(
                    std::fs::read(path)?
                        .iter()
                        .map(|x| *x as char)
                        .collect::<String>()
                        .as_str(),
                )?;
                x
            }
        } else {
            let path = format!("test");
            make_index(starting_url, depth).await?;
            let x: Data = serde_json::from_str(
                std::fs::read(path)?
                    .iter()
                    .map(|x| *x as char)
                    .collect::<String>()
                    .as_str(),
            )?;
            x
        }
    };

    let indexer = x.weed_out_too_deep(depth);

    let pages = indexer.sort_strings_sensitive(query, params, modifiers);

    Ok(pages)
}

pub async fn search_normal_case_insensitive(
    starting_url: String,
    query: Vec<String>,
    params: UserParams,
    modifiers: Vec<UserModifier>,
    depth: usize,
) -> Result<Vec<Page>, Box<dyn Error>> {
    let x = {
        if let Ok(inner) = std::fs::read(format!("url_map_icepick_json_{starting_url}.txt")) {
            let x: Data = serde_json::from_str(
                inner
                    .into_iter()
                    .map(|x| x as char)
                    .collect::<String>()
                    .as_str(),
            )?;

            if x.meta.max_depth <= depth
            /*x.depth <= depth*/
            {
                x
            } else {
                let path = format!("url_map_icepick_json_{starting_url}.txt");
                make_index(starting_url, depth).await?;
                let x: Data = serde_json::from_str(
                    std::fs::read(path)?
                        .iter()
                        .map(|x| *x as char)
                        .collect::<String>()
                        .as_str(),
                )?;
                x
            }
        } else {
            let path = format!("url_map_icepick_json_{starting_url}.txt");
            make_index(starting_url, depth).await?;
            let x: Data = serde_json::from_str(
                std::fs::read(path)?
                    .iter()
                    .map(|x| *x as char)
                    .collect::<String>()
                    .as_str(),
            )?;
            x
        }
    };

    let indexer = x.weed_out_too_deep(depth);

    let pages = indexer.sort_strings_insensitive(query, params, modifiers);

    Ok(pages)
}

#[tokio::test]
async fn test() {
    let pages: Vec<Page> = search_normal_case_sensitive(
        "https://askiiart.net/".to_string(),
        vec!["askiiart".to_string()],
        UserParams::default(),
        vec![],
        1,
    )
    .await
    .unwrap();
    std::fs::write("results_wow", format!("{pages:#?}")).unwrap();
    let urls = pages
        .iter()
        .map(|x: &Page| x.url.to_string())
        .collect::<Vec<String>>();
    std::fs::write("results_wow2", format!("{urls:#?}")).unwrap();
}

// ...........
