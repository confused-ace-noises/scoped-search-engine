use serde::{ser::SerializeSeq, Serialize};

use crate::main_logic::scoring::ScoredFlatTree;

#[derive(Debug, Clone)]
pub struct ReplyInfo (
    pub Vec<ReplyPage>
); impl ReplyInfo {
    pub fn from_flat_tree(score_flat_tree: ScoredFlatTree) -> ReplyInfo {
        let vec = score_flat_tree.0.into_iter().map(|element| element.1.to_reply_page()).collect::<Vec<_>>();
        ReplyInfo (vec)
    }
}
impl Serialize for ReplyInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_seq(Some(self.0.len()))?;
        for rep_page in self.0.iter() {
            state.serialize_element(rep_page)?;
        }

        state.end()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ReplyPage {
    pub url: String,
    pub title: String,
    pub score: f64,
}

#[test]
fn test() {
    let reply_page = ReplyPage {
        url: "adfaEF".to_string(),
        title: "hewwo".to_string(),
        score: 45.0
    };

    let reply_page1 = ReplyPage {
        url: "zdliyfgyadg".to_string(),
        title: "hewwooooo".to_string(),
        score: 15.0
    };

    let reply_info = ReplyInfo (vec![reply_page, reply_page1]);

    let str_json = serde_json::to_string_pretty(&reply_info).unwrap();

    println!("{}", str_json)
}