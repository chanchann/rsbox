use yew::{html, Component, ComponentLink, Html, ShouldRender};
pub struct App {}
pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <button>{ "clickme" }</button>
        }
    }
}