mod components;
mod models;

use crate::components::board::{Board as BoardComponent, BoardMode};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/solve/:puzzle")]
    Solve { puzzle: String },
    #[at("/set/:puzzle")]
    Set { puzzle: String },
    #[at("/set")]
    SetNew,
}

fn switch(route: &Route) -> Html {
    let (mode, puzzle) = match route {
        Route::Home => {
            return html! {
                <Redirect<Route> to={Route::Set{puzzle: "".to_owned()}}/>
            }
        }
        Route::Solve { puzzle } => (BoardMode::Solve, puzzle.clone()),
        Route::Set { puzzle } => (BoardMode::Set, puzzle.clone()),
        Route::SetNew => (BoardMode::Set, "".to_owned()),
    };
    html! {
        <>
            <h1>{"Nonogram Game"}</h1>
            <div class={"content-box"}>
                <BoardComponent mode={mode} puzzle={puzzle}/>
            </div>
        </>
    }
}

struct NonogramGame {}

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
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}

fn main() {
    yew::start_app::<NonogramGame>();
}
