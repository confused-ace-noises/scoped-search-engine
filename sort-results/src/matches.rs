use indexer::indexer_maker::Indexer;
use regex::{self, Regex};
use serde::{
    de::{self, value, MapAccess, Visitor}, ser::SerializeMap, Deserialize, Deserializer, Serialize
};
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt,
    iter::Sum,
    ops::{Add, Div}, string,
};
use url::{self, Url};

use crate::{UserModifier, UserParams};

#[derive(Debug)]
pub struct Matches {
    url: Url,
    html: String,
    n_matches: f64,
    depth: usize,
    frequency: usize,
    score: f64,
}
impl Matches {
    pub fn new_regex(
        url: &Url,
        text: impl AsRef<str>,
        regex: &Vec<Regex>,
        depth: usize,
        frequency: usize,
    ) -> Matches {
        let x = regex
            .iter()
            .map(|r| r.find_iter(text.as_ref()).count())
            .average();

        Matches {
            url: url.clone(),
            n_matches: x,
            html: text.as_ref().to_string(),
            depth,
            frequency,
            score: 0_f64,
        }
    }

    pub fn new_strings_insensitive(
        url: &Url,
        text: impl AsRef<str>,
        keywords: &Vec<String>,
        depth: usize,
        frequency: usize,
    ) -> Matches {
        let x = keywords
            .iter()
            .map(|k| {
                text.as_ref()
                    .to_lowercase()
                    .matches(&k.to_lowercase())
                    .count()
            })
            .average();

        Matches {
            url: url.clone(),
            n_matches: x,
            html: text.as_ref().to_string(),
            depth,
            frequency,
            score: 0_f64,
        }
    }

    pub fn new_strings_sensitive(
        url: &Url,
        text: impl AsRef<str>,
        keywords: &Vec<String>,
        depth: usize,
        frequency: usize,
    ) -> Matches {
        let x = keywords
            .iter()
            .map(|k| text.as_ref().matches(k).count())
            .average();

        Matches {
            url: url.clone(),
            n_matches: x,
            html: text.as_ref().to_string(),
            depth,
            frequency,
            score: 0_f64,
        }
    }

    pub fn sort_vec(a: &Self, b: &Self) -> Ordering {
        let w_1 = -0.7;
        let w_2 = 1.2;
        let w_3 = 2.5;

        let calc =
            |x: &Self| x.depth as f64 * w_1 + x.frequency as f64 * w_2 + x.n_matches as f64 * w_3;

        let w_a = calc(a);
        let w_b = calc(b);

        w_a.total_cmp(&w_b)
    }

    pub fn to_page(self) -> Page {
        Page {
            url: self.url,
            score: self.score,
        }
    }
}

fn sort(mut list: Vec<Matches>, params: UserParams, modifiers: Vec<UserModifier>) -> Vec<Matches> {
    let w_1 = params.depth_w;
    let w_2 = params.frequency_w;
    let w_3 = params.n_matches_w;

    let calc =
        |x: &Matches| x.depth as f64 * w_1 + x.frequency as f64 * w_2 + x.n_matches as f64 * w_3;

    let mut vec = update_values(list.into_iter()
        .map(|mut x| {
            x.score = calc(&x);
            x
        })
        .collect::<Vec<_>>(), &modifiers);
        

    vec.sort_by(|a, b| {
        let w_a = a.score;
        let w_b = b.score;

        w_b.total_cmp(&w_a)
    });

    vec
}


fn update_values(vec1: Vec<Matches>, vec2: &Vec<UserModifier>) -> Vec<Matches> {
    let mut vec = Vec::new();
    
    for mut entry1 in vec1.into_iter() {
        if let Some(entry2) = vec2.iter().find(|entry2| {
            match entry2.pattern {
                crate::Patt::String(ref string) => entry1.url.to_string().contains(string.as_str()),
                crate::Patt::Regex(ref regex) => regex.is_match(&entry1.url.to_string()),
            }
        }) {
            match entry2.value_modifier {
                crate::UserMod::Boost(value) => {entry1.score += value; vec.push(entry1)},
                crate::UserMod::Penal(value) => {entry1.score -= value; vec.push(entry1)},
                crate::UserMod::Ban => (),
            }
        } else {
            vec.push(entry1);
        }
    }

    vec
}

