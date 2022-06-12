use std::{cell::RefCell, rc::Rc};

use itertools::iproduct;
use yew::prelude::*;

use crate::models::grid::Grid;
use crate::models::board::FieldCell;

pub struct NonogramPreview;

#[derive(Properties, Clone, PartialEq)]
pub struct NonogramPreviewProps {
    pub field: Rc<RefCell<Grid<FieldCell>>>,
    pub pass: u32,
    pub width_px: u32,
    pub margin_px: u32,
}

impl Component for NonogramPreview {
    type Message = ();

    type Properties = NonogramPreviewProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let grid = ctx.props().field.borrow();
        let n_field_rows = grid.width();
        let cell_width_px = ctx.props().width_px as usize / n_field_rows;
        let margin = ctx.props().margin_px as usize;

        let cell_svg = |xi: usize, yi: usize| {
            let x = cell_width_px * xi + margin;
            let y = cell_width_px * yi + margin;
            let (x, y, width) = (x.to_string(), y.to_string(), cell_width_px.to_string());
            let height = width.clone();
            let class = "game-cell-preview".to_owned();
            html! {
                <rect {x} {y} {width} {height} {class}/>
            }
        };
        iproduct!(0..n_field_rows, 0..n_field_rows)
            .map(|(xi, yi)| match grid[yi][xi] {
                FieldCell::Filled => cell_svg(xi, yi),
                _ => html! {},
            })
            .collect()
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}
