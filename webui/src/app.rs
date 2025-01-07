use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::pages::{LandingPage, LoadingPage, ResultsPage};

use crate::pages;



#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Landing => html! { <LandingPage /> },
        Route::Loading => html! { <LoadingPage /> },
        Route::Results => html! { <ResultsPage /> },
    }
}