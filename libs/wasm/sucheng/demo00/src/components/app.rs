use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {}
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <button>{ "my button" }</button>
        }
    }
}
