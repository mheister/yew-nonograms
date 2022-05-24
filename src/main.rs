mod board;

use board::Board;
use itertools::iproduct;
use yew::prelude::*;

struct NonogramGame {
    state1: i32,
    board: Board,
}

enum ModelMsg {
    PlusOne,
}

impl Component for NonogramGame {
    type Message = ModelMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state1: 0,
            board: Board::new(),
        }
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

        let n_hints = self.board.hint_len();
        let n_field_rows = self.board.width();
        let n_rows = n_hints + n_field_rows;
        let target_width_px = 500;
        let cell_width_px = target_width_px / n_rows;
        let width_px = cell_width_px * n_rows;

        let grid_svg = (n_hints..n_rows)
            .map(|xi| cell_width_px * xi)
            .into_iter()
            .map(|x| {
                html! {
                    <>
                        <line x1={x.to_string()} y1="0"
                              x2={x.to_string()} y2={width_px.to_string()}
                              style="stroke:#DADADA;stroke-width:2" />
                        <line x1="0" y1={x.to_string()}
                              x2={width_px.to_string()} y2={x.to_string()}
                              style="stroke:#DADADA;stroke-width:2" />
                    </>
                }
            })
            .collect::<Html>();

        let hints_svg = {
            let col_hints = iproduct!(0..n_field_rows, 0..n_hints)
                .map(|(xi, yi)| (n_hints + xi, yi, self.board.col_hint(xi, yi).number));
            let row_hints = iproduct!(0..n_hints, 0..n_field_rows)
                .map(|(xi, yi)| (xi, n_hints + yi, self.board.row_hint(yi, xi).number));
            col_hints
                .chain(row_hints)
                .filter(|(_,_,val)| *val != 0u8)
                .map(|(xi, yi, val)| {
                    (
                        cell_width_px * xi + cell_width_px / 2 - 4,
                        cell_width_px * yi + cell_width_px / 2 + 6,
                        val,
                    )
                })
                .map(|(x, y, val)| (x.to_string(), y.to_string(), val.to_string()))
                .map(|(x, y, val)| {
                    html! {
                        <text {x} {y} fill="blue">{val}</text>
                    }
                })
                .collect::<Html>()
        };

        let cell_svg = |xi: usize, yi: usize| {
            let x = cell_width_px * xi + 1;
            let y = cell_width_px * yi + 1;
            let rect_width = cell_width_px - 2;
            html! {
                <rect x={x.to_string()} y={y.to_string()}
                 width={rect_width.to_string()}
                 height={rect_width.to_string()}
                 style="fill:#8D6D6D;stroke-width:1;stroke:#6E4E4E" />
            }
        };
        let cells_svg = iproduct!(0..n_field_rows, 0..n_field_rows)
            .map(|(xi, yi)| (xi + n_hints, yi + n_hints, self.board.solution()[yi][xi]))
            .filter(|(_, _, cell)| cell.eq(&board::FieldCell::Filled))
            .map(|(xi, yi, _)| cell_svg(xi, yi))
            .collect::<Html>();

        html! {
            <div>
                <svg width={target_width_px.to_string()} height={target_width_px.to_string()}>
                    {grid_svg}{hints_svg}{cells_svg}
                </svg>
                <p></p>
                <button onclick={link.callback(|_| ModelMsg::PlusOne)}>{"+1"}</button>
                <p>{self.state1}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<NonogramGame>();
}
