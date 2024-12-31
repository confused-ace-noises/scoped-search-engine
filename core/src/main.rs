use yew::prelude::*;
use core::pages::Main;



// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <>
//             <h1>{ "My Search Engine" }</h1>
//             <SearchBar />
//         </>
//     }

//     // html! {
//     //     <div>
//     //         <h1> {"something"} </h1>
//     //     </div>
//     // }
// }

fn main() {
    yew::Renderer::<Main>::new().render();
}
