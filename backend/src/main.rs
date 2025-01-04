use backend::{main_logic::{scoring::ScoredFlatTree, store::StorableData, user_options::{Query, UserOptions, UserParameters}}, utils::misc::W};
use reqwest::Client;
use url::Url;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let data = StorableData::get(W(Url::parse("https://askiiart.net/").unwrap()), 2, true).await.unwrap();
    let flat_tree = data.flat_tree;
    let scored_flat_tree = ScoredFlatTree::new(flat_tree, &client, &Query::SensitiveString("askiiart".to_string()), &UserOptions {
        parameters: UserParameters::new(-0.7, 1.7, 2.5),

        modifiers: vec![]
    }).await.unwrap().sort();
    // starting_url: String,
    std::fs::write("hewwwooooo", serde_json::to_string_pretty(&scored_flat_tree).unwrap()).unwrap();
}
