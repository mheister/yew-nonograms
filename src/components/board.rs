use crate::components::preview::NonogramPreview;
use crate::models::board::{Board as BoardModel, FieldCell};

use itertools::iproduct;
use yew::prelude::*;

pub struct Board {
    board: BoardModel,
}

pub enum BoardMsg {
    RightClick(i32, i32),
    LeftClick(i32, i32),
}

#[derive(Clone, Copy, PartialEq)]
pub enum BoardMode {
    Solve,
    Set,
}

impl yew::html::ImplicitClone for BoardMode {}

#[derive(PartialEq, Properties)]
pub struct BoardProps {
    pub mode: BoardMode,
    pub puzzle: String,
}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = BoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            board: match &ctx.props().puzzle.as_ref() {
                &"" => BoardModel::new(),
                puzzle => BoardModel::from_serialized_solution(puzzle),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (ctx.props().mode, msg) {
            (BoardMode::Solve, BoardMsg::RightClick(row, col)) => {
                self.board.fill(row as usize, col as usize);
                true
            }
            (BoardMode::Solve, BoardMsg::LeftClick(row, col)) => {
                self.board.mark(row as usize, col as usize);
                true
            }
            (BoardMode::Set, BoardMsg::RightClick(row, col)) => {
                if self.board.solution(row as usize, col as usize) != FieldCell::Filled {
                    self.board.set(row as usize, col as usize, true);
                    true
                } else {
                    false
                }
            }
            (BoardMode::Set, BoardMsg::LeftClick(row, col)) => {
                if self.board.solution(row as usize, col as usize) == FieldCell::Filled {
                    self.board.set(row as usize, col as usize, false);
                    true
                } else {
                    false
                }
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
        let preview_width_px = cell_width_px * n_hints * 8 / 10;
        let preview_margin_px = cell_width_px * n_hints / 10;

        let grid_svg = grid_svg(n_hints, n_rows, cell_width_px);
        let hints_svg = hints_svg(&self.board, cell_width_px);
        let cells_svg = cells_svg(&self.board, ctx.props().mode, cell_width_px);

        let onclick = link.batch_callback(move |evt: MouseEvent| {
            let row = (evt.offset_y() / cell_width_px as i32) - n_hints as i32;
            let col = (evt.offset_x() / cell_width_px as i32) - n_hints as i32;
            if evt.button() == 0 && row >= 0 && col >= 0 {
                Some(Self::Message::RightClick(row, col))
            } else {
                None
            }
        });
        let oncontextmenu = link.batch_callback(move |evt: MouseEvent| {
            evt.prevent_default();
            let row = (evt.offset_y() / cell_width_px as i32) - n_hints as i32;
            let col = (evt.offset_x() / cell_width_px as i32) - n_hints as i32;
            if row >= 0 && col >= 0 {
                Some(Self::Message::LeftClick(row, col))
            } else {
                None
            }
        });

        let preview_field = match ctx.props().mode {
            BoardMode::Solve => self.board.field_ref(),
            BoardMode::Set => self.board.solution_ref(),
        };

        html! {
            <>
                <svg id={"game-board"}
                     width={target_width_px.to_string()}
                     height={target_width_px.to_string()}
                     {onclick} {oncontextmenu}>
                    <NonogramPreview field={preview_field.clone()}
                                     width_px={preview_width_px as u32}
                                     margin_px={preview_margin_px as u32}/>
                    {grid_svg}{hints_svg}{cells_svg}
                </svg>
                <p>{self.board.solution_ref().serialize_base64()}</p>
            </>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}

fn cells_svg(board: &BoardModel, mode: BoardMode, cell_width_px: usize) -> Html {
    let n_hints = board.hint_len();
    let n_field_rows = board.width();
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
    match mode {
        BoardMode::Solve => iproduct!(0..n_field_rows, 0..n_field_rows)
            .map(|(xi, yi)| match board.field(yi, xi) {
                FieldCell::Empty => html! {},
                FieldCell::Filled => {
                    let correct = board.solution(yi, xi) == FieldCell::Filled;
                    filled_cell_svg(xi, yi, correct)
                }
                FieldCell::Marked => marked_cell_svg(xi, yi),
            })
            .collect(),
        BoardMode::Set => iproduct!(0..n_field_rows, 0..n_field_rows)
            .map(|(xi, yi)| match board.solution(yi, xi) {
                FieldCell::Empty => html! {},
                FieldCell::Filled => filled_cell_svg(xi, yi, true),
                FieldCell::Marked => marked_cell_svg(xi, yi),
            })
            .collect(),
    }
}

fn hints_svg(board: &BoardModel, cell_width_px: usize) -> Html {
    let n_hints = board.hint_len();
    let n_field_rows = board.width();
    let col_hints = iproduct!(0..n_field_rows, 0..n_hints)
        .map(|(xi, yi)| (n_hints + xi, yi, board.col_hint(xi, yi).number));
    let row_hints = iproduct!(0..n_hints, 0..n_field_rows)
        .map(|(xi, yi)| (xi, n_hints + yi, board.row_hint(yi, xi).number));
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
}

fn grid_svg(
    n_hints: usize,
    n_rows: usize,
    cell_width_px: usize,
) -> yew::virtual_dom::VNode {
    let width_px = cell_width_px * n_rows;
    (n_hints..n_rows)
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
        .collect::<Html>()
}
