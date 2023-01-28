mod dragselection;
mod preview;

use crate::{
    models::board::{Board as BoardModel, FieldCell},
    routes::Route,
};
use dragselection::DragSelection;
use preview::NonogramPreview;

use itertools::iproduct;
use yew::prelude::*;
use yew_router::prelude::*;

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
    mode: BoardMode,
    puzzle_code: String,
    drag: Option<Drag>,
}

pub enum BoardMsg {
    CompleteDragSelection(i32, i32, LeftRight),
    UpdateDragSelection(i32, i32, LeftRight),
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
    pub puzzle: UseStateHandle<AttrValue>,
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
            mode: ctx.props().mode,
            puzzle_code: ctx.props().puzzle.to_string(),
            drag: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let rerender = match msg {
            BoardMsg::UpdateDragSelection(row, col, btn) => {
                self.update_drag_selection(row, col, btn)
            }
            BoardMsg::CompleteDragSelection(row, col, btn) => {
                self.complete_drag_selection(self.mode, row, col, btn)
            }
        };
        if rerender {
            self.puzzle_code = self.board.solution_ref().serialize_base64();
            ctx.props().puzzle.set(self.puzzle_code.clone().into());
            let navigator = ctx.link().navigator().unwrap();
            let route = match self.mode {
                BoardMode::Solve => Route::Solve {
                    puzzle: self.puzzle_code.clone(),
                },
                BoardMode::Set => Route::Set {
                    puzzle: self.puzzle_code.clone(),
                },
            };
            navigator.replace(&route)
        }
        rerender
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
        let cells_svg = cells_svg(&self.board, self.mode, cell_width_px);
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
                    1 => Some(Self::Message::UpdateDragSelection(
                        row,
                        col,
                        LeftRight::Left,
                    )),
                    2 => Some(Self::Message::UpdateDragSelection(
                        row,
                        col,
                        LeftRight::Right,
                    )),
                    _ => None,
                }
            })
        });
        let onclick = onmousemove.clone();
        let oncontextmenu = onmousemove.clone();
        let onmouseup = link.batch_callback(move |evt: MouseEvent| {
            offset_to_coord((evt.offset_x(), evt.offset_y())).and_then(|(row, col)| {
                match evt.button() {
                    0 => Some(Self::Message::CompleteDragSelection(
                        row,
                        col,
                        LeftRight::Left,
                    )),
                    2 => Some(Self::Message::CompleteDragSelection(
                        row,
                        col,
                        LeftRight::Right,
                    )),
                    _ => None,
                }
            })
        });

        let preview_field = match self.mode {
            BoardMode::Solve => self.board.field_ref(),
            BoardMode::Set => self.board.solution_ref(),
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
            </>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _orig_props: &Self::Properties) -> bool {
        let mut rerender = false;
        let puzzle_from_prop = ctx.props().puzzle.as_ref();
        if *puzzle_from_prop != self.puzzle_code {
            self.board = BoardModel::from_serialized_solution(&puzzle_from_prop);
            log::info!("Updating puzzle from code");
            rerender = true;
        }
        if ctx.props().mode != self.mode {
            self.mode = ctx.props().mode;
            rerender = true;
        }
        rerender
    }
}

impl Board {
    // returns true if the selection changed
    fn update_drag_selection(&mut self, row: i32, col: i32, btn: LeftRight) -> bool {
        let start = self.drag.clone().map(|sel| sel.start).unwrap_or((row, col));
        self.drag = Some(Drag {
            start,
            end: (row, col),
            button: btn,
        });
        true
    }

    // returns true if any action was performed
    fn complete_drag_selection(
        &mut self,
        mode: BoardMode,
        row: i32,
        col: i32,
        btn: LeftRight,
    ) -> bool {
        let drag = self.drag.take().unwrap_or(Drag {
            start: (row, col),
            end: (row, col),
            button: btn,
        });
        match mode {
            BoardMode::Solve => {
                let start_cell_state = self
                    .board
                    .field(drag.start.0 as usize, drag.start.1 as usize);
                let mut action = |row, col| match (drag.button, start_cell_state) {
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
            BoardMode::Set => {
                DragSelection::new(drag.start, drag.end)
                    .map(|(row, col)| (row as usize, col as usize))
                    .filter(|&(row, col)| {
                        self.board.set(row, col, drag.button == LeftRight::Left)
                    })
                    .count()
                    > 0
            }
        }
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
    let font_size = |val: &str| {
        let px = if val.len() == 1 {
            cell_width_px -1
        } else {
            cell_width_px * 6 / 10 - 1
        };
        format!("{px}px")
    };
    col_hints
        .chain(row_hints)
        .filter(|(_, _, val)| *val != 0u8)
        .map(|(xi, yi, val)| {
            (
                cell_width_px * xi + cell_width_px / 2 - 6,
                cell_width_px * yi + cell_width_px - 2,
                val,
            )
        })
        .map(|(x, y, val)| (x.to_string(), y.to_string(), val.to_string()))
        .map(|(x, y, val)| {
            html! {
                <text {x} {y} font-size={font_size(&val)}>{val}</text>
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
