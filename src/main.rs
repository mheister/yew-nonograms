mod components;
mod models;
mod routes;

use crate::components::board::{Board as BoardComponent, BoardMode};
use crate::models::board::Board as BoardModel;
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
    let puzzle = use_state(|| props.puzzle.clone());

    html! {
        <>
            if props.mode == BoardMode::Set {
                <SettingPanel puzzle={puzzle.clone()} puzzle_width={puzzle_width}/>
            }
            <BoardComponent mode={props.mode} puzzle={puzzle}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct SettingPanelProps {
    pub puzzle: UseStateHandle<String>,
    pub puzzle_width: UseStateHandle<usize>,
}

#[function_component(SettingPanel)]
fn setting_panel(props: &SettingPanelProps) -> Html {
    let puzzle_state = props.puzzle.clone();
    let puzzle_width_state = props.puzzle_width.clone();
    let onclick = {
        let puzzle = props.puzzle.clone();
        Callback::from(move |_| {
            let new_width = *puzzle_width_state + 1;
            puzzle_width_state.set(new_width);
            log::info!("Increasing puzzle width to {new_width}");
            let mut grid = BoardModel::from_serialized_solution(puzzle.as_ref());
            grid.resize(new_width);
            puzzle_state.set(grid.solution_ref().serialize_base64())
        })
    };
    html! {
        <>
            <div>{"Width: "}{props.puzzle_width.to_string()}</div>
            <button {onclick}>{"+"}</button>
            // <p>{(*props.puzzle).clone()}</p>
            <p>
                <Link<Route> to={Route::Set{puzzle: (*props.puzzle).clone()}}>
                    {"Link (Continue Setting)"}
                </Link<Route>>
            </p>
            <p>
                <Link<Route> to={Route::Solve{puzzle: (*props.puzzle).clone()}}>
                    {"Link (Solve)"}
                </Link<Route>>
            </p>
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

#[function_component(NonogramGame)]
fn nonogram_game() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<NonogramGame>();
}
