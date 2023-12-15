use std::marker::PhantomData;

use crate::{
    char_image::builder::CharImageBuilder, char_plotter::PlotCommand,
    utf8::box_drawing::BoxDrawing, Offset, Size,
};

use super::{
    line::{Line, Terminated},
    CharImage,
};

pub struct Rect<BD: BoxDrawing> {
    size: Size,
    _phantom: PhantomData<BD>,
}

impl<BD: BoxDrawing> Rect<BD> {
    pub fn new(size: Size) -> Self {
        Rect {
            size,
            _phantom: Default::default(),
        }
    }
}

impl<BD: BoxDrawing> CharImage for Rect<BD> {
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let Size(width, height) = self.size;

        let mut builder = CharImageBuilder::default();

        builder.subimage(Terminated::<BD>::new(
            BD::DOWN_AND_RIGHT,
            BD::DOWN_AND_LEFT,
            Line::horizontal(width),
        ));

        builder.push_stack();
        builder.move_head(Offset(0, 1));
        builder.subimage(Line::<BD>::vertical(height - 2));
        builder.pop_stack();

        builder.push_stack();
        builder.move_head(Offset(0, (height - 1) as isize));
        builder.subimage(Terminated::<BD>::new(
            BD::UP_AND_RIGHT,
            BD::UP_AND_LEFT,
            Line::horizontal(width),
        ));
        builder.pop_stack();

        builder.push_stack();
        builder.move_head(Offset((width - 1) as isize, 1));
        builder.subimage(Line::<BD>::vertical(height - 2));
        builder.pop_stack();

        builder.finish().commands()
    }

    fn size(&self) -> Size {
        self.size
    }
}
