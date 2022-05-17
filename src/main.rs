use yew::prelude::*;

struct Model {
    state1: i32,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { state1: 0 });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Model {
                state1: state.state1 + 1,
            })
        })
    };

    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{state.state1}</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
