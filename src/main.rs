mod components;
mod models;
mod routes;

use crate::components::board::{Board as BoardComponent, BoardMode};
use crate::models::board::Board as BoardModel;
use crate::routes::Route;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
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
    let puzzle_width_set_value_state = use_state(|| *puzzle_width_state);
    let unapplied_change = *puzzle_width_state != *puzzle_width_set_value_state;
    let puzzle_width_set_value_oninput = {
        let puzzle_width_set_value_state = puzzle_width_set_value_state.clone();
        Callback::from(move |e: InputEvent| {
            let target: EventTarget =
                e.target().expect("Error: No target on oninput event");
            let setval = target.unchecked_into::<HtmlInputElement>().value();
            if let Ok(value) = setval.parse() {
                puzzle_width_set_value_state.set(value);
            }
        })
    };
    let apply = {
        let puzzle = props.puzzle.clone();
        let puzzle_width_set_value_state = puzzle_width_set_value_state.clone();
        move || {
            let new_width = *puzzle_width_set_value_state;
            puzzle_width_state.set(new_width);
            log::info!("Increasing puzzle width to {new_width}");
            let mut grid = BoardModel::from_serialized_solution(puzzle.as_ref());
            grid.resize(new_width);
            puzzle_state.set(grid.solution_ref().serialize_base64())
        }
    };
    let apply_click = {
        let apply = apply.clone();
        Callback::from(move |_| apply())
    };
    let apply_onchange = Callback::from(move |_| apply());
    html! {
        <>
            <div class="panel">
                <h3>{"Set a Nonogram"}</h3>
                <label>{"Width:"}
                    <input type={"text"} id={"puzzle_width_input"}
                     value={puzzle_width_set_value_state.to_string()} size={4}
                     oninput={puzzle_width_set_value_oninput}
                     onchange={apply_onchange}
                     />
                </label>
                if unapplied_change {<button onclick={apply_click}>{"apply"}</button>}
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
            </div>
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
