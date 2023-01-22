use crate::components::copy_to_clipboard::CopyToClipboard;
use crate::models::board::Board as BoardModel;
use crate::routes::Route;

use wasm_bindgen::JsCast;
use web_sys::EventTarget;
use web_sys::HtmlAnchorElement;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
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
    // to get an 'absolute' URI from a Route
    let to_href = {
        let navigator = use_navigator().expect("Failed to get navigator");
        let anchor = web_sys::window()
            .expect("Could not get window")
            .document()
            .expect("Could not get document")
            .create_element("a")
            .expect("Could not create anchor")
            .unchecked_into::<HtmlAnchorElement>();
        move |route: Route| {
            let url = match navigator.basename() {
                Some(base) => format!("{}{}", base, route.to_path()),
                None => route.to_path(),
            };
            anchor.set_href(&url);
            anchor.href()
        }
    };
    html! {
        <div style={"display:flex"}>
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
            <p>
                <label for={"solvelink_inp"}>{"Link (Solve):"}</label>
                <CopyToClipboard
                    value={to_href({Route::Solve{puzzle: props.puzzle.to_string()}})}
                    input_id={"solvelink_inp"}
                />
            </p>
            <p>
                <label for={"setlink_inp"}>{"Link (Continue Setting):"}</label>
                <CopyToClipboard
                    value={to_href({Route::Set{puzzle: props.puzzle.to_string()}})}
                    input_id={"setlink_inp"}
                />
            </p>
        </div>
        </div>
    }
}
