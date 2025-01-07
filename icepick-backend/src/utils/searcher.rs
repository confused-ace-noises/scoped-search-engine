use crate::Error;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum Searcher {
    InsensitiveString(String),
    SensitiveString(String),
    Regex(Regex),
}
impl Searcher {
    pub fn from_string(string: impl Into<String>, case_sensitive: bool) -> Searcher {
        if case_sensitive {
            Searcher::SensitiveString(Into::<String>::into(string))
        } else {
            Searcher::InsensitiveString(Into::<String>::into(string).to_lowercase())
        }
    }

    pub fn from_regex(regex: impl Into<String>) -> Result<Searcher, Error> {
        Ok(Searcher::Regex(Regex::new(&Into::<String>::into(regex))?))
    }

    pub fn search<S: Into<String>>(&self, haystack: S) -> usize {
        let haystack = Into::<String>::into(haystack);

        match self {
            Searcher::InsensitiveString(string) => haystack.to_lowercase().matches(string).count(),
            Searcher::SensitiveString(string) => haystack.matches(string).count(),
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
            Searcher::InsensitiveString(string) => haystack.to_lowercase().matches(&string).count(),
            Searcher::SensitiveString(string) => haystack.matches(&string).count(),
            Searcher::Regex(regex) => regex.captures_iter(&haystack).count(),
        };

        n_matches
    }
}
