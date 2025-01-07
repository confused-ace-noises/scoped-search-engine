use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub total_items: usize,
    pub items_per_page: usize,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let total_pages = (props.total_items as f64 / props.items_per_page as f64).ceil() as usize;

    html! {
        <div>
            { (1..=total_pages).map(|page| html! {
                <button>{ format!("Page {}", page) }</button>
            }).collect::<Html>() }
        </div>
    }
}