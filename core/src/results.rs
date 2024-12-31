use std::fmt::format;

use serde::{Deserialize, Serialize};
use yew::{function_component, html, platform::spawn_local, use_state, Callback, Html, InputEvent, Properties, SubmitEvent, TargetCast};
use yew_router::{self, components, hooks::use_navigator, BrowserRouter, Routable};
use reqwasm::{self, http::Request};
use crate::pages::Pages;

#[derive(Debug, PartialEq, Properties, Serialize)]
pub struct SearchProps {
    pub starting_url: String,
    pub query: Vec<String>, // Use appropriate data types for the query, e.g., String for simple text search
    pub modifiers: Vec<(String, i8, f64, u8)>, // url, (-1: Penal, 0: Ban, 1: Boost), value, (0: string, 1: regex)
    pub params: (f64, f64, f64),
    pub max_depth: usize,
    pub query_type: u8, // 0: strings sensitive, 1: strings insensitive, 2: regexes
}

#[derive(Debug, PartialEq, Properties, Clone, Deserialize)]
struct ChildProps {
    url: String,
    score: f64,
    title: String
} 

#[function_component(ResultsPage)]
pub fn results_page() -> Html {
    // let navigator = use_navigator().unwrap();
    // let onclick = Callback::from(move |_| navigator.push(&Pages::Home));
    
    let results = use_state(|| vec![]);
    println!("?");
    let on_search = {
        let results = results.clone();
        Callback::from(move |_| {
            let results = results.clone();
            spawn_local(async move {
                let payload = serde_json::json!(SearchProps { 
                    starting_url: "https://askiiart.net/".to_string(), 
                    query: vec!["askiiart".to_string()], 
                    modifiers: vec![], 
                    params: (-0.7, 1.7, 2.5), 
                    max_depth: 1, 
                    query_type: 0 
                });
                gloo::console::log!(format!("{}", payload));
                // Send POST request
                if let Ok(response) = Request::post("http://localhost:6728/api/search_any")
                .header("Allow-Control-Allow-Origin", "*")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .send()
                    .await
                {
                    let data= response.json::<Vec<ChildProps>>().await.unwrap();
                    results.set(data);
                    gloo::console::log!("alr made it");
                    println!("alr")
                } else {
                    println!("fuuuuu");
                    gloo::console::log!("fuuuu");
                }
            });
        })
    };

    println!("{:#?}", results);

    html!{
        <div id="results-div">
            <button onclick={on_search}> { "Search "}</button>
            // <button {onclick}> { "Search "}</button>
            <ul>
                { for results.iter().map(|child_props| {
                    let title = child_props.title.clone();
                    let url = child_props.url.clone();
                    let score = child_props.score;
                    html!{ <ResultChild{url}{score}{title} />}
                }) }
            </ul>
        </div>
    }
}

#[function_component(ResultChild)]
fn results_page_child(child_props: &ChildProps) -> Html {
    let child_props = child_props.clone();

    html! {
        <div id="result">
            <a href={ child_props.url.clone() }>{ child_props.title }{":    "}{ child_props.score }</a>
            <p>{ child_props.url }</p>
        </div>
    }
}