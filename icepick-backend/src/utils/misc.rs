use std::ops::{Deref, DerefMut};

/// wrapper type
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct W<T>(pub T);

impl<T> Deref for W<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for W<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// ------------ W(Url) ------------
use url::Url;

impl AsRef<str> for W<Url> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}


// ------------ W(Url) ------------


//---------------------------------

pub trait SplitToString{
    fn split_to_string(&self, p: impl Into<String>) -> Vec<String>;
    fn split_to_string_at_occurrence_tuple<P: Clone + Into<String>>(&self, p: P, occurrence: Occurrence) -> (String, String);
    fn split_to_string_at_occurrence<P: Clone + Into<String>>(&self, p: P, occurrence: Occurrence) -> Vec<String>;
}

impl SplitToString for String {
    fn split_to_string(&self, p: impl Into<String>) -> Vec<String> {
        self.split(Into::<String>::into(p).as_str()).map(|s| s.to_string()).collect()
    }

    fn split_to_string_at_occurrence_tuple<P: Clone + Into<String>>(&self, p: P, occurrence: Occurrence) -> (String, String) {
        match occurrence {
            Occurrence::First => {
                let x = self.split_once(&Into::<String>::into(p)).unwrap_or((self.as_str(), ""));
                return (x.0.to_string(), x.1.to_string())
            },
            Occurrence::Last => {
                let x = self.chars().rev().collect::<String>();
                let x = x.split_once(&Into::<String>::into(p)).unwrap_or(("", x.as_str()));
                return (x.1.to_string().chars().rev().collect::<String>(), x.0.to_string().chars().rev().collect::<String>()) // has to be reversed
            },
            Occurrence::Nth(n) => {
                let split = self.split(&Into::<String>::into(p.clone())).collect::<Vec<&str>>();

                let len = split.len();

                if n >= len {
                    return (split.join(&Into::<String>::into(p)).to_string(), "".to_string())
                } else {
                    let first = split[..n].join(&Into::<String>::into(p.clone()));
                    let second = split[n..].join(&Into::<String>::into(p)); 

                    (first, second)
                }
            },
        }
    }

    fn split_to_string_at_occurrence<P: Clone + Into<String>>(&self, p: P, occurrence: Occurrence) -> Vec<String> {
        let x = self.split_to_string_at_occurrence_tuple(p, occurrence);

        vec![x.0, x.1]
    }
}

pub enum Occurrence {
    First, 
    Last,
    Nth(usize),    
}