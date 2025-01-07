use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SliderProps {
    pub class: String,
    pub state: UseStateHandle<bool>,
    pub text: String,
}

#[function_component]
pub fn Slider(SliderProps { state, class, text }: &SliderProps) -> Html {
    // let use_state = use_state(|| false);
    
    let when_pressed = {
        let state = state.clone();
        Callback::from(move |_| {
        
        state.set(!*state);
    })};

    html! {
        <label class="switch">
            <input type="checkbox" checked={**state} onclick={when_pressed} />
            <span class={class}>{text}</span>
        </label>
    }
}