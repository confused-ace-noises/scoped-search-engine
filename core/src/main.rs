use yew::prelude::*;

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

    let onsubmit = {
        let search_query = search_query.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // Prevent page reload
            gloo::console::log!(format!("Search query: {}", *search_query));
        })
    };

    // html! {
    //     <div style="display: flex; justify-content: center; margin-top: 100px;">
    //         <form style="display: flex;" id="search-form" {onsubmit}>
    //             <input
    //                 type="text"
    //                 id="search-input"
    //                 placeholder="Search..."
    //                 style="width: 300px; padding: 10px; border: 1px solid #ccc; border-radius: 4px;"
    //                 value={(*search_query).clone()}
    //                 oninput={on_input}
    //             />
    //             <button
    //                 type="submit"
    //                 style="padding: 10px 15px; margin-left: 5px; background-color: #007BFF; color: white; border: none; border-radius: 4px; cursor: pointer;"
    //             >
    //                 { "Search" }
    //             </button>
    //         </form>
    //     </div>
    // }

    html! {
        <div class="search-container">
            <form id="search-form" {onsubmit}>
                <input
                    type="text"
                    id="search-input"
                    placeholder="Search..."
                    value={(*search_query).clone()}
                    oninput={on_input}
                />
                <button type="submit">{ "Search" }</button>
            </form>
        </div>
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "My Search Engine" }</h1>
            <SearchBar />
        </>
    }

    // html! {
    //     <div>
    //         <h1> {"something"} </h1>
    //     </div>
    // }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
