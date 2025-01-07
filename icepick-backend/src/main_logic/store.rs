use std::{collections::HashMap, io::Error, path::Path};

use crate::{
    main_logic::tree::{FlatTree, Tree},
    utils::misc::{Occurrence, SplitToString},
    W,
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
    pub fn new(tree: Tree, force_refresh: bool) -> Result<Self, crate::Error> {
        let folder_name = Path::new("icepick_crawl_data");
        if !std::fs::exists(folder_name)? {
            std::fs::create_dir(folder_name)?;
        }

        let starting_url = tree.url.clone();
        let hash = xxh3_64(starting_url.as_str().as_bytes());
        let str_file = format!("icepick_crawl_data#{}.json", hash);
        let end_filename = Path::new(&str_file);
        let filename = folder_name.join(end_filename);

        let flat_tree = FlatTree::new(&tree);
        let mut max_depth = 0;
        for (_, page_descriptor) in flat_tree.iter() {
            let depth = page_descriptor.depth;
            if max_depth < depth {
                max_depth = depth
            }
        }

        if std::fs::exists(&filename)? && !force_refresh {
            let read_json = std::fs::read_to_string(&filename)?;
            let from_json: StorableData = serde_json::from_str(&read_json)?;

            if from_json.metadata.max_depth == max_depth {
                return Ok(from_json);
            } else if from_json.metadata.max_depth > max_depth {
                let tree_part = tree.remove_too_deep(max_depth).unwrap();
                let flat_tree_part = FlatTree(flat_tree.0.into_iter().filter(|x| x.1.depth <= max_depth).collect::<HashMap<_, _>>());
                let metadata = MetaData {
                    max_depth,
                    starting_url,
                    storage_hash: from_json.metadata.storage_hash,
                };

                Ok(StorableData { tree: tree_part, flat_tree: flat_tree_part, metadata })
            } else {
                let metadata = MetaData {
                    max_depth,
                    starting_url,
                    storage_hash: from_json.metadata.storage_hash,
                };

                let storable_data = StorableData {
                    tree,
                    flat_tree,
                    metadata
                };

                std::fs::write(filename, &serde_json::to_string_pretty(&storable_data)?)?;
                println!("wrote");
                Ok(storable_data)
            }
        } else {
           

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

    pub async fn get(url: W<Url>, depth_to_reach: usize, force_refresh: bool) -> Result<StorableData, crate::Error> {
        let folder_path = Path::new(&"icepick_crawl_data");
        let hash = xxh3_64(url.as_str().as_bytes());
        Ok(if std::fs::exists(folder_path)? {
            let mut file_found = false;
            for file in std::fs::read_dir(folder_path)? {
                if let Ok(file) = file {
                    let hash_file = file
                        .file_name()
                        .to_string_lossy()
                        .to_string()
                        .split_to_string_at_occurrence_tuple("#", Occurrence::First)
                        .1
                        .split_to_string_at_occurrence_tuple(".", Occurrence::First)
                        .0
                        .parse::<u64>()
                        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
                    if hash == hash_file {
                        file_found = true;
                    }
                }
            }
            println!("{}, hash: {}", file_found, hash);
            if file_found && !force_refresh {
                let file_string = format!("icepick_crawl_data#{}.json", hash);
                let end_file_path = Path::new(&file_string);

                let full_path = folder_path.join(end_file_path);
                let data = std::fs::read_to_string(full_path)?;
                let storable_data: StorableData = serde_json::from_str(&data)?;
                if storable_data.metadata.max_depth == depth_to_reach {
                    return Ok(storable_data);
                } else if storable_data.metadata.max_depth > depth_to_reach {
                    let part_tree = storable_data.tree;
                    let part_flat_tree: FlatTree = storable_data.flat_tree;
                    let metadata = storable_data.metadata;

                    return Ok(StorableData {
                        tree: part_tree.remove_too_deep(depth_to_reach).unwrap(), // unwrapping here is okay
                        flat_tree: part_flat_tree.remove_too_deep(depth_to_reach),
                        metadata: MetaData { max_depth: depth_to_reach, starting_url: metadata.starting_url, storage_hash: metadata.storage_hash }
                    })
                } else {
                    StorableData::new(Tree::new(url.to_string(), depth_to_reach).await?, force_refresh)?
                }
            } else {
                let tree = Tree::new(url.to_string(), depth_to_reach).await?;
                return Self::new(tree, force_refresh);
            }
        } else {
            std::fs::create_dir(folder_path)?;
            let tree = Tree::new(url.as_str().to_string(), depth_to_reach).await?;
            return Self::new(tree, force_refresh)
        })
    }
}

#[tokio::test]
async fn idkkkkk() {
    let data = StorableData::get(W(Url::parse("https://en.wikipedia.org/wiki/Belphegor%27s_prime").unwrap()), 1, true)
        .await
        .unwrap();

    println!("{:#?}", data);
}
