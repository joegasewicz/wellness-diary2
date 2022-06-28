use yew::{Component, Context, Html, html};

pub struct DayComponent;

impl Component for DayComponent {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h3>{"Day's Details"}</h3>
                <div class="card" style="width: 18rem;">
                  <ul class="list-group list-group-flush">
                    <li class="list-group-item">{"An item"}</li>
                  </ul>
                </div>
            </>
        }
    }
}