use yew::prelude::*;

struct Model {
    state1: i32,
}

enum ModelMsg {
    PlusOne,
}

impl Component for Model {
    type Message = ModelMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { state1: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModelMsg::PlusOne => {
                self.state1 += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| ModelMsg::PlusOne)}>{"+1"}</button>
                <p>{self.state1}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
