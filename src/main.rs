use itertools::iproduct;
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
        let width = 500;
        let cell_width = width / n_rows;

        let grid_svg = (n_hints..n_rows)
            .map(|xi| cell_width * xi)
            .into_iter()
            .map(|x| {
                html! {
                    <>
                        <line x1={x.to_string()} y1="0"
                              x2={x.to_string()} y2="500"
                              style="stroke:#DADADA;stroke-width:2" />
                        <line x1="0" y1={x.to_string()}
                              x2="500" y2={x.to_string()}
                              style="stroke:#DADADA;stroke-width:2" />
                    </>
                }
            })
            .collect::<Html>();

        let hints_svg = iproduct!(n_hints..n_rows, 0..n_hints)
            .chain(iproduct!(0..n_hints, n_hints..n_rows))
            .map(|(xi, yi)| {
                (
                    cell_width * xi + cell_width / 2 - 4,
                    cell_width * yi + cell_width / 2 + 6,
                )
            })
            .map(|(x, y)| {
                html! {
                    <text x={x.to_string()} y={y.to_string()} fill="blue">{"0"}</text>
                }
            })
            .collect::<Html>();

        let cell_svg = |xi: i32, yi: i32| {
            let x = cell_width * xi + 1;
            let y = cell_width * yi + 1;
            let rect_width = cell_width - 2;
            html! {
                <rect x={x.to_string()} y={y.to_string()}
                 width={rect_width.to_string()}
                 height={rect_width.to_string()}
                 style="fill:#8D6D6D;stroke-width:1;stroke:#6E4E4E" />
            }
        };
        let cells_svg = iproduct!(n_hints..n_rows, n_hints..n_rows)
            .map(|(xi, yi)| cell_svg(xi, yi))
            .collect::<Html>();

        html! {
            <div>
                <svg width="500" height="500">{grid_svg}{hints_svg}{cells_svg}</svg>
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
