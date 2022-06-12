mod models;
mod components;

use crate::models::board::Board;
use crate::components::board::Board as BoardComponent;

use yew::prelude::*;

struct NonogramGame {
}

impl Component for NonogramGame {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h1>{"Nonogram Game"}</h1>
            <div class={"content-box"}>
                <BoardComponent/>
            </div>
            </>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}

fn main() {
    yew::start_app::<NonogramGame>();
}
