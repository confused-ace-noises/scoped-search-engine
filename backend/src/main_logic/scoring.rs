use std::{backtrace, collections::HashMap, future::Future, sync::Arc};

use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use url::Url;

use crate::{main_logic::tree::PageDescriptor, Error, W};
use rayon::{self, iter::{IntoParallelIterator, ParallelIterator}};
use super::{eval::{Html, ScorePage}, store::StorableData, tree::FlatTree, user_options::{Query, UserOptions, UserParameters}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredFlatTree(pub HashMap<W<Url>, ScorePage>);

impl From<HashMap<W<Url>, ScorePage>> for ScoredFlatTree {
    fn from(value: HashMap<W<Url>, ScorePage>) -> Self {
        ScoredFlatTree(value)
    }
}

impl ScoredFlatTree {
    // pub async fn new_maybe(flat_tree: FlatTree, query: &Query, user_options: &UserOptions) -> Result<ScoredFlatTree, Error> {
    //     let client = Arc::new(Client::new());
    //     let rt = Runtime::new()?;

    //     // let scored_flat_tree: ScoredFlatTree = <HashMap<W<Url>, PageDescriptor> as Clone>::clone(&flat_tree).into_par_iter().map(|element| -> Result<_, Error>{
    //     //     let url = element.0;

    //     //     let html = rt.block_on(Html::new_client(&url, &Arc::clone(&client), element.1.depth))?;

    //     //     Ok((url, rt.block_on(html.evaluate(query, user_options, element.1.frequency))))
    //     // }).filter(|element| element.is_ok()).map(|element| element.unwrap()).filter(|element| element.1.is_ok()).map(|element| (element.0, element.1.unwrap())).collect::<HashMap<_, _>>().into();

    //     let tasks = flat_tree.iter().map(|element| {
    //         (Html::new_client(element.0, &client, element.1.depth), element.1)
    //     });

    //     let unzipped: (Iter<Item = impl Future<Output = Html>>, impl Iterator<Item = &PageDescriptor>) = tasks.unzip();

    //     let partial_tasks = unzipped.0.join_all(tasks).await.into_iter().filter_map(|item| {
    //         if item.0.is_ok() {
    //             Some(async {(item.0.unwrap().evaluate(query, user_options, item.1.frequency).await, item.0.unwrap().url.clone())})
    //         } else {
    //             None
    //         }
    //     });

    //     let scored_flat_tree = ScoredFlatTree(join_all(partial_tasks).await.into_iter().filter_map(|item| {
    //         if item.0.is_ok() {
    //             Some((item.1, item.0.unwrap()))
    //         } else {
    //             None
    //         }
    //     }).collect());

    //     Ok(scored_flat_tree)
    // }

    pub async fn /*better*/new(
        flat_tree: FlatTree,
        client: &Client,
        query: &Query,
        user_options: &UserOptions,
    ) -> Result<ScoredFlatTree, Error> {
        // Step 1: Map flat_tree into async tasks and their descriptors
        let tasks: Vec<_> = flat_tree
            .iter()
            .map(|(url, descriptor)| {
                let client = client.clone();
                let depth = descriptor.depth;
                async move {
                    let html_result = Html::new_client(url, &client, depth).await;
                    (html_result, descriptor)
                }
            })
            .collect();
    
        // Step 2: Run all tasks concurrently
        let results = join_all(tasks).await;
    
        // Step 3: Filter out errors and create evaluation tasks
        let evaluation_tasks: Vec<_> = results
            .into_iter()
            .filter_map(|(html_result, descriptor)| {
                if let Ok(html) = html_result {
                    let url = html.url.clone();
                    let frequency = descriptor.frequency;
                    Some(async move {
                        let evaluation_result = html.evaluate(query, user_options, frequency).await;
                        (evaluation_result, url)
                    })
                } else {
                    None
                }
            })
            .collect();
    
        // Step 4: Run evaluation tasks concurrently and collect results
        let scored_flat_tree = ScoredFlatTree(
            join_all(evaluation_tasks)
                .await
                .into_iter()
                .filter_map(|(evaluation_result, url)| {
                    if let Ok(score) = evaluation_result {
                        Some((url, score))
                    } else {
                        None
                    }
                })
                .collect(),
        );
    
        Ok(scored_flat_tree)
    }

    pub fn worse_new(flat_tree: FlatTree, query: &Query, user_options: &UserOptions) -> Result<ScoredFlatTree, Error> {
        let client = Arc::new(Client::new());
        let rt = Runtime::new()?; // Create a new Tokio runtime
    
        // Use block_on to perform async operations within the runtime
        let scored_flat_tree = rt.block_on(async {
            // Collect async tasks into a Vec
            let mut tasks = Vec::new();
            for element in <HashMap<W<Url>, PageDescriptor> as Clone>::clone(&flat_tree).into_iter() {
                let client = Arc::clone(&client);
                let query = query.clone();
                let user_options = user_options.clone();
    
                tasks.push(tokio::spawn(async move {
                    let url = element.0;
                    let html = Html::new_client(&url, &client, element.1.depth).await?;
                    let evaluated = html.evaluate(&query, &user_options, element.1.frequency).await?;
                    Ok::<_, Error>((url, evaluated))
                }));
            }
    
            // Await all tasks and collect results
            let mut results = HashMap::new();
            for task in tasks {
                match task.await {
                    Ok(Ok((url, evaluated))) => {
                        results.insert(url, evaluated);
                    }
                    _ => {
                        // Handle individual task failure here if needed
                    }
                }
            }
            Result::<_, Error>::Ok(results)
        })?;
    
        Ok(scored_flat_tree.into())
    }

    pub async fn new_x(
        flat_tree: FlatTree,
        query: &Query,
        user_options: &UserOptions,
    ) -> Result<ScoredFlatTree, Error> {
        let client = Arc::new(Client::new());
    
        // Collect async tasks into a Vec
        let mut tasks = Vec::new();
        for element in flat_tree.0 {
            let client = Arc::clone(&client);
            let query = query.clone();
            let user_options = user_options.clone();
    
            tasks.push(tokio::spawn(async move {
                let url = element.0;
                let html = Html::new_client(&url, &client, element.1.depth).await?;
                let evaluated = html.evaluate(&query, &user_options, element.1.frequency).await?;
                Ok::<_, Error>((url, evaluated))
            }));
        }
    
        // Await all tasks and collect results
        let mut results = HashMap::new();
        for task in tasks {
            match task.await {
                Ok(Ok((url, evaluated))) => {
                    results.insert(url, evaluated);
                }
                Ok(Err(e)) => {
                    eprintln!("Task error: {:?}", e); // Log inner errors
                }
                Err(e) => {
                    eprintln!("Join error: {:?}", e); // Log Tokio task join errors
                }
            }
        }
    
        Ok(results.into())
    }

    pub fn sort(self) -> ScoredFlatTree {
        let mut vec = self.0.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| (a.1.score.round_ties_even() as usize).cmp(&(b.1.score.round_ties_even() as usize)));
        ScoredFlatTree(vec.into_iter().rev().collect::<HashMap<_, _>>())
    }
}

#[tokio::test]
async fn test() {
    let data = StorableData::get(W(Url::parse("https://askiiart.net/").unwrap()), 2, false).await.unwrap();
    let flat_tree = data.flat_tree;
    let scored_flat_tree = ScoredFlatTree::new_x(flat_tree, &Query::SensitiveString("askiiart".to_string()), &UserOptions {
        parameters: UserParameters::new(-0.7, 1.7, 2.5),

        modifiers: vec![]
    }).await.unwrap().sort();
    // starting_url: String,
    std::fs::write("hewwwooooo", serde_json::to_string_pretty(&scored_flat_tree).unwrap()).unwrap();

}