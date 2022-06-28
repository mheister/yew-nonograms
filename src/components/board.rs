mod dragselection;
mod preview;

use crate::models::board::{Board as BoardModel, FieldCell};
use crate::routes::Route;
use dragselection::DragSelection;
use preview::NonogramPreview;

use itertools::iproduct;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeftRight {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Drag {
    start: (i32, i32),
    end: (i32, i32),
    button: LeftRight,
}

pub struct Board {
    board: BoardModel,
    drag: Option<Drag>,
}

pub enum BoardMsg {
    Click(i32, i32, LeftRight),
    Drag(i32, i32, LeftRight),
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
            drag: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (ctx.props().mode, msg) {
            (BoardMode::Solve, BoardMsg::Click(row, col, btn)) => {
                let drag = self.drag.take().unwrap_or(Drag {
                    start: (row, col),
                    end: (row, col),
                    button: btn,
                });
                let first_cell = self
                    .board
                    .field(drag.start.0 as usize, drag.start.1 as usize);
                let mut action = |row, col| match (drag.button, first_cell) {
                    (LeftRight::Left, _) => self.board.fill(row, col),
                    (LeftRight::Right, FieldCell::Marked) => self.board.unmark(row, col),
                    (LeftRight::Right, _) => self.board.mark(row, col),
                };
                DragSelection::new(drag.start, drag.end)
                    .map(|(row, col)| (row as usize, col as usize))
                    .filter(|&(row, col)| action(row, col))
                    .count()
                    > 0
            }
            (BoardMode::Set, BoardMsg::Click(row, col, btn)) => {
                let drag = self.drag.take().unwrap_or(Drag {
                    start: (row, col),
                    end: (row, col),
                    button: btn,
                });
                DragSelection::new(drag.start, drag.end)
                    .map(|(row, col)| (row as usize, col as usize))
                    .filter(|&(row, col)| {
                        self.board.set(row, col, drag.button == LeftRight::Left)
                    })
                    .count()
                    > 0
            }
            (_, BoardMsg::Drag(row, col, btn)) => {
                let start = self.drag.clone().map(|sel| sel.start).unwrap_or((row, col));
                self.drag = Some(Drag {
                    start,
                    end: (row, col),
                    button: btn,
                });
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
        let preview_width_px = cell_width_px * n_hints * 8 / 10;
        let preview_margin_px = cell_width_px * n_hints / 10;

        let grid_svg = grid_svg(n_hints, n_rows, cell_width_px);
        let hints_svg = hints_svg(&self.board, cell_width_px);
        let cells_svg = cells_svg(&self.board, ctx.props().mode, cell_width_px);
        let drag_sel_svg = self.drag.as_ref().map_or(html!(), |drag| {
            selection_svg(&self.board, drag, cell_width_px)
        });

        let offset_to_coord = move |(offset_x, offset_y): (i32, i32)| {
            let row = (offset_y / cell_width_px as i32) - n_hints as i32;
            let col = (offset_x / cell_width_px as i32) - n_hints as i32;
            if row >= 0
                && col >= 0
                && (row as usize) < n_field_rows
                && (col as usize) < n_field_rows
            {
                Some((row, col))
            } else {
                None
            }
        };

        let current_drag_end = self.drag.as_ref().map(|sel| sel.end);
        let onmousemove = link.batch_callback(move |evt: MouseEvent| {
            evt.prevent_default();
            if evt.buttons() == 0 {
                return None;
            }
            offset_to_coord((evt.offset_x(), evt.offset_y())).and_then(|(row, col)| {
                if Some((row, col)) == current_drag_end {
                    return None;
                }
                match evt.buttons() {
                    1 => Some(Self::Message::Drag(row, col, LeftRight::Left)),
                    2 => Some(Self::Message::Drag(row, col, LeftRight::Right)),
                    _ => None,
                }
            })
        });
        let onclick = onmousemove.clone();
        let oncontextmenu = onmousemove.clone();
        let onmouseup = link.batch_callback(move |evt: MouseEvent| {
            offset_to_coord((evt.offset_x(), evt.offset_y())).and_then(|(row, col)| {
                match evt.button() {
                    0 => Some(Self::Message::Click(row, col, LeftRight::Left)),
                    2 => Some(Self::Message::Click(row, col, LeftRight::Right)),
                    _ => None,
                }
            })
        });

        let preview_field = match ctx.props().mode {
            BoardMode::Solve => self.board.field_ref(),
            BoardMode::Set => self.board.solution_ref(),
        };

        let links_to_puzzle = match ctx.props().mode {
            BoardMode::Solve => html!(),
            BoardMode::Set => {
                let serialized_solution = self.board.solution_ref().serialize_base64();
                html! {
                    <>
                    <p>
                        <Link<Route> to={Route::Set{puzzle: serialized_solution.clone()}}>
                            {"Link (Continue Setting)"}
                        </Link<Route>>
                    </p>
                    <p>
                        <Link<Route> to={Route::Solve{puzzle: serialized_solution}}>
                            {"Link (Solve)"}
                        </Link<Route>>
                    </p>
                    </>
                }
            }
        };

        html! {
            <>
                <svg id={"game-board"}
                     width={target_width_px.to_string()}
                     height={target_width_px.to_string()}
                     {onmousemove} {onmouseup} {onclick} {oncontextmenu}>
                    <NonogramPreview field={preview_field.clone()}
                                     width_px={preview_width_px as u32}
                                     margin_px={preview_margin_px as u32}/>
                    {grid_svg}{hints_svg}{cells_svg}{drag_sel_svg}
                </svg>
                {links_to_puzzle}
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

fn selection_svg(board: &BoardModel, drag: &Drag, cell_width_px: usize) -> Html {
    let n_hints = board.hint_len();
    let selected_for_fill_svg = |xi: usize, yi: usize| {
        let x = cell_width_px * (xi + n_hints) + 1;
        let y = cell_width_px * (yi + n_hints) + 1;
        let rect_width = cell_width_px - 2;
        let (x, y, width) = (x.to_string(), y.to_string(), rect_width.to_string());
        let height = width.clone();
        let class = "game-cell-hint";
        html! {
            <rect {x} {y} {width} {height} {class}/>
        }
    };
    let selected_for_mark_svg = |xi: usize, yi: usize| {
        let x = cell_width_px * (xi + n_hints) + cell_width_px / 2 - 4;
        let y = cell_width_px * (yi + n_hints) + cell_width_px / 2 + 6;
        let (x, y) = (x.to_string(), y.to_string());
        html! {
            <text {x} {y} fill="grey">{"X"}</text>
        }
    };
    DragSelection::new(drag.start, drag.end)
        .map(|(row, col)| (col as usize, row as usize))
        .map(|(xi, yi)| match drag.button {
            LeftRight::Left => selected_for_fill_svg(xi, yi),
            LeftRight::Right => selected_for_mark_svg(xi, yi),
        })
        .collect()
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
