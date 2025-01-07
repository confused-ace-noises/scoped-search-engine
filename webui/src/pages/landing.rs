use crate::{components::switch::Slider, Route};
use yew::prelude::*;
use yew_router::prelude::*;


// #[function_component(LandingPage)]
// pub fn landing_page() -> Html {
//     let navigator = use_navigator().unwrap();
//     let search_query = use_state(|| "".to_string());
//     let depth = use_state(|| "0".to_string());
//     let is_regex = use_state(|| false);
//     let is_case_sensitive = use_state(|| false);

//     // Handle search query input change
//     let on_search_query_change = {
//         let search_query = search_query.clone();
//         Callback::from(move |e: InputEvent| {
//             if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
//                 search_query.set(input.value());
//             }
//         })
//     };

//     // Handle depth input change
//     let on_depth_change = {
//         let depth = depth.clone();
//         Callback::from(move |e: InputEvent| {
//             if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
//                 depth.set(input.value());
//             }
//         })
//     };

//     // Handle toggle switches
//     let on_regex_toggle = {
//         let is_regex = is_regex.clone();
//         Callback::from(move |_| {
//             is_regex.set(!*is_regex);
//         })
//     };

//     let on_case_sensitive_toggle = {
//         let is_case_sensitive = is_case_sensitive.clone();
//         Callback::from(move |_| {
//             is_case_sensitive.set(!*is_case_sensitive);
//         })
//     };

//     // Form submission
//     let onsubmit = {
//         let search_query = search_query.clone();
//         let depth = depth.clone();
//         Callback::from(move |e: FocusEvent| {
//             e.prevent_default(); // Prevent form from reloading the page
//             gloo::console::info!(format!(
//                 "Search Query: {}; Depth: {}; Regex: {}; Case Sensitive: {}",
//                 *search_query,
//                 *depth,
//                 *is_regex,
//                 *is_case_sensitive
//             ));
//         })
//     };

//     html! {
//         <form id="main-form" {onsubmit}>
//             <div class="start-url-container">
//                 <input
//                     type="text"
//                     id="crawl-input"
//                     placeholder="Enter search query"
//                     value={(*search_query).clone()}
//                     oninput={on_search_query_change}
//                 />
//                 <input
//                     type="number"
//                     id="depth-input"
//                     min="0"
//                     value={(*depth).clone()}
//                     oninput={on_depth_change}
//                 />
//             </div>

//             <div class="search-container">
//                 <div class="slider-stack">
//                     <label>
//                         <input type="checkbox" checked={*is_regex} onclick={on_regex_toggle} />
//                         {"Regex"}
//                     </label>
//                     <label>
//                         <input type="checkbox" checked={*is_case_sensitive} onclick={on_case_sensitive_toggle} />
//                         {"Case Sensitive"}
//                     </label>
//                 </div>

//                 <button type="submit">{"Search"}</button>
//             </div>
//         </form>
//     }
// }



#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let navigator = use_navigator().unwrap();
    // let input_value = use_state(String::new);
    let is_regex = use_state(|| false);
    let is_case_sensitive = use_state(|| false);
    let depth = use_state(|| "0");
    let search_query = use_state(|| "".to_string());

    let on_search_query_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                search_query.set(input.value());
            }
        })
    };

    let onsubmit = {
        // let navigator = navigator.clone();
        let search_query = search_query.clone();
        let depth = depth.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            gloo::console::info!(format!("search: {}; depth: {}", *search_query, *depth));

        })
    };

    html! {
        <>
        // <a>{"settings"}</a>
        <form id="main-form" onsubmit={onsubmit}>
        

            <div class="start-url-container">
                <input type="text" id="crawl-input" value={(*search_query).clone()} oninput={on_search_query_change} />
                <input type="number" id="depth-input" min="0" value={*depth}/>
            </div>

            <div class="search-container">
                <div class="slider-stack">
                    <Slider class="slider" state={is_regex} text={""} />
                    if *is_case_sensitive {
                        <Slider class="slider" state={is_case_sensitive} text={""} />
                    } else {
                        <Slider class="case_sensitive_slider" state={is_case_sensitive} text={""} />
                    }
                </div>

                <input type="text" />

                <button type="submit">{"search"}</button>
            </div>
        </form>
        </>
    }
}

// html! {
// <>
//     <div class="url-crawl-container">
//         <form id="url-form">
//             <input  type="text"
//                 id="crawl-input"
//                 placeholder="Url to crawl..."
//                 autocomplete="off"
//                 value={(*search_query).clone()}
//             />

//             <input type="number"
//                 id="depth-to-reach"
//                 placeholder="Depth..."
//                 autocomplete="off"
//                 min="0"

//             />
//         </form>
//     </div>

//     <div class="search-container">
//         <form id="search-form" {onsubmit}>
//             // <button id="regex">{".*"}</button>

//             <Slider state={is_regex} class="slider round" text="" />

//             <input
//                 type="text"
//                 id="search-input"
//                 placeholder="Search..."
//                 autocomplete="off" 
//                 value={(*search_query).clone()}
//             />
//             <button type="submit">{ "Search" }</button>
//         </form>
//     </div>
// </>
// }
