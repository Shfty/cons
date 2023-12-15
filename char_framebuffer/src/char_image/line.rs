use std::marker::PhantomData;

use crate::{
    char_image::builder::CharImageBuilder,
    char_plotter::PlotCommand,
    utf8::box_drawing::{BoxDrawing, Lines},
    Offset, Size,
};

use super::CharImage;

#[derive(Debug, Copy, Clone)]
pub enum LineDirection {
    Horizontal,
    Vertical,
}

impl Into<Offset> for LineDirection {
    fn into(self) -> Offset {
        match self {
            LineDirection::Horizontal => Offset(1, 0),
            LineDirection::Vertical => Offset(0, 1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Line<L: Lines> {
    direction: LineDirection,
    length: usize,
    _phantom: PhantomData<L>,
}

impl<L: Lines> Line<L> {
    pub fn new(direction: LineDirection, length: usize) -> Self {
        Line {
            direction,
            length,
            _phantom: Default::default(),
        }
    }

    pub fn horizontal(length: usize) -> Self {
        Line {
            direction: LineDirection::Horizontal,
            length,
            _phantom: Default::default(),
        }
    }

    pub fn vertical(length: usize) -> Self {
        Line {
            direction: LineDirection::Vertical,
            length,
            _phantom: Default::default(),
        }
    }
}

impl<L: Lines> CharImage for Line<L> {
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let mut builder = CharImageBuilder::default();

        builder.push_stack();

        for i in 0..self.length {
            builder.plot_char(match self.direction {
                LineDirection::Horizontal => L::HORIZONTAL,
                LineDirection::Vertical => L::VERTICAL,
            });
            if i < self.length - 1 {
                builder.move_head(self.direction.into());
            }
        }

        builder.pop_stack();

        builder.finish().commands()
    }

    fn size(&self) -> Size {
        match self.direction {
            LineDirection::Horizontal => Size(self.length, 1),
            LineDirection::Vertical => Size(1, self.length),
        }
    }
}

pub struct Terminated<L: Lines> {
    start: char,
    end: char,
    line: Line<L>,
}

impl<L: Lines> Terminated<L> {
    pub fn new(start: char, end: char, line: Line<L>) -> Self {
        Terminated { start, end, line }
    }
}

impl<L: Lines> CharImage for Terminated<L> {
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let mut builder: CharImageBuilder = self.line.commands().collect();
        builder[1] = PlotCommand::PlotChar(self.start);
        let last_idx = builder.len() - 2;
        builder[last_idx] = PlotCommand::PlotChar(self.end);
        builder.finish().commands()
    }

    fn size(&self) -> Size {
        self.line.size()
    }
}

pub fn conn_line_h<BD: BoxDrawing>(width: usize) -> Terminated<BD> {
    Terminated::new(
        BD::VERTICAL_AND_RIGHT,
        BD::VERTICAL_AND_LEFT,
        Line::horizontal(width),
    )
}

pub fn conn_line_v<BD: BoxDrawing>(height: usize) -> Terminated<BD> {
    Terminated::new(
        BD::DOWN_AND_HORIZONTAL,
        BD::UP_AND_HORIZONTAL,
        Line::vertical(height),
    )
}
