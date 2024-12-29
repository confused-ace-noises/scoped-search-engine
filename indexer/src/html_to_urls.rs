use reqwest::Client;
use select::{document::Document, predicate::Name};
use serde::{self, ser::SerializeStruct, Serialize};
use serde_json;
use std::{error::Error, sync::Arc, time::Duration, vec};
use url::{self, Url};

use crate::{indexer_maker::Indexer, SplitToString};

type GenError = Box<dyn Error>; // generic error
#[allow(unused_variables)]
#[derive(Debug, Clone)]
pub struct Tree {
    url: Url,
    html: String,
    father: Option<(Url, usize)>, // url, depth
    subtree: Option<Vec<Tree>>,
    depth: usize,
}
impl Tree {
    pub async fn new(url: Url, depth_to_reach: usize) -> Result<Tree, GenError> {
        Tree::new_recursive(url, None, depth_to_reach, &Arc::new(None)).await
    }

    pub async fn new_recursive(
        url: Url,
        father: Option<(Arc<Url>, usize)>,
        depth_to_reach: usize,
        client: &Arc<Option<&Client>>,
    ) -> Result<Tree, GenError> {
        match father {
            None => {
                let client = Client::builder()
                    .pool_idle_timeout(Duration::from_secs(90)) // Keep connections alive
                    .pool_max_idle_per_host(10) // Max idle connections per host
                    .build()?;
                let mut vec = Vec::new();
                let list_links = ListLinks::new(&url, &client).await?;
                let value = Arc::new(url.clone());
                for x in list_links.0 {
                    vec.push(
                        Box::pin(Tree::new_recursive(
                            x,
                            Some((Arc::clone(&value), 0)),
                            depth_to_reach,
                            &Arc::new(Some(&client)),
                        ))
                        .await?,
                    );
                }

                Ok(Tree {
                    url: Arc::clone(&value).as_ref().clone(),
                    html: list_links.1,
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
                    let html = Html::new(&url, client.unwrap()).await.unwrap_or(Html { url: url.clone(), html: "".to_string() }).html;
                    return Ok(Tree {
                        url: url.clone(),
                        html,
                        father: Some((borrowed_url, father.1)),
                        subtree: None,
                        depth: father.1 + 1,
                    });
                } else {
                    let borrowed_url = Arc::clone(&father.0).as_ref().clone(); // black magic to get the Url out of the Arc
                                                                               // other case
                    let mut vec = Vec::new();
                    let list_links = match ListLinks::new(&url, client.unwrap()).await {
                        Ok(inner) => inner,
                        Err(_) => {
                            return Ok(Tree {
                                url,
                                html: "".to_string(),
                                father: Some((borrowed_url, father.1)),
                                subtree: None,
                                depth: depth_to_reach,
                            });
                        }
                    };
                    let value = Arc::new(url.clone());
                    for x in list_links.0 {
                        std::fs::write("uhh", format!("{}", x)).unwrap();
                        vec.push(
                            Box::pin(Tree::new_recursive(
                                x,
                                Some((Arc::clone(&value), father.1 + 1)),
                                depth_to_reach,
                                client,
                            ))
                            .await?,
                        );
                    }

                    Ok(Tree {
                        url: url,
                        html: list_links.1,
                        father: Some((borrowed_url, father.1)),
                        subtree: Some(vec),
                        depth: father.1 + 1,
                    })
                }
            }
        }
    }

    pub fn flatten<'a>(&'a self, mut nodes: impl AsMut<Vec<&'a Tree>>) {
        let nodes = nodes.as_mut();

        nodes.push(self);

        if let Some(ref children) = self.subtree {
            for child in children {
                Tree::flatten(child, &mut *nodes);
            }
        }
    }
}

impl Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Tree", 3)?;
        state.serialize_field("url", &self.url.as_str()).unwrap();
        state.serialize_field("subtree", &self.subtree).unwrap();
        state.serialize_field("depth", &self.depth).unwrap();
        state.serialize_field("html", &self.html).unwrap();

        state.end()
    }
}

#[tokio::test]
async fn test() {
    let tree = Tree::new(Url::parse("https://askiiart.net/").unwrap(), 1)
        .await
        .unwrap();

    std::fs::write("uhhhhhhhhh", format!("{:#?}", tree)).unwrap();


    let json = serde_json::to_string_pretty(&tree).unwrap();

    println!("{}", json);
    std::fs::write("OMGF2", format!("{}", json)).unwrap();

    let mut vec: Vec<&Tree> = Vec::new();
    tree.flatten(&mut vec);
    std::fs::write("vec", format!("{:#?}", vec)).unwrap();
    let vec = Indexer::new(vec).await.unwrap();

    // let flat = serde_json::to_string(&vec).unwrap();
    // println!("{}", flat);
    std::fs::write(
        "OMGF1",
        format!("{}", serde_json::to_string_pretty(&vec).unwrap()),
    )
    .unwrap();
}

pub struct Html {
    url: Url,
    html: String,
}
impl Html {
    pub async fn new(url: &Url, clint: &Client) -> Result<Html, GenError> {
        let req: reqwest::Request = clint.get(url.as_str()).build()?;
        let resp = clint.execute(req).await?;
        Ok(Html {
            url: url.clone(),
            html: resp.text().await?,
        })
    }
}

#[tokio::test]
async fn test2() {
    let binding = Url::parse("https://askiiart.net/").unwrap();
    let binding2 = Client::new();
    let x = ListLinks::new(&binding, &binding2).await.unwrap();

    println!("{:#?}", x)
}

#[derive(Debug)]
pub struct ListLinks(pub Vec<Url>, pub String);
impl ListLinks {
    pub async fn new(url: &Url, client: &Client) -> Result<Self, GenError> {
        if let Ok(html) = Html::new(url, client).await {
            let mut res = Vec::new();
            let document = Document::from(html.html.as_str());
            for node in document.find(Name("a")) {
                if let Some(href) = node.attr("href") {
                    // Resolve the relative URL against the base URL
                    if let Ok(resolved_url) = url.join(href) {
                        let fixed_url = Url::parse(
                            &resolved_url
                                .as_str()
                                .to_string()
                                .split_to_string_at_occurrence_tuple("#", crate::Occurrence::Last)
                                .0,
                        )?;
                        if !res.contains(&fixed_url) {
                            res.push(fixed_url);
                        }
                    }
                }
            }

            Ok(ListLinks(res, html.html))
        } else {
            Ok(ListLinks(vec![], "".to_string()))
        }
    }
}
