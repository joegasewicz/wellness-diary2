use yew::{
    Component,
    Context,
    html,
    Html,
    Properties,
    events::Event,
};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use log::{info};

pub struct DayFormComponent {
    day: String,
}

pub enum Msg {
    Click,
    DayValue(String),
}


impl Component for DayFormComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            day: "".parse().unwrap(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => info!("Form Submitted!"),
            Msg::DayValue(day) => self.day = day,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{"Day 1"}</h2>
                <form>
                    <div class="mb-3">
                        <label>{"Day"}</label>
                        <input name="day" onchange={ctx.link().callback(|e: Event| {
                            let target: EventTarget = e
                                .target()
                                .expect("error details");

                            Msg::
                        })} />
                    </div>
                    <button onclick={ctx.link().callback(|_| Msg::Click)} type="button" class="btn btn-success btn-lg">
                        {"Add"}
                    </button>
                </form>
            </>
        }
    }

}