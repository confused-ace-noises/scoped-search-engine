use yew::prelude::*;
use crate::components::pagination::Pagination;

#[function_component(ResultsPage)]
pub fn results_page() -> Html {
    html! {
        <div>
            <h1>{ "Search Results" }</h1>
            <input type="text" placeholder="Refine search" />
            <ul>
                // Simulating results as a placeholder
                <li>{ "Result 1" }</li>
                <li>{ "Result 2" }</li>
                <li>{ "Result 3" }</li>
            </ul>
            <Pagination total_items={100} items_per_page={25} />
        </div>
    }
}