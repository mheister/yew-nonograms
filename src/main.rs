mod components;
mod models;

use crate::components::board::{Board as BoardComponent, BoardMode};

use web_sys::HtmlInputElement;
use yew::{prelude::*, TargetCast};

struct NonogramGame {
    mode: BoardMode,
}

enum NonogramGameMsg {
    SetMode(BoardMode),
}

impl Component for NonogramGame {
    type Message = NonogramGameMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            mode: BoardMode::Solve,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NonogramGameMsg::SetMode(new_mode) => {
                let update = self.mode != new_mode;
                self.mode = new_mode;
                update
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mode_selector_onchange = ctx.link().batch_callback(|evt: Event| {
            let target = evt.target_dyn_into::<HtmlInputElement>();
            target.map(|checkbox| {
                if checkbox.checked() {
                    NonogramGameMsg::SetMode(BoardMode::Set)
                } else {
                    NonogramGameMsg::SetMode(BoardMode::Solve)
                }
            })
        });
        html! {
            <>
            <h1>{"Nonogram Game"}</h1>
            <div class={"content-box"}>
                <BoardComponent mode={self.mode}/>
                <p><input type={"checkbox"} id={"modeselector"} onchange={mode_selector_onchange}/>
                <label for={"modeselector"}>{"Set Puzzle"}</label></p>
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
