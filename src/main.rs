mod components;
mod models;
mod routes;

use crate::components::board::{Board as BoardComponent, BoardMode};
use crate::components::setting_panel::SettingPanel;
use crate::routes::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
struct MainProps {
    pub mode: BoardMode,
    pub puzzle: String,
}

#[function_component(MainComp)]
fn main_component(props: &MainProps) -> Html {
    let puzzle_width = use_state(|| 10);
    let puzzle = use_state(|| AttrValue::from(props.puzzle.clone()));

    html! {
        <div class={"content-box"}>
            if props.mode == BoardMode::Set {
                <SettingPanel puzzle={puzzle.clone()} puzzle_width={puzzle_width}/>
            }
            <BoardComponent mode={props.mode} puzzle={puzzle}/>
        </div>
    }
}

fn switch(route: Route) -> Html {
    const STARTER_PUZZLE: &str = "CgAKAA==ABAAQAAAAQAEAFRVQRQQVAFBFBBUVQEAAA";
    let (mode, puzzle) = match route {
        Route::Home => {
            return html! {
                <Redirect<Route> to={Route::Set{puzzle: STARTER_PUZZLE.to_owned()}}/>
            }
        }
        Route::Solve { puzzle } => (BoardMode::Solve, puzzle.clone()),
        Route::Set { puzzle } => (BoardMode::Set, puzzle.clone()),
        Route::SetNew => (BoardMode::Set, "".to_owned()),
    };
    html! {
        <>
            <h1>{"Nonogram Game"}</h1>
            <MainComp mode={mode} puzzle={puzzle}/>
        </>
    }
}

#[function_component(NonogramGame)]
fn nonogram_game() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<NonogramGame>::new().render();
}
