use indexer::{html_to_urls::Tree, indexer_maker::Indexer};
use regex::{self, Regex};
use serde::de::{self, MapAccess, Visitor};
use serde::{self, Deserialize, Deserializer};
use serde_json;
use sort_results::matches::{Matches, Page, Sorter};
use sort_results::{UserMod, UserModifier, UserParams};
use std::fmt;
use std::{error::Error, ops::Index, slice::SliceIndex};
use url::{self, Url};
use tokio;

#[derive(Debug, Deserialize)]
pub struct MetaData {
    max_depth: usize,
}

#[derive(Deserialize)]
pub struct Data {
    data: Indexer,
    meta: MetaData,
} impl Data {
    pub fn weed_out_too_deep(self, depth: usize) -> Indexer {
        let max_depth = self.meta.max_depth;
        if depth == max_depth {
            self.data
        } else {
            self.data.into_iter().filter(|x| !(x.1.0 > depth)).collect::<Indexer>()
        }
    }
}

// #[derive(Deserialize)]
// pub struct Data {
//     data: Indexer,
//     depth: usize,
// } impl Data {
//     pub fn weed_out_too_deep(self, depth: usize) -> Indexer {
//         let max_depth = self.depth;
//         if depth == max_depth {
//             self.data
//         } else {
//             self.data.into_iter().filter(|x| x.1.0 > depth).collect::<Indexer>()
//         }
//     }
// }


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
                std::fs::read("url_map_icepick_json_test.txt") {
            let x: Data = serde_json::from_str(
                inner
                    .into_iter()
                    .map(|x| x as char)
                    .collect::<String>()
                    .as_str(),
            )?;

            if x.meta.max_depth <= depth /*x.depth <= depth*/ {
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
                std::fs::read("test") {
            let x: Data = serde_json::from_str(
                inner
                    .into_iter()
                    .map(|x| x as char)
                    .collect::<String>()
                    .as_str(),
            )?;

            if x.meta.max_depth <= depth /*x.depth <= depth*/ {
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

            if x.meta.max_depth <= depth /*x.depth <= depth*/ {
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
    let pages: Vec<Page> = search_normal_case_sensitive("https://askiiart.net/".to_string(), vec!["askiiart".to_string()], UserParams::default(), vec![], 1).await.unwrap();
    std::fs::write("results_wow", format!("{pages:#?}")).unwrap();
    let urls = pages.iter().map(|x: &Page| x.url.to_string()).collect::<Vec<String>>();
    std::fs::write("results_wow2", format!("{urls:#?}")).unwrap();
}

// ...........