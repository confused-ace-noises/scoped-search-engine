pub mod matches;

pub use indexer::indexer_maker::Indexer;
use regex::Regex;
use url::Url;

pub enum Patt {
    String(String),
    Regex(Regex)
}

pub enum UserMod {
    Boost(f64),
    Penal(f64),
    Ban 
}

pub struct UserModifier {
    value_modifier: UserMod,
    pattern: Patt
}

pub struct UserParams {
    depth_w: f64,
    frequency_w: f64,
    n_matches_w: f64,
}

impl Default for UserParams {
    fn default() -> Self {
        Self { depth_w: -0.7, frequency_w: 1.7, n_matches_w: 2.5 }
    }
}