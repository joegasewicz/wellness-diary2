use yew::{Component, Context, Html, html};

pub struct DayFormComponent;

impl Component for DayFormComponent {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{"Day 1"}</h2>
                <form>
                    <label>{"Work"}</label>
                    <input name="work" value="" />
                </form>
            </>
        }
    }

}