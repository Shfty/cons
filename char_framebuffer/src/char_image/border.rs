use std::marker::PhantomData;

use crate::{char_plotter::PlotCommand, utf8::box_drawing::BoxDrawing, Offset, Size};

use super::{builder::CharImageBuilder, padding::Padding, rect::Rect, CharImage};

#[derive(Debug, Copy, Clone)]
pub struct Border<BD: BoxDrawing, CI: CharImage> {
    child: CI,
    _phantom: PhantomData<BD>,
}

impl<BD: BoxDrawing, CI: CharImage> Border<BD, CI> {
    pub fn new(child: CI) -> Self {
        Border {
            child,
            _phantom: Default::default(),
        }
    }
}

impl<BD: BoxDrawing, CI: CharImage + Clone> CharImage for Border<BD, CI> {
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let mut builder = CharImageBuilder::default();

        let padded = Padding::new(Offset(1, 1), Offset(1, 1), self.child.clone());
        let padded_size = padded.size();

        builder.push_stack();
        {
            builder.subimage(padded);
            builder.subimage(Rect::<BD>::new(padded_size));
        }
        builder.pop_stack();

        builder.finish().commands()
    }

    fn size(&self) -> Size {
        Padding::new(Offset(1, 1), Offset(1, 1), self.child.clone()).size()
    }
}

#[cfg(test)]
mod tests {
    use super::Border;
    use crate::{
        char_image::text::TextLines, char_image::CharImage, char_plotter::CharPlotter,
        utf8::box_drawing::Light, CharFramebuffer,
    };

    #[test]
    fn test_border() {
        println!();
        let image = Border::<Light, _>::new(TextLines::new("One\nTwo\nThree\nFour"));

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }
}
