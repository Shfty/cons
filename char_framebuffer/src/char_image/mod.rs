pub mod border;
pub mod builder;
pub mod char_image_cons_tree;
pub mod cons_tree;
pub mod line;
pub mod linear_cells;
pub mod linear_layout;
pub mod padding;
pub mod rect;
pub mod text;

use super::{char_plotter::PlotCommand, Offset, Size};

pub trait CharImage {
    type Iter: Iterator<Item = PlotCommand>;
    fn commands(&self) -> Self::Iter;
    fn size(&self) -> Size;
}
