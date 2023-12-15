use crate::{char_plotter::PlotCommand, Offset, Size};

use super::{builder::CharImageBuilder, CharImage};

#[derive(Debug, Copy, Clone)]
pub struct Padding<CI: CharImage> {
    top_left: Offset,
    bottom_right: Offset,
    child: CI,
}

impl<CI: CharImage> Padding<CI> {
    pub fn new(top_left: Offset, bottom_right: Offset, child: CI) -> Self {
        Padding {
            top_left,
            bottom_right,
            child,
        }
    }
}

impl<CI: CharImage + Clone> CharImage for Padding<CI> {
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let mut builder = CharImageBuilder::default();

        builder.push_stack();
        {
            builder.move_head(self.top_left);
            builder.subimage(self.child.clone());
        }
        builder.pop_stack();

        builder.finish().commands()
    }

    fn size(&self) -> Size {
        let Size(w, h) = self.child.size();
        let Offset(sx, sy) = self.top_left;
        let Offset(ex, ey) = self.bottom_right;
        Size(
            (w as isize + sx + ex) as usize,
            (h as isize + sy + ey) as usize,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        char_image::{text::TextLines, CharImage},
        char_plotter::CharPlotter,
        CharFramebuffer, Offset,
    };

    use super::Padding;

    #[test]
    fn test_padding() {
        println!();
        let image = Padding::new(
            Offset(2, 2),
            Offset(2, 2),
            TextLines::new("One\nTwo\nThree\nFour"),
        );

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }
}
