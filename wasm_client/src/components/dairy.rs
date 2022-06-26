use yew::{Component, Context, Html, html};

pub struct DiaryComponent;

impl Component for DiaryComponent  {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>
                    <h2>{"Dairy"}</h2>
                    <input
                        type="date"
                    />
                </div>
            </>
        }
    }
}