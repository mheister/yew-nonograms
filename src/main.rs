mod routes;
mod components;
mod models;

use crate::routes::Route;
use crate::components::board::{Board as BoardComponent, BoardMode};
use crate::models::board::Board as BoardModel;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
struct MainProps {
    pub mode: BoardMode,
    pub puzzle: String
}

#[function_component(MainComp)]
fn main_component(props: &MainProps) -> Html {
    let puzzle_width = use_state(|| 10);
    let puzzle_width_clone = puzzle_width.clone();
    let puzzle = use_state(|| props.puzzle.clone());
    let onclick = {
        let puzzle = puzzle.clone();
        Callback::from(move |_| {
            let new_width = *puzzle_width + 1;
            puzzle_width.set(new_width);
            log::info!("Increasing puzzle width to {new_width}");
            let mut grid = BoardModel::from_serialized_solution(puzzle.as_ref());
            grid.resize(*puzzle_width);
            puzzle.set(grid.solution_ref().serialize_base64())})
    };

    html! {
        <>
            <div>{"Width: "}{puzzle_width_clone.to_string()}</div>
            <button {onclick}>{"+"}</button>
            // <p>{(*puzzle).clone()}</p>
            <BoardComponent mode={props.mode} puzzle={puzzle.clone()}/>
        </>
    }
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
                <MainComp mode={mode} puzzle={puzzle}/>
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
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<NonogramGame>();
}
