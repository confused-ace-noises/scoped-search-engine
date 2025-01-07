use yew::prelude::*;

#[function_component(LoadingPage)]
pub fn loading_page() -> Html {
    html! {
        <div>
            <h1>{ "Loading..." }</h1>
            <p>{ "Please wait while we fetch your results. This could take up to 10 minutes." }</p>
        </div>
    }
}