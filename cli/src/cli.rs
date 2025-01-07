use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    /// the starting url for the crawler.
    #[arg(long, short)]
    pub starting_url: String,

    /// the depth for the crawler to crawl to.
    #[arg(long, short)]
    pub depth: u32,
    
    /// the query.
    #[arg(long, short)]
    pub query: String,

    #[arg(short = 'r', long, default_value_t = false)]
    pub force_refresh: bool,
    
    /// the type of the query. 0 = case sensitive string, 1 = case insensitive string, 2 = rust regex. 
    #[arg(long, default_value_t=0)]
    pub query_type: u8,
    
    /// coefficient of depth for score calculation.
    #[arg(long, default_value_t = -0.7)]
    pub params_depth_coefficient: f64,

    /// coefficient of frequency of url for score calculation.
    #[arg(long, default_value_t = 1.7)]
    pub params_frequency_coefficient: f64,

    /// coefficient of number of matches of the query for the score calculation.
    #[arg(long, default_value_t = 2.5)]
    pub params_n_matches_coefficient: f64
}