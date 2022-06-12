use crate::models::board::FieldCell;
use crate::components::preview::NonogramPreview;

use yew::prelude::*;
use itertools::iproduct;

pub struct Board {
    board: crate::models::board::Board,
}

pub enum BoardMsg {
    Fill(i32, i32),
    Mark(i32, i32),
}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            board: crate::models::board::Board::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMsg::Fill(row, col) => {
                self.board.fill(row as usize, col as usize);
                true
            }
            BoardMsg::Mark(row, col) => {
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
        let preview_width_px = cell_width_px * n_hints * 8 / 10;
        let preview_margin_px = cell_width_px * n_hints * 1 / 10;

        let grid_svg = (n_hints..n_rows)
            .map(|xi| cell_width_px * xi)
            .into_iter()
            .map(|x| {
                html! {
                    <>
                        <line x1={x.to_string()} y1="0"
                              x2={x.to_string()} y2={width_px.to_string()}
                              class={"game-grid-line"} />
                        <line x1="0" y1={x.to_string()}
                              x2={width_px.to_string()} y2={x.to_string()}
                              class={"game-grid-line"} />
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
                        <text {x} {y}>{val}</text>
                    }
                })
                .collect::<Html>()
        };

        let filled_cell_svg = |xi: usize, yi: usize, correct: bool| {
            let x = cell_width_px * (xi + n_hints) + 1;
            let y = cell_width_px * (yi + n_hints) + 1;
            let rect_width = cell_width_px - 2;
            let (x, y, width) = (x.to_string(), y.to_string(), rect_width.to_string());
            let height = width.clone();
            let class = if correct {
                "game-cell-filled".to_owned()
            } else {
                "game-cell-filled-incorrect".to_owned()
            };
            html! {
                <rect {x} {y} {width} {height} {class}/>
            }
        };
        let marked_cell_svg = |xi: usize, yi: usize| {
            let x = cell_width_px * (xi + n_hints) + cell_width_px / 2 - 4;
            let y = cell_width_px * (yi + n_hints) + cell_width_px / 2 + 6;
            let (x, y) = (x.to_string(), y.to_string());
            html! {
                <text {x} {y} fill="black">{"X"}</text>
            }
        };
        let cells_svg: Html = iproduct!(0..n_field_rows, 0..n_field_rows)
            .map(|(xi, yi)| match self.board.field(yi, xi) {
                FieldCell::Empty => html! {},
                FieldCell::Filled => {
                    let correct = self.board.solution(yi, xi) == FieldCell::Filled;
                    filled_cell_svg(xi, yi, correct)
                }
                FieldCell::Marked => marked_cell_svg(xi, yi),
            })
            .collect();

        let onclick = link.batch_callback(move |evt: MouseEvent| {
            let row = (evt.offset_y() / cell_width_px as i32) - n_hints as i32;
            let col = (evt.offset_x() / cell_width_px as i32) - n_hints as i32;
            if evt.button() == 0 && row >= 0 && col >= 0 {
                Some(Self::Message::Fill(row, col))
            } else {
                None
            }
        });
        let oncontextmenu = link.batch_callback(move |evt: MouseEvent| {
            evt.prevent_default();
            let row = (evt.offset_y() / cell_width_px as i32) - n_hints as i32;
            let col = (evt.offset_x() / cell_width_px as i32) - n_hints as i32;
            if row >= 0 && col >= 0 {
                Some(Self::Message::Mark(row, col))
            } else {
                None
            }
        });

        html! {
            <>
                <svg id={"game-board"}
                     width={target_width_px.to_string()}
                     height={target_width_px.to_string()}
                     {onclick}
                     {oncontextmenu}>
                    <NonogramPreview field={self.board.field_ref()}
                                     pass={self.board.preview_generation()}
                                     width_px={preview_width_px as u32}
                                     margin_px={preview_margin_px as u32}/>
                    {grid_svg}{hints_svg}{cells_svg}
                </svg>
                <p>{self.board.field_ref().borrow().serialize_base64()}</p>
            </>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}
