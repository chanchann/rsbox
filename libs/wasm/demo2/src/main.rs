use yew::prelude::*;

enum Msg {
    AddOne,
    MinusOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::MinusOne => {
                if self.value > 0 {
                    self.value -= 1;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)} class="btn">{ "+1" }</button>
                <p class="result">{ self.value }</p>
                <button onclick={link.callback(|_| Msg::MinusOne)} class="btn">{ "-1" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}