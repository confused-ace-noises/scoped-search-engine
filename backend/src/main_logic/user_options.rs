use serde::{Deserialize, Serialize};

use crate::{main_logic::eval::{Page, ScorePage}, utils::searcher::Searcher, Error};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UserModType {
    Boost(f64),
    Penalize(f64),
    Ban,
}

#[derive(Debug, Clone)]
pub struct UserModifiers {
    pub modification_type: UserModType,
    pub searcher: Searcher,
}
impl UserModifiers {
    pub fn modify_page_score(&self, mut page: ScorePage) -> ScorePage {
        if self.searcher.search(page.url.as_str()) != 0
            || self.searcher.search(page.title.clone()) != 0
        {
            page.modifier = Some(self.modification_type);

            match self.modification_type {
                UserModType::Boost(value) => page.score += value,
                UserModType::Penalize(value) => page.score -= value,
                UserModType::Ban => (),
            }
        }

        page
    }

    pub fn matches_with(&self, page: &Page) -> bool {
        self.searcher.search(page.url.as_str()) != 0
            || self.searcher.search(page.title.clone()) != 0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserParameters {
    depth_coefficient: f64,
    mention_frequency_coefficient: f64,
    n_matches_coefficient: f64,
}
impl UserParameters {
    pub fn new(
        depth_coefficient: f64,
        mention_frequency_coefficient: f64,
        n_matches_coefficient: f64,
    ) -> UserParameters {
        UserParameters {
            depth_coefficient,
            mention_frequency_coefficient,
            n_matches_coefficient,
        }
    }

    pub fn calculate_score(&self, depth: usize, frequency: usize, n_matches: usize) -> f64 {
        depth as f64 * self.depth_coefficient
            + frequency as f64 * self.mention_frequency_coefficient
            + n_matches as f64 * self.n_matches_coefficient
    }

    pub fn calculate_score_no_freq(&self, depth: usize, n_matches: usize) -> f64 {
        depth as f64 * self.depth_coefficient
            + n_matches as f64 * self.n_matches_coefficient
    }

    pub fn calculate_score_freq(&self, current_score: f64, frequency: usize) -> f64 {
        current_score + frequency as f64 * self.mention_frequency_coefficient
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserOptions {
    pub parameters: UserParameters,
    pub modifiers: Vec<UserModifiers>,
}
impl UserOptions {
    pub fn new(parameters: UserParameters, modifiers: Vec<UserModifiers>) -> Self {
        UserOptions {
            parameters,
            modifiers,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Query {
    Regex(Vec<String>),
    InsensitiveString(String),
    SensitiveString(String),
}
impl Query {
    pub fn to_searchers(&self) -> Result<Vec<Searcher>, Error> {
        match self {
            // checks if any of them error'es, if even one does, return Err, else return the vector
            Query::Regex(regexes) => {
                let mut regex_iter = regexes.into_iter().map(|r| Searcher::from_regex(r));

                if regex_iter.any(|regex| regex.is_err()) {
                    Err(regex_iter
                        .find(|element| element.is_err())
                        .unwrap()
                        .unwrap_err())
                } else {
                    Ok(regex_iter.map(|regex| regex.unwrap()).collect())
                }
            }
            // splits at ascii whitespace and returns the searchers with one word
            Query::SensitiveString(query_string) => Ok(query_string
                .split_ascii_whitespace()
                .map(|string| Searcher::from_string(string, true))
                .collect()),

            Query::InsensitiveString(query_string) => Ok(
                query_string.to_ascii_lowercase()
                .split_ascii_whitespace()
                .map(|string| Searcher::from_string(string, false))
                .collect()
            )
        }
    }
}
