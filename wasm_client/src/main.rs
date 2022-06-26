mod components;
mod core;

use yew::prelude::*;

use components::dairy::DiaryComponent;
use components::day_form::DayFormComponent;

enum Diary {

}

struct App {

}

impl Component for App {
    type Message = Diary;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {

        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div  class="container">
                <h1 class="text-center">{"Wellness Diary"}</h1>
                <div class="row">
                    <div class="col-md-6">
                        <DiaryComponent />
                    </div>
                    <div class="col-md-6">
                        <DayFormComponent />
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}