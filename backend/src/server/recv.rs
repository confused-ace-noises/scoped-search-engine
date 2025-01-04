use serde::Deserialize;

use crate::main_logic::user_options::{Query, UserOptions};

use super::reply::ReplyInfo;

// pub struct UserParameters {
//     depth_coefficient: f64,
//     mention_frequency_coefficient: f64,
//     n_matches_coefficient: f64,
// }

#[derive(Debug, Clone, Deserialize)]
pub struct RecvInfo {
    query_data: Query,
    starting_url: String,
    user_options: UserOptions,
    force_refresh: bool, // TODO: implement this
} impl RecvInfo {
    pub fn to_reply_info() -> ReplyInfo {
        todo!()
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