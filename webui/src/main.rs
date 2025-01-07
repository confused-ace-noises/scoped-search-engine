// use webui::components::switch::Switch;
use app::App;
use webui::app;
use yew::{function_component, html, use_state, Html};

// #[function_component]
// fn App() -> Html {
//     let state = use_state(|| false);
//     let text = ".*".to_string();


//     html! {
//         <>
//             <Switch class="slider round" state={state.clone()} {text}/>
//             if *state {
//                 {"haiiii"}
//             }
//         </>
//     }
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}