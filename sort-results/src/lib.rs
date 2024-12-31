pub mod matches;

use actix_web::{body::BoxBody, HttpResponse, Responder};
pub use indexer::indexer_maker::Indexer;
use matches::Page;
use regex::Regex;
use url::Url;
use serde::{self, Serialize};


pub struct Pages(pub Vec<Page>);

impl Serialize for Pages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        serde::Serialize::serialize(&self.0, serializer)
    }
}

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
    pub value_modifier: UserMod,
    pub pattern: Patt
}

pub struct UserParams {
    pub depth_w: f64,
    pub frequency_w: f64,
    pub n_matches_w: f64,
}

impl Default for UserParams {
    fn default() -> Self {
        Self { depth_w: -0.7, frequency_w: 1.7, n_matches_w: 2.5 }
    }
}

impl Responder for Pages {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(self) // This automatically converts self to JSON using Serde
    }
}