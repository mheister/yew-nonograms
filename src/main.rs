mod board;

use board::Board;
use itertools::iproduct;
use yew::prelude::*;

use crate::board::FieldCell;

struct NonogramGame {
    state1: i32,
    board: Board,
}

enum ModelMsg {
    PlusOne,
    Fill(i32, i32),
    Mark(i32, i32),
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
            ModelMsg::Fill(row, col) => {
                self.board.fill(row as usize, col as usize);
                true
            }
            ModelMsg::Mark(row, col) => {
                self.board.mark(row as usize, col as usize);
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
                .filter(|(_, _, val)| *val != 0u8)
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

        let filled_cell_svg = |xi: usize, yi: usize| {
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
        let marked_cell_svg = |xi: usize, yi: usize| {
            let x = cell_width_px * xi + cell_width_px / 2 - 4;
            let y = cell_width_px * yi + cell_width_px / 2 + 6;
            let (x, y) = (x.to_string(), y.to_string());
            html! {
                <text {x} {y} fill="black">{"X"}</text>
            }
        };
        let cells_svg: Html = iproduct!(0..n_field_rows, 0..n_field_rows)
            .map(|(xi, yi)| (xi + n_hints, yi + n_hints, self.board.field(yi, xi)))
            .map(|(xi, yi, cell)| match cell {
                FieldCell::Empty => html! {},
                FieldCell::Filled => filled_cell_svg(xi, yi),
                FieldCell::Marked => marked_cell_svg(xi, yi),
            })
            .collect();

        let onclick = ctx.link().batch_callback(move |evt: MouseEvent| {
            let row = (evt.offset_y() / cell_width_px as i32) - n_hints as i32;
            let col = (evt.offset_x() / cell_width_px as i32) - n_hints as i32;
            if evt.button() == 0 {
                Some(Self::Message::Fill(row, col))
            } else {
                None
            }
        });
        let oncontextmenu = ctx.link().callback(move |evt: MouseEvent| {
            evt.prevent_default();
            let row = (evt.offset_y() / cell_width_px as i32) - n_hints as i32;
            let col = (evt.offset_x() / cell_width_px as i32) - n_hints as i32;
            Self::Message::Mark(row, col)
        });

        html! {
            <div>
                <svg width={target_width_px.to_string()} height={target_width_px.to_string()} {onclick} {oncontextmenu}>
                    {grid_svg}{hints_svg}{cells_svg}
                </svg>
                <p></p>
                <button onclick={link.callback(|_| ModelMsg::PlusOne)}>{"+1"}</button>
                <p>{self.state1}</p>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

fn main() {
    yew::start_app::<NonogramGame>();
}