#[derive(Debug)]
pub struct Page {
    pub url: Url,
    pub score: f64
}

impl Serialize for Page {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_map(Some(2))?;
        state.serialize_entry("url", &self.url.to_string()).unwrap();
        state.serialize_entry("score", &self.score).unwrap();
        state.end()
    }
}

pub trait Sorter {
    fn sort_regex(&self, regex: Vec<Regex>, params: UserParams, modifiers: Vec<UserModifier>) -> Vec<Page>;
    fn sort_strings_insensitive(&self, keywords: Vec<String>, params: UserParams, modifiers: Vec<UserModifier>) -> Vec<Page>;
    fn sort_strings_sensitive(&self, keywords: Vec<String>, params: UserParams, modifiers: Vec<UserModifier>) -> Vec<Page>;
}

impl Sorter for Indexer {
    fn sort_regex(&self, regex: Vec<Regex>, params: UserParams, modifiers: Vec<UserModifier>) -> Vec<Page> {
        let x = self
            .iter()
            .map(|(url, (depth, frequency, html))| {
                Matches::new_regex(url, html, &regex, *depth, *frequency)
            })
            .collect::<Vec<Matches>>();
        let x = sort(x, params, modifiers);
        x.into_iter().map(|y| y.to_page()).collect::<Vec<Page>>()
    }

    fn sort_strings_insensitive(&self, keywords: Vec<String>, params: UserParams, modifiers: Vec<UserModifier>) -> Vec<Page> {
        let x = self
            .iter()
            .map(|(url, (depth, frequency, html))| {
                Matches::new_strings_insensitive(url, html, &keywords, *depth, *frequency)
            })
            .collect::<Vec<Matches>>();
        let x: Vec<Matches> = sort(x, params, modifiers);
        std::fs::write("idkk", format!("{x:#?}")).unwrap();
        x.into_iter().map(|y| y.to_page()).collect::<Vec<Page>>()
    }

    fn sort_strings_sensitive(&self, keywords: Vec<String>, params: UserParams, modifiers: Vec<UserModifier>) -> Vec<Page> {
        let x = self
            .iter()
            .map(|(url, (depth, frequency, html))| {
                Matches::new_strings_insensitive(url, html, &keywords, *depth, *frequency)
            })
            .collect::<Vec<Matches>>();
        let x = sort(x, params, modifiers);
        std::fs::write("idkk", format!("{x:#?}")).unwrap();
        x.into_iter().map(|y| y.to_page()).collect::<Vec<Page>>()
    }
}

trait Average {
    type Output;
    fn average(self) -> Self::Output;
}

// impl<I, X> Average for I
// where
//     I: Iterator<Item = X> + Clone, // Ensure the iterator can be cloned
//     X: Add<Output = X> + Div<Output = X> + Sum<X> + Copy, // Ensure X can be added, divided, and summed
//     f64: TryFrom<X>, // Ensure we can convert X to f64
//     <f64 as TryFrom<X>>::Error: std::fmt::Debug,
// {
//     type Output = f64;

//     fn average(self) -> Self::Output {
//         // Collect the iterator into a Vec for counting
//         let collected: Vec<X> = self.clone().collect();

//         // Calculate sum and count
//         let sum: X = collected.iter().cloned().sum();
//         let count = collected.len() as f64;

//         // Convert sum to f64 and return the average
//         f64::try_from(sum).unwrap() / count
//     }
// }

impl<I> Average for I
where
    I: Iterator<Item = usize> + Clone,
{
    type Output = f64;

    fn average(self) -> Self::Output {
        // Collect the iterator into a Vec for counting
        let collected: Vec<usize> = self.clone().collect();

        // Calculate sum and count
        let sum: usize = collected.iter().cloned().sum();
        let count = collected.len() as f64;

        // Convert sum to f64 and return the average
        sum as f64 / count
    }
}
