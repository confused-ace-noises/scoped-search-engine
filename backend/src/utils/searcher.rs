use crate::Error;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum Searcher {
    String(String),
    Regex(Regex),
}
impl Searcher {
    pub fn from_string(string: impl Into<String>) -> Searcher {
        Searcher::String(Into::<String>::into(string))
    }

    pub fn from_regex(regex: impl Into<String>) -> Result<Searcher, Error> {
        Ok(Searcher::Regex(Regex::new(&Into::<String>::into(regex))?))
    }

    pub fn search<S: Into<String>>(&self, haystack: S) -> usize {
        let haystack = Into::<String>::into(haystack);

        match self {
            Searcher::String(string) => haystack.matches(string).count(),
            Searcher::Regex(regex) => regex.captures_iter(&haystack).count(),
        }
    }
}

pub trait Searchable {
    fn search(&self, searcher: Searcher) -> usize;
}

impl<S: Into<String> + Clone> Searchable for S {
    fn search(&self, searcher: Searcher) -> usize {
        let haystack = Into::<String>::into(self.clone());

        let n_matches = match searcher {
            Searcher::String(string) => haystack.matches(&string).count(),
            Searcher::Regex(regex) => regex.captures_iter(&haystack).count(),
        };

        n_matches
    }
}
