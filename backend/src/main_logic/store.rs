use std::{io::Error, path::Path};

use crate::{
    main_logic::tree::{FlatTree, Tree},
    W,
    utils::misc::{Occurrence, SplitToString}
};
use serde::{Deserialize, Serialize};
use url::Url;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    max_depth: usize,
    starting_url: W<Url>,
    storage_hash: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorableData {
    pub tree: Tree,
    pub flat_tree: FlatTree,
    pub metadata: MetaData,
}

// TODO: add returning less deep StorableDatas from deeper ones, like the other backend.
impl StorableData {
    pub fn new(tree: Tree) -> Result<Self, crate::Error> {
        let folder_name = Path::new("icepick_crawl_data");
        if !std::fs::exists(folder_name)? {
            std::fs::create_dir(folder_name)?;
        }

        let starting_url = tree.url.clone();
        let hash = xxh3_64(starting_url.as_str().as_bytes());
        let str_file = format!("icepick_crawl_data#{}.json", hash);
        let end_filename = Path::new(&str_file);
        let filename = folder_name.join(end_filename);

        if std::fs::exists(&filename)? {
            let read_json = std::fs::read_to_string(&filename)?;
            let from_json: StorableData = serde_json::from_str(&read_json)?;
            return Ok(from_json);
        } else {
            let flat_tree = FlatTree::new(&tree);

            let mut max_depth = 0;
            for (_, page_descriptor) in flat_tree.iter() {
                let depth = page_descriptor.depth;
                if max_depth < depth {
                    max_depth = depth
                }
            }

            let storable_data = StorableData {
                tree,
                flat_tree,
                metadata: MetaData {
                    max_depth,
                    starting_url,
                    storage_hash: hash,
                },
            };

            let json = serde_json::to_string_pretty(&storable_data)?;
            std::fs::write(filename, json)?;

            return Ok(storable_data);
        }
    }

    pub async fn get(url: W<Url>, depth_to_reach: usize) -> Result<StorableData, crate::Error> {
        let folder_path = Path::new(&"icepick_crawl_data");
        let hash = xxh3_64(url.as_str().as_bytes());
        if std::fs::exists(folder_path)? {
            let mut file_found = false;
            for file in std::fs::read_dir(folder_path)? {
                if let Ok(file) = file {
                    let hash_file = file.file_name().to_string_lossy().to_string().split_to_string_at_occurrence_tuple("#", Occurrence::First).1.split_to_string_at_occurrence_tuple(".", Occurrence::First).0.parse::<u64>().map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
                    if hash == hash_file {
                        file_found = true;
                    }
                }
            }

            if file_found {
                let file_string = format!("icepick_crawl_data#{}.json", hash);
                let end_file_path = Path::new(&file_string);

                let full_path = folder_path.join(end_file_path);
                let data = std::fs::read_to_string(full_path)?;
                let storable_data: StorableData = serde_json::from_str(&data)?;
                return Ok(storable_data);
            } else {
                let tree = Tree::new(url.to_string(), depth_to_reach).await?;
                return Self::new(tree)
            }
        } else {
            std::fs::create_dir(folder_path)?;
            todo!()
            // TODO add new 
        }
    }
}

#[tokio::test]
async fn idkkkkk() {
    let data = StorableData::get(W(Url::parse("https://askiiart.net/").unwrap()), 1).await.unwrap();

    println!("{:#?}", data);
}