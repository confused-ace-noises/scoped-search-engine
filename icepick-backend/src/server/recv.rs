use reqwest::Client;
use serde::Deserialize;
use url::Url;
use crate::{main_logic::{scoring::ScoredFlatTree, store::StorableData, user_options::{Query, UserOptions}}, Error, W};
use super::reply::ReplyInfo;

#[derive(Debug, Clone, Deserialize)]
pub struct RecvInfo {
    query_data: Query,
    starting_url: String,
    depth_to_reach: usize,
    user_options: UserOptions,
    force_refresh: bool,
} impl RecvInfo {
    pub async fn to_reply_info(self) -> Result<ReplyInfo, Error> {
    
        let client = Client::new();
        let data = StorableData::get(W(Url::parse(&self.starting_url)?), self.depth_to_reach, self.force_refresh)
        .await?;

        let flat_tree = data.flat_tree;
    
        let scored_flat_tree = ScoredFlatTree::new(
            flat_tree,
            &client,
            &self.query_data,
            &self.user_options,
        )
            .await?
            .sort();

        Ok(ReplyInfo::from_flat_tree(scored_flat_tree))
    }
}

#[test]
fn test() {
    let json = r#"{
        "query_data": {
            "query_type": 0,
            "query": "queryhere"
        },
        "starting_url": "https://hewwo.com",
        "depth_to_reach": 2,
        "user_options": {
            "parameters": {
                "depth_coefficient": -0.7,
                "mention_frequency_coefficient": 1.7,
                "n_matches_coefficient": 2.5
            }, 
            "modifiers": [
                {
                    "mod_type": 0,
                    "amount": 7.0,
                    "search": "not_hewwo",
                    "search_type": 0
                },
                {
                    "mod_type": 1,
                    "amount": 4.0,
                    "search": "maybe_hewwo",
                    "search_type": 1
                }
            ]
        },
        "force_refresh": false
    }
    "#;

    let data: RecvInfo = serde_json::from_str(&json).unwrap();

    println!("{:#?}", data);
}