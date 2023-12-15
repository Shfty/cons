use std::fmt::{Debug, Display};

use super::{line::{conn_line_h, conn_line_v}, linear_layout::LayoutDirection};
use super::linear_cells::LinearCells;
use super::text::{TextLine, TextLines};
use super::{border::Border};
use super::{padding::Padding};
use super::{CharImage, Offset, Size};
use crate::{char_image::builder::CharImageBuilder, char_plotter::CharPlotter, Position};
use crate::{
    char_plotter::PlotCommand,
    utf8::box_drawing::{Arc, Heavy, Light},
};
use cons::list;

#[derive(Debug, Copy, Clone)]
pub struct ConsCellImage<CAR: CharImage, CDR: CharImage> {
    car: CAR,
    cdr: CDR,
    cell_direction: LayoutDirection,
    branch_direction: LayoutDirection,
}

impl<CAR: CharImage, CDR: CharImage> ConsCellImage<CAR, CDR> {
    pub fn new(
        car: CAR,
        cdr: CDR,
        cell_direction: LayoutDirection,
        branch_direction: LayoutDirection,
    ) -> Self {
        ConsCellImage {
            car,
            cdr,
            cell_direction,
            branch_direction,
        }
    }

    pub fn hh(car: CAR, cdr: CDR) -> Self {
        Self::new(
            car,
            cdr,
            LayoutDirection::Horizontal,
            LayoutDirection::Horizontal,
        )
    }

    pub fn hv(car: CAR, cdr: CDR) -> Self {
        Self::new(
            car,
            cdr,
            LayoutDirection::Horizontal,
            LayoutDirection::Vertical,
        )
    }

    pub fn vh(car: CAR, cdr: CDR) -> Self {
        Self::new(
            car,
            cdr,
            LayoutDirection::Vertical,
            LayoutDirection::Horizontal,
        )
    }

    pub fn vv(car: CAR, cdr: CDR) -> Self {
        Self::new(
            car,
            cdr,
            LayoutDirection::Vertical,
            LayoutDirection::Vertical,
        )
    }
}

impl<CAR: CharImage, CDR: CharImage> CharImage for ConsCellImage<CAR, CDR>
where
    CAR: Clone,
    CDR: Clone,
{
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let mut builder = CharImageBuilder::default();

        let a_cell = TextLine::new(" A ");
        let d_cell = TextLine::new(" D ");

        match (self.cell_direction, self.branch_direction) {
            (LayoutDirection::Horizontal, LayoutDirection::Horizontal) => {
                builder.subimage(LinearCells::<Arc, _, _>::horizontal(
                    1,
                    list![a_cell, d_cell],
                ));
            }
            (LayoutDirection::Horizontal, LayoutDirection::Vertical) => {
                builder.subimage(LinearCells::<Arc, _, _>::horizontal(
                    1,
                    list![d_cell, a_cell],
                ));
            }
            (LayoutDirection::Vertical, LayoutDirection::Horizontal) => {
                builder.subimage(LinearCells::<Arc, _, _>::vertical(1, list![d_cell, a_cell]));
            }
            (LayoutDirection::Vertical, LayoutDirection::Vertical) => {
                builder.subimage(LinearCells::<Arc, _, _>::vertical(1, list![a_cell, d_cell]));
            }
        }

        builder.push_stack();
        builder.move_head(match (self.cell_direction, self.branch_direction) {
            (LayoutDirection::Horizontal, LayoutDirection::Horizontal) => Offset(0, 3),
            (LayoutDirection::Horizontal, LayoutDirection::Vertical) => Offset(4, 3),
            (LayoutDirection::Vertical, LayoutDirection::Horizontal) => Offset(6, 2),
            (LayoutDirection::Vertical, LayoutDirection::Vertical) => Offset(6, 0),
        });
        builder.subimage(self.car.clone());
        builder.pop_stack();

        builder.push_stack();
        builder.move_head(match (self.cell_direction, self.branch_direction) {
            (LayoutDirection::Horizontal, LayoutDirection::Horizontal) => Offset(2, 2),
            (LayoutDirection::Horizontal, LayoutDirection::Vertical) => Offset(6, 2),
            (LayoutDirection::Vertical, LayoutDirection::Horizontal) => Offset(4, 3),
            (LayoutDirection::Vertical, LayoutDirection::Vertical) => Offset(4, 1),
        });

        match (self.cell_direction, self.branch_direction) {
            (LayoutDirection::Horizontal, LayoutDirection::Horizontal) => {
                builder.subimage(conn_line_v::<Heavy>(2))
            }
            (LayoutDirection::Horizontal, LayoutDirection::Vertical) => {
                builder.subimage(conn_line_v::<Heavy>(2))
            }
            (LayoutDirection::Vertical, LayoutDirection::Horizontal) => {
                builder.subimage(conn_line_h::<Heavy>(3))
            }
            (LayoutDirection::Vertical, LayoutDirection::Vertical) => {
                builder.subimage(conn_line_h::<Heavy>(3))
            }
        };

        builder.pop_stack();

        let image = builder.finish();

        let Size(w, h) = image.size();

        let mut builder: CharImageBuilder = image.into();

        builder.push_stack();
        builder.move_head(match (self.cell_direction, self.branch_direction) {
            (LayoutDirection::Horizontal, LayoutDirection::Horizontal) => {
                Offset((w + 1) as isize, 0)
            }
            (LayoutDirection::Horizontal, LayoutDirection::Vertical) => Offset(0, h as isize),
            (LayoutDirection::Vertical, LayoutDirection::Horizontal) => Offset((w + 1) as isize, 0),
            (LayoutDirection::Vertical, LayoutDirection::Vertical) => Offset(0, h as isize),
        });
        builder.subimage(self.cdr.clone());
        builder.pop_stack();

        builder.push_stack();
        builder.move_head(match (self.cell_direction, self.branch_direction) {
            (LayoutDirection::Horizontal, LayoutDirection::Horizontal) => Offset(8, 1),
            (LayoutDirection::Horizontal, LayoutDirection::Vertical) => Offset(2, 2),
            (LayoutDirection::Vertical, LayoutDirection::Horizontal) => Offset(4, 1),
            (LayoutDirection::Vertical, LayoutDirection::Vertical) => Offset(2, 4),
        });

        match (self.cell_direction, self.branch_direction) {
            (LayoutDirection::Horizontal, LayoutDirection::Horizontal) => {
                builder.subimage(conn_line_h::<Heavy>((w - 6) as usize));
            }
            (LayoutDirection::Horizontal, LayoutDirection::Vertical) => {
                builder.subimage(conn_line_v::<Heavy>((h - 1) as usize));
            }
            (LayoutDirection::Vertical, LayoutDirection::Horizontal) => {
                builder.subimage(conn_line_h::<Heavy>(w - 2));
            }
            (LayoutDirection::Vertical, LayoutDirection::Vertical) => {
                builder.subimage(conn_line_v::<Heavy>((h - 3) as usize));
            }
        };

        builder.pop_stack();

        builder.finish().commands()
    }

    fn size(&self) -> Size {
        let mut size = Size(0usize, 0usize);

        CharPlotter::run_commands(self.commands(), |position, _| {
            let Position(x, y) = position;
            let Size(sx, sy) = &mut size;

            *sx = (*sx).max(x + 1);
            *sy = (*sy).max(y + 1);
        });

        size
    }
}

