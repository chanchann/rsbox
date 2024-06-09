use gloo::console::log;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // log!("123");
    html! {
        <h1>{ "Hello World" }</h1>
    }
}
