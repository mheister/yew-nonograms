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

        let n_rows = 10 + self.state1;
        let n_hints = 3;
        assert!(n_rows > n_hints);
        let items = (n_hints..n_rows).map(|x| 500 / n_rows * x).collect::<Vec<_>>();

        html! {
            <div>
                <svg width="500" height="500">
                {
                    items.into_iter().map(|i| {
                        html! {
                            <>
                                <line x1={i.to_string()} y1="0"
                                      x2={i.to_string()} y2="500"
                                      style="stroke:#DADADA;stroke-width:2" />
                                <line x1="0" y1={i.to_string()}
                                      x2="500" y2={i.to_string()}
                                      style="stroke:#DADADA;stroke-width:2" />
                            </>
                        }
                    }).collect::<Html>()
                }
                </svg>
                <p></p>
                <button onclick={link.callback(|_| ModelMsg::PlusOne)}>{"+1"}</button>
                <p>{self.state1}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
