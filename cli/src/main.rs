use std::error::Error;

use clap::Parser;
use cli::{cli::Cli, Params, Query, ReplyPage, SendInfo, UserOptions};

fn main() -> Result<(), Box<dyn Error>>{
    let cli = Cli::parse();

    let send_info = SendInfo {
        query_data: Query {
            query_type: cli.query_type as usize,
            query: cli.query,
        },
        starting_url: cli.starting_url,
        depth_to_reach: cli.depth as usize,
        user_options: UserOptions {
            parameters: Params {
                depth_coefficient: cli.params_depth_coefficient,
                mention_frequency_coefficient: cli.params_frequency_coefficient,
                n_matches_coefficient: cli.params_n_matches_coefficient,
            },
            modifiers: Vec::new(),
        },
        force_refresh: cli.force_refresh
    }; 

    let json = serde_json::to_string(&send_info)?;
    
    println!("{}", json);

    let client = reqwest::blocking::Client::builder().timeout(None).build()?;

    let resp = client.post("http://localhost:6728/api/search").header("content-type", "application/json").body(json).send().map_err(|_| "the icepick server isn't running.")?;

    if resp.status().is_success() {
        let text = resp.text()?;
        let deserialized: Vec<ReplyPage> = serde_json::from_str(&text).unwrap();

        deserialized.iter().for_each(|reply_page| println!("{}", reply_page));

        Ok(())
    } else {
        return Err(format!("something went wrong. check your arguments. response code: {}", resp.status()).into());
    }
}
