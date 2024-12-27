use select::{document::Document, predicate::Name};
use std::{error::Error, sync::Arc};
use url::{self, Url};
use serde::{self, ser::SerializeStruct, Serialize};
use serde_json;

type GenError = Box<dyn Error>; // generic error

#[derive(Debug, Clone)]
pub struct Tree {
    url: Url,
    father: Option<(Url, usize)>, // url, depth
    subtree: Option<Vec<Tree>>,
    depth: usize,
}
impl Tree {
    pub async fn new(url: Url, depth_to_reach: usize) -> Result<Tree, GenError> {
        Tree::new_recursive(url, None, depth_to_reach).await
    }

    pub async fn new_recursive(
        url: Url,
        father: Option<(Arc<Url>, usize)>,
        depth_to_reach: usize,
    ) -> Result<Tree, GenError> {
        match father {
            None => {
                let mut vec = Vec::new();
                let list_links = ListLinks::new(url.clone()).await?;
                let value = Arc::new(url.clone());
                for x in list_links.0 {
                    vec.push(
                        Box::pin(Tree::new_recursive(x, Some((Arc::clone(&value), 0)), depth_to_reach))
                            .await?,
                    );
                }

                Ok(Tree {
                    url: Arc::clone(&value).as_ref().clone(),
                    father: None,
                    subtree: Some(vec),
                    depth: 0,
                })
            }
            Some(father) => {
                // base case
                if father.1 + 1 >= depth_to_reach {
                    // depth of the father +1 is equal or higher then the depth to reach :3
                    let borrowed_url = Arc::clone(&father.0).as_ref().clone(); // black magic to get the Url out of the Arc
                    return Ok(Tree {
                        url,
                        father: Some((borrowed_url, father.1)),
                        subtree: None,
                        depth: father.1 + 1,
                    });
                } else {
                    let borrowed_url = Arc::clone(&father.0).as_ref().clone(); // black magic to get the Url out of the Arc
                    println!("at least it's working i guess");
                    // other case
                    let mut vec = Vec::new();
                    let list_links = match ListLinks::new(url.clone()).await {
                        Ok(inner) => inner,
                        Err(_) => return Ok(Tree {
                            url,
                            father: Some((borrowed_url, father.1)),
                            subtree: None,
                            depth: depth_to_reach,
                        }),
                    };
                    let value = Arc::new(url.clone());
                    for x in list_links.0 {
                        vec.push(
                            Box::pin(Tree::new_recursive(x, Some((Arc::clone(&value), father.1+1)), depth_to_reach))
                                .await?,
                        );
                    }

                    Ok(Tree {
                        url: url,
                        father: Some((borrowed_url, father.1)),
                        subtree: Some(vec),
                        depth: father.1+1,
                    })
                }
            }
        }
    }
}

impl Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_struct("Tree", 3)?;
        state.serialize_field("url", &self.url.as_str()).unwrap();
        state.serialize_field("subtree", &self.subtree).unwrap();
        state.serialize_field("depth", &self.depth).unwrap();

        state.end()
    }
}

#[tokio::test]
async fn test() {
    let tree = Tree::new(Url::parse("https://www.fanton.com/").unwrap(), 2).await.unwrap();

    let json = serde_json::to_string_pretty(&tree).unwrap();

    println!("{}", json);
    std::fs::write("OMGF2", format!("{}", json)).unwrap();
}

pub struct Html {
    url: Url,
    html: String,
}
impl Html {
    pub async fn new(url: Url) -> Result<Html, GenError> {
        let resp = reqwest::get(url.clone()).await?;

        Ok(Html {
            url,
            html: resp.text().await?,
        })
    }
}

pub struct ListLinks(pub Vec<Url>);
impl ListLinks {
    pub async fn new(url: Url) -> Result<Self, GenError> {
        let html = Html::new(url.clone()).await?;
        let mut res = Vec::new();
        let document = Document::from(html.html.as_str());
        for node in document.find(Name("a")) {
            if let Some(href) = node.attr("href") {
                // Resolve the relative URL against the base URL
                if let Ok(resolved_url) = url.join(href) {
                    res.push(resolved_url);
                }
            }
        }
    
        Ok(ListLinks(res))
    }    
}
