use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast};
use yew_router::{self, components, hooks::use_navigator, switch, BrowserRouter, Routable};

use crate::results::{ResultsPage, SearchProps};

#[function_component(SearchBar)]
fn search_bar() -> Html {
    let search_query = use_state(|| "".to_string());

    let on_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                search_query.set(input.value());
            }
        })
    };

    let navigator = use_navigator().unwrap();
    // let onsubmit = {
    //     let search_query = search_query.clone();
    //     Callback::from(move |e: SubmitEvent| {
    //         e.prevent_default(); // Prevent page reload
    //         gloo::console::log!(format!("Search query: {}", *search_query));
    //     })
    // };

    let onsubmit = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let props = SearchProps { 
                starting_url: "https://example.com".to_string(), 
                query: vec!["example".to_string(), "query".to_string()], 
                modifiers: vec![], 
                params: (-0.7, 1.7, 2.5), 
                max_depth: 1, 
                query_type: 0
            };
                

            navigator.push_with_state(
                &Pages::ResultsPage,
                Some(props), // Pass state
            );
        })
    };

    html! {
        <>
        <div class="url-crawl-container"> 
            <form id="url-form">
                <input  type="text"
                    id="crawl-input"
                    placeholder="Url to crawl..."
                    autocomplete="off"
                    value={(*search_query).clone()}
                />
                
                <input type="number"
                    id="depth-to-reach"
                    placeholder="Depth..."
                    autocomplete="off"
                    min="0"
                    
                />
            </form>
        </div>

        <div class="search-container">
            <form id="search-form" {onsubmit}>
                <button id="regex">{".*"}</button> 
                
                <input
                    type="text"
                    id="search-input"
                    placeholder="Search..."
                    autocomplete="off"
                    value={(*search_query).clone()}
                    oninput={on_input}
                />
                <button type="submit">{ "Search" }</button>
            </form>
        </div>
        </>
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Pages {
    #[at("/")]
    Home,
    #[at("/results")]
    ResultsPage
}

fn switch(routes: Pages) -> Html {
    match routes {
        Pages::Home => html! { <SearchBar /> },
        Pages::ResultsPage => html! { <ResultsPage /> },
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <yew_router::Switch<Pages> render={switch} />
        </BrowserRouter> 
    }
}