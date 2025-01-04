use crate::server::reply::ReplyPage;
use crate::utils::misc::{SplitToString, Occurrence};
use crate::{
    main_logic::user_options::{Query, UserModType, UserModifiers, UserOptions},
    Error, W,
};
use rayon::{
    self,
    iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
};
use reqwest::{self, Client};
use rocket::time::Duration;
use select::{
    document::Document,
    predicate::{Class, Name},
};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorePage {
    pub(crate) url: W<Url>,
    pub(crate) title: String,
    pub(crate) score: f64,
    pub(crate) modifier: Option<UserModType>,
}
impl ScorePage {
    pub fn modify_score(self, modifiers: &UserModifiers) -> Self {
        modifiers.modify_page_score(self)
    }

    pub fn to_page(&self) -> Page {
        Page { url: self.url.clone(), title: self.title.clone() }
    }

    pub fn to_reply_page(&self) -> ReplyPage {
        ReplyPage {
            url: self.url.to_string(),
            title: self.title.clone(),
            score: self.score
        }
    }
}

pub struct Page {
    pub url: W<Url>,
    pub title: String,
}

pub struct Html {
    pub url: W<Url>,
    html: String,
    depth: usize,
}
impl Html {
    /// creates a new Html, by making a request to the specified Url
    pub async fn new(url: impl AsRef<str>, depth: usize) -> Result<Html, Error> {
        let url = Url::parse(url.as_ref())?;
        let response = reqwest::get(url.clone()).await?;
        let text = response.text().await?;

        Ok(Html {
            url: W(url),
            html: text,
            depth,
        })
    }

    pub async fn new_client(
        url: impl AsRef<str>,
        client: &Client,
        depth: usize,
    ) -> Result<Html, Error> {
        let url1 = Url::parse(url.as_ref())?;
        let timeout_duration = std::time::Duration::from_secs(10);

        let response = client.get(url.as_ref()).timeout(timeout_duration).send().await?;
        let text = response.text().await?;

        Ok(Html {
            url: W(url1),
            html: text,
            depth,
        })
    }

    pub fn get_links(&self) -> Result<Vec<W<Url>>, Error> {
        let mut res = Vec::new();
        let document = Document::from(self.html.as_str());
        for item in document.find(Name("a")) {
            if let Some(href) = item.attr("href") {
                if let Ok(href) = self.url.join(href) {
                    let fixed_url = W(Url::parse(
                        &href
                            .to_string()
                            .split_to_string_at_occurrence_tuple("#", Occurrence::Last)
                            .0,
                    )?);

                    if !res.contains(&fixed_url) {
                        res.push(fixed_url);
                    }
                }
            }
        }
        Ok(res)
    }

    pub async fn evaluate(
        self,
        query: &Query,
        user_options: &UserOptions,
        n_mentions: usize,
    ) -> Result<ScorePage, Error> {
        let searchers = query.to_searchers()?;
        let text = self.html;
        let title = {
            let document = Document::from(text.as_str());
            if let Some(title) = document.find(Class("title")).next() {
                title.text()
            } else {
                self.url.to_string()
            }
        };
        let n_matches: usize = searchers.into_par_iter().map(|x| x.search(&text)).sum();
        drop(text);

        let partial_score = user_options
            .parameters
            .calculate_score(self.depth, n_mentions, n_matches);

        let page_partial = ScorePage {
            url: self.url,
            title,
            score: partial_score,
            modifier: None,
        };

        let page: ScorePage;

        if let Some(modifier) = user_options
            .modifiers
            .par_iter()
            .find_any(|modifier| modifier.matches_with(&page_partial.to_page()))
        {
            page = modifier.modify_page_score(page_partial);
        } else {
            page = page_partial
        }

        Ok(page)
    }

    pub async fn evaluate_no_n_mentions(
        self,
        query: &Query,
        user_options: &UserOptions,
    ) -> Result<ScorePage, Error> {
        let searchers = query.to_searchers()?;
        let text = self.html;
        let title = {
            let document = Document::from(text.as_str());
            if let Some(title) = document.find(Class("title")).next() {
                title.text()
            } else {
                self.url.to_string()
            }
        };
        let n_matches: usize = searchers.into_par_iter().map(|x| x.search(&text)).sum();
        drop(text);

        let partial_score = user_options
            .parameters
            .calculate_score_no_freq(self.depth, n_matches);

        let page_partial = ScorePage {
            url: self.url,
            title,
            score: partial_score,
            modifier: None,
        };

        let page: ScorePage;

        if let Some(modifier) = user_options
            .modifiers
            .par_iter()
            .find_any(|modifier| modifier.matches_with(&page_partial.to_page()))
        {
            page = modifier.modify_page_score(page_partial);
        } else {
            page = page_partial
        }

        Ok(page)
    }

    pub fn title(&self) -> String {
        let document = Document::from(self.html.as_str());
        if let Some(title) = document.find(Name("title")).next() {
            title.text()
        } else {
            self.url.to_string()
        }
    }
}