pub fn single_cell_debug<T: Debug>(value: T) -> Border<Light, Padding<TextLines<String>>> {
    Border::new(Padding::new(Offset(1, 0), Offset(1, 0), TextLines::new(format!("{:#?}", value))))
}

pub fn single_cell_display<T: Display>(value: T) -> Border<Light, Padding<TextLines<String>>> {
    Border::new(Padding::new(Offset(1, 0), Offset(1, 0), TextLines::new(format!("{}", value))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char_image::CharImage;
    use crate::CharFramebuffer;

    #[test]
    fn test_cons_cell_hh() {
        println!();

        let image = ConsCellImage::hh(
            ConsCellImage::hh(
                single_cell_display("Hello World!\n\tThis is a test."),
                single_cell_display("As is this.\n\nIn fact,\nthe entire point\nof this particularly\nverbose cell\nis to test\nvertical overlapping\nwith other cells\nin the graph."),
            ),
            ConsCellImage::hh(
                single_cell_debug("Also for Debug \n instead of Display."),
                single_cell_debug(8008135),
            ),
        );

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }

    #[test]
    fn test_cons_cell_hv() {
        println!();

        let image = ConsCellImage::hv(
            ConsCellImage::hv(
                single_cell_display("Hello World!\n\tThis is a test."),
                single_cell_display("As is this.\n\nIn fact,\nthe entire point\nof this particularly\nverbose cell\nis to test\nvertical overlapping\nwith other cells\nin the graph."),
            ),
            ConsCellImage::hv(
                single_cell_debug("Also for Debug \n instead of Display."),
                single_cell_debug(8008135),
            ),
        );

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }

    #[test]
    fn test_cons_cell_vh() {
        println!();

        let image = ConsCellImage::vh(
            ConsCellImage::vh(
                single_cell_display("Hello World!\n\tThis is a test."),
                single_cell_display("As is this.\n\nIn fact,\nthe entire point\nof this particularly\nverbose cell\nis to test\nvertical overlapping\nwith other cells\nin the graph."),
            ),
            ConsCellImage::vh(
                single_cell_debug("Also for Debug \n instead of Display."),
                single_cell_debug(8008135),
            ),
        );

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }

    #[test]
    fn test_cons_cell_vv() {
        println!();

        let image = ConsCellImage::vv(
            ConsCellImage::vv(
                single_cell_display("Hello World!\n\tThis is a test."),
                single_cell_display("As is this.\n\nIn fact,\nthe entire point\nof this particularly\nverbose cell\nis to test\nvertical overlapping\nwith other cells\nin the graph."),
            ),
            ConsCellImage::vv(
                single_cell_debug("Also for Debug \n instead of Display."),
                single_cell_debug(8008135),
            ),
        );

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }
}
