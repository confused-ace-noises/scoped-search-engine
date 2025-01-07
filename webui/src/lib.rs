use yew;
use yew_router::Routable;

pub mod pages;
pub mod app;
pub mod components;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Landing,
    #[at("/loading")]
    Loading,
    #[at("/results")]
    Results,
}