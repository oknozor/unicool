extern crate unicool_lib;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    value: String,
}

pub enum Msg {
    GotInput(String),
    Save,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { value: "".into() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = unicool_lib::convert_non_ascii_to_unicode(&new_value)
                    .expect("ascii transform error");
            }
            Msg::Save => {
                self.value = "unimplemented".to_string();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <textarea rows=5,
                        value=&self.value,
                        oninput=|e| Msg::GotInput(e.value),
                        placeholder="placeholder",>
                    </textarea>
                     <button onclick=|_| Msg::Save,>{ "save ouptut" }</button>
                </div>
                <div>
                    {&self.value}
                </div>
            </div>
        }
    }
}
