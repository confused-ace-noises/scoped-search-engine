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
use futures::future::join_all;

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
        println!("start");
        
        let client = Client::builder()
                    .pool_idle_timeout(Duration::from_secs(90)) // Keep connections alive
                    .pool_max_idle_per_host(10) // Max idle connections per host
                    .user_agent("icepick-crawler")
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
        println!("{}", url.as_str());
        let delay_duration = Duration::from_millis(150);
        std::thread::sleep(delay_duration);
        // tokio::time::sleep(delay_duration);
        // std::fs::write("testing", url.to_string()).unwrap();
        // tokio::fs::write("testing", url.to_string()).await.unwrap();
        match first_recursion {
            true => {
                let depth = 0; // because it's the starting point
                let html = Html::new_client(&url, &client, depth).await?;
                let links = html.get_links()?;
                let title = html.title();
                drop(html);
                
                let tasks = links.into_iter().map(|link| {
                    Self::new_recursive(client.clone(), link, depth+1, depth_to_reach, false)
                });

                let vec_result = join_all(tasks).await.into_iter().filter(|page| page.is_ok()).map(|page| page.unwrap()).collect::<Vec<_>>();

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
                        let links = inner.get_links()?;
                        let title = inner.title();
                        drop(inner);
                        
                        let tasks = links.into_iter().map(|link| {
                            Self::new_recursive(
                                Arc::clone(&client),
                                link,
                                depth+1,
                                depth_to_reach,
                                false,
                            )
                        });

                        let tree_vec = join_all(tasks).await.into_iter().filter_map(|item| {
                            if item.is_ok() {
                                Some(item.unwrap())
                            } else {
                                None
                            }
                        }).collect::<Vec<_>>();
                        
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

    pub fn remove_too_deep(&self, max_depth: usize) -> Option<Tree> {
        if self.depth > max_depth {
            // Node exceeds max_depth, skip it entirely
            return None;
        }

        // Recursively process subtree if it exists
        let shallow_subtree = self.subtree.as_ref().map(|children| {
            children
                .iter()
                .filter_map(|child| child.remove_too_deep(max_depth)) // Filter valid children
                .collect::<Vec<_>>() // Collect into a Vec<Tree>
        });

        // Return a new Tree with the filtered subtree
        Some(Tree {
            url: self.url.clone(),
            title: self.title.clone(),
            subtree: shallow_subtree,
            depth: self.depth,
        })
    }
}

#[test]
fn test2() {
    let tree: Tree = serde_json::from_str(&std::fs::read_to_string("test1").unwrap()).unwrap();

    let new_tree = tree.remove_too_deep(1).unwrap();
    let json = serde_json::to_string_pretty(&new_tree).unwrap();
    std::fs::write("less_depth", json).unwrap();
}

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

        hashmap.insert(tree.url.clone(), PageDescriptor { depth: tree.depth, frequency: 1 });

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

    pub fn remove_too_deep(&self, max_depth: usize) -> FlatTree {
        FlatTree(<HashMap<W<Url>, PageDescriptor> as Clone>::clone(&self).into_iter().filter(|member| member.1.depth <= max_depth).collect::<HashMap<_, _>>())
    }
}

impl FromIterator<(W<Url>, PageDescriptor)> for FlatTree {
    fn from_iter<T: IntoIterator<Item = (W<Url>, PageDescriptor)>>(iter: T) -> Self {
        FlatTree(iter.into_iter().collect())
    }
}

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