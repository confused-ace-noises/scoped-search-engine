use crate::{
    main_logic::eval::Html,
    Error, W,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map, HashMap}, ops::{Deref, DerefMut}, sync::Arc, time::Duration
};
use url::Url;
use tokio;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// ## WARNING
/// doesn't fully calculate score of pages. (everything but frequency of url appearance)
pub struct Tree {
    pub url: W<Url>,
    pub title: String,
    pub subtree: Option<Vec<Tree>>,
    pub depth: usize,
}

impl Tree {
    pub async fn new(starting_url: String, depth_to_reach: usize) -> Result<Tree, Error> {
        let client = Client::builder()
                    .pool_idle_timeout(Duration::from_secs(90)) // Keep connections alive
                    .pool_max_idle_per_host(10) // Max idle connections per host
                    .build()?;

        Self::new_recursive(Arc::new(client), W(Url::parse(&starting_url)?), 0, depth_to_reach, true).await
    }

    pub async fn new_recursive(
        client: Arc<Client>,
        url: W<Url>,
        depth: usize,
        depth_to_reach: usize,
        first_recursion: bool
    ) -> Result<Tree, Error> {
        std::fs::write("testing", url.to_string()).unwrap();
        match first_recursion {
            true => {
                let depth = 0; // because it's the starting point
                let html = Html::new_client(&url, &client, depth).await?;
                let mut vec_result = Vec::new();
                let links = html.get_links()?;
                let title = html.title();
                drop(html);

                // let to_be_father_url = Arc::new(url);

                for link in links {
                    vec_result.push(
                        Box::pin(Self::new_recursive(
                            client.clone(),
                            link,
                            depth+1,
                            depth_to_reach,
                            false
                        ))
                        .await?,
                    )
                }

                Ok(Tree {
                    url,
                    title,
                    depth,
                    subtree: Some(vec_result),
                })
            }
            false => {
                // let depth = father_depth + 1;

                if depth >= depth_to_reach {
                    //base case
                    let html = Html::new_client(&url, client.as_ref(), depth).await.unwrap_or(
                    return Ok(Self::dead_tree(url, depth))
                    );

                    let title = html.title();
                    drop(html);

                    return Ok(Tree {
                        title,
                        url,
                        depth,
                        subtree: None,
                    });
                } else {
                    // middle case
                    let html = Html::new_client(&url, client.as_ref(), depth).await;
                    if let Ok(inner) = html {
                        let mut tree_vec = Vec::new();
                        let links = inner.get_links()?;
                        let title = inner.title();
                        drop(inner);
                        for link in links {
                            tree_vec.push(Box::pin(
                                Self::new_recursive(
                                    Arc::clone(&client),
                                    link,
                                    depth+1,
                                    depth_to_reach,
                                    false,
                                )).await?,
                            );
                        }


                        return Ok(Tree {
                            title,
                            url,
                            depth,
                            subtree: Some(tree_vec),
                        });
                    } else {
                        // dead page
                        return Ok(Self::dead_tree(url, depth));
                    }
                }
            }
        }
    }

    fn dead_tree(url: W<Url>, depth: usize) -> Tree {
        Tree {
            title: url.to_string(),
            url,
            depth,
            subtree: None,
        }
    }
}

// pub struct Tree {
//     pub page: Page,
//     pub depth: usize,
//     pub father: Option<(Arc<W<Url>>, usize)>, // url, depth
//     // pub score: f64,
//     pub subtree: Option<Vec<Tree>>,
// }

// impl Serialize for Tree {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer 
//     {
//         let mut state = serializer.serialize_map(Some(4))?;

//         state.serialize_entry("url", &self.url)?;
//         state.serialize_entry("title", &self.title)?;
//         state.serialize_entry("subtree", &self.subtree)?;
//         state.serialize_entry("depth", &self.depth)?;

//         state.end()
//     }
// }

// impl<'de> Deserialize<'de> for Tree {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> 
//     {
//         pub struct TreeVisitor;

//         impl<'de> Visitor<'de> for TreeVisitor {
//             type Value = Tree;
        
//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("expected a nested tree of urls, depths and titles")
//             }

//             fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::MapAccess<'de>, 
//             {
//                 while let Some(inner) = map.next_key()? {
                    
//                 }

//                 todo!()
//             }
//         }

//         todo!()
//     }
// }

#[tokio::test]
async fn xxx() {
    let tree = Tree::new("https://askiiart.net/".to_string(), 2).await.unwrap();

    let json = serde_json::to_string_pretty(&tree).unwrap();

    std::fs::write("test1", json).unwrap();

    let json_read: Tree = serde_json::from_str(&std::fs::read_to_string("test1").unwrap()).unwrap();
    std::fs::write("test2", format!("{:#?}", json_read)).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDescriptor {
    pub depth: usize,
    pub frequency: usize,
}

#[tokio::test]
async fn xx() {
    // let binding = Query::String("rust".to_string());
    // let binding1 = UserOptions::new(UserParameters::new(-0.7, 1.7, 2.5), vec![UserModifiers { modification_type: crate::user_options::UserModType::Penalize(-25.0), searcher: crate::utils::searcher::Searcher::from_string("codeberg") }]);
    let tree = Tree::new("https://doc.rust-lang.org/book/".to_string(), 2).await.unwrap();

    std::fs::write("idk3", serde_json::to_string_pretty(&tree).unwrap()).unwrap();

    let flat_tree = FlatTree::new(&tree);

    std::fs::write("idk", format!("{flat_tree:#?}")).unwrap();

    let json = serde_json::to_string_pretty(&flat_tree).unwrap();
    std::fs::write("idk2", json).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatTree(pub HashMap<W<Url>, PageDescriptor>);
impl FlatTree {
    pub fn new(tree: &Tree) -> Self {
        let mut hashmap: HashMap<W<Url>, PageDescriptor> = HashMap::new();

        let vec = Self::recursive_eval(tree);
        for element in vec {
            if hashmap.contains_key(&element.0) {
                hashmap.get_mut(&element.0).unwrap().frequency += 1;
            } else {
                hashmap.insert(element.0, PageDescriptor { depth: element.1, frequency: 1 });
            }
        }

        FlatTree(hashmap)
    }

    fn recursive_eval(tree: &Tree) -> Vec<(W<Url>, usize)> {
        match tree.subtree {
            Some(ref subtree) => {
                let mut vec = Vec::new();
                for tree in subtree {
                    vec.push(Self::recursive_eval(&tree))
                }
                vec.into_iter().flatten().collect()
            },
            None => {
                let url = tree.url.clone();
                let mut res = Vec::new();
                res.push((url, tree.depth));
                res
            },
        }
    }

    pub fn iter(&self) -> hash_map::Iter<'_, W<Url>, PageDescriptor> {
        self.0.iter()
    }
}

// impl IntoIterator for FlatTree {
//     type Item = (W<Url>, PageDescriptor);

//     type IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

impl Deref for FlatTree {
    type Target = HashMap<W<Url>, PageDescriptor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlatTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// TODO: Add a flattened tree struct that also calculates the n_mentions: serde it all

#[tokio::test]
async fn test_tree() {
    // let binding = Query::String("askiiart".to_string());
    // let binding1 = UserOptions::new(UserParameters::new(-0.7, 1.7, 2.5), vec![UserModifiers { modification_type: crate::user_options::UserModType::Penalize(-25.0), searcher: crate::utils::searcher::Searcher::from_string("codeberg") }]);
    let tree = Tree::new("https://askiiart.net/".to_string(), 1).await.unwrap();

    let string = format!("{tree:#?}");
    std::fs::write("output", string).unwrap();
}