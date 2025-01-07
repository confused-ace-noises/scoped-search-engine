use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod cli;

// {
//     "query_data": {
//         "query_type": 0,
//         "query": "queryhere"
//     },
//     "starting_url": "https://hewwo.com",
//     "depth_to_reach": 2,
//     "user_options": {
//         "parameters": {
//             "depth_coefficient": -0.7,
//             "mention_frequency_coefficient": 1.7,
//             "n_matches_coefficient": 2.5
//         }, 
//         "modifiers": [
//             {
//                 "mod_type": 0,
//                 "amount": 7.0,
//                 "search": "not_hewwo",
//                 "search_type": 0
//             },
//             {
//                 "mod_type": 1,
//                 "amount": 4.0,
//                 "search": "maybe_hewwo",
//                 "search_type": 1
//             }
//         ]
//     },
//     "force_refresh": false
// }

#[derive(Serialize, Debug, Clone)]
pub struct SendInfo {
    pub query_data: Query,
    pub starting_url: String,
    pub depth_to_reach: usize,
    pub user_options: UserOptions,
    pub force_refresh: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserOptions {
    pub parameters: Params,
    pub modifiers: Vec<String>
}

#[derive(Serialize, Debug, Clone)]
pub struct Params {
    pub depth_coefficient: f64,
    pub mention_frequency_coefficient: f64,
    pub n_matches_coefficient: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct Query {
    pub query_type: usize,
    pub query: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReplyPage {
    pub url: String,
    pub title: String,
    pub score: f64,
}

impl Display for ReplyPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:  {};    {}", self.title, self.url, self.score)
    }
}