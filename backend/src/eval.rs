use crate::{user_options::{Query, UserModType, UserModifiers, UserOptions}, Error, W};
use url::Url;
use reqwest::{self, Client};
use select::{document::Document, predicate::Class};
use rayon::{self, iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator}};

pub struct Page {
    pub(crate) url: W<Url>, 
    pub(crate) title: String,
    pub(crate) score: f64,
    pub(crate) modifier: Option<UserModType>
} impl Page {
    pub fn modify_score(self, modifiers: &UserModifiers) -> Self {
        modifiers.modify_page_score(self)
    }
}

pub struct Html {
    url: W<Url>,
    html: String,
    depth: usize,
} impl Html {
    /// creates a new Html, by making a request to the specified Url
    pub async fn new(url: impl AsRef<str>, depth: usize) -> Result<Html, Error> {
        let url = Url::parse(url.as_ref())?;
        let response = reqwest::get(url.clone()).await?;
        let text = response.text().await?;

        Ok(Html {
            url: W(url),
            html: text,
            depth
        })
    }

    pub async fn new_client(url: impl AsRef<str>, client: &Client, depth: usize) -> Result<Html, Error> {
        let url1 = Url::parse(url.as_ref())?;
        let response = client.get(url.as_ref()).send().await?;
        let text = response.text().await?;

        Ok(Html {
            url: W(url1),
            html: text,
            depth
        })
    }

    pub async fn evaluate(self, query: &Query, user_options: &UserOptions, n_mentions: usize) -> Result<Page, Error> {
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

        let partial_score = user_options.parameters.calculate_score(self.depth, n_mentions, n_matches);

        let page_partial = Page{url: self.url, title, score: partial_score, modifier: None};
        
        let page: Page;

        if let Some(modifier) = user_options.modifiers.par_iter().find_any(|modifier| modifier.matches_with(&page_partial)) {
            page = modifier.modify_page_score(page_partial);
        } else {
            page = page_partial
        }

        Ok(page)
    }
}