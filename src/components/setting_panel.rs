use crate::models::board::Board as BoardModel;
use crate::routes::Route;

use wasm_bindgen::JsCast;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SettingPanelProps {
    pub puzzle: UseStateHandle<AttrValue>,
    pub puzzle_width: UseStateHandle<usize>,
}

#[function_component(SettingPanel)]
pub fn setting_panel(props: &SettingPanelProps) -> Html {
    let puzzle_state = props.puzzle.clone();
    let width_state = props.puzzle_width.clone();
    let width = *width_state;
    let width_onchange = {
        let puzzle = props.puzzle.clone();
        Callback::from(move |e: Event| {
            let target: EventTarget =
                e.target().expect("Error: No target on oninput event");
            let setval = target.unchecked_into::<HtmlInputElement>().value();
            let new_width = setval.parse().expect("Could not parse width");
            width_state.set(new_width);
            log::info!("Increasing puzzle width to {new_width}");
            let mut grid = BoardModel::from_serialized_solution(puzzle.as_ref());
            grid.resize(new_width);
            puzzle_state.set(grid.solution_ref().serialize_base64().into())
        })
    };
    html! {
        <>
            <div class="panel">
                <h3>{"Set a Nonogram"}</h3>
                <label for="puzzle_width_input">{"Width:"}</label>
                <select id={"puzzle_width_input"} onchange={width_onchange}>
                    {
                        (5..=25).map(|w| html!{
                            <option
                                value={w.to_string()}
                                selected={w == width}>
                               {w.to_string()}
                            </option>
                        }).collect::<Html>()
                    }
                </select>
            // <p>{(*props.puzzle).clone()}</p>
            <p>
                <Link<Route> to={Route::Set{puzzle: props.puzzle.to_string()}}>
                    {"Link (Continue Setting)"}
                </Link<Route>>
            </p>
            <p>
                <Link<Route> to={Route::Solve{puzzle: props.puzzle.to_string()}}>
                    {"Link (Solve)"}
                </Link<Route>>
            </p>
            </div>
        </>
    }
}
