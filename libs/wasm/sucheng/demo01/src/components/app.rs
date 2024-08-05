extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink,InputData, Html, ShouldRender};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub struct App {
    link: ComponentLink<Self>,
    user_name: String,
    user_pass: String,
}
pub enum Msg {
    BtnClick,
    SetInput(InputData),
}
impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link: link,
            user_name: "".to_string(),
            user_pass: "".to_string(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::BtnClick => alert(self.user_name.as_str()),
            Msg::SetInput(e) => {
                self.user_name = e.value;
            }
        }
        true
    }
    fn view(&self) -> Html {
        html! {
            <div id="app">
               <div><span>{"Username:"}</span>
             <input  type="text" oninput=self.link.callback(|e:InputData| Msg::SetInput(e))/>
               </div>
               <div><span>{"Password:"}</span>
                  <input type="text"/>
               </div>
               <div>
                  <button onclick=self.link.callback(|_| Msg::BtnClick )>{ "button" }</button>
               </div>

            </div>
        }
    }
}
