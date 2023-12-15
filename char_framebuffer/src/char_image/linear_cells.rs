use std::{marker::PhantomData, ops::Rem};

use cons::{list::ConsList, mapper::Mapper, tree::map::ConsTreeMap};
use typenum::Unsigned;

use crate::{char_plotter::PlotCommand, utf8::box_drawing::BoxDrawing, Offset, Size};

use super::{
    border::Border,
    builder::CharImageBuilder,
    line::{Line, LineDirection, Terminated},
    linear_layout::{LayoutDirection, LinearLayout, LinearLayoutChildren},
    CharImage,
};

pub trait LinearCellChildren<BD: BoxDrawing, I>:
    LinearLayoutChildren<I> + for<'a> ConsTreeMap<I, LinearCellsLineMapper<'a, BD>>
{
}

impl<BD: BoxDrawing, I, T> LinearCellChildren<BD, I> for T where
    T: LinearLayoutChildren<I> + for<'a> ConsTreeMap<I, LinearCellsLineMapper<'a, BD>>
{
}

pub struct LinearCells<BD, I, C>
where
    BD: BoxDrawing,
    C: LinearCellChildren<BD, I>,
{
    direction: LayoutDirection,
    separation: usize,
    children: C,
    _phantom: PhantomData<(BD, I)>,
}

impl<BD, I, C> LinearCells<BD, I, C>
where
    BD: BoxDrawing,
    C: LinearCellChildren<BD, I>,
{
    pub fn new(direction: LayoutDirection, separation: usize, children: C) -> Self {
        assert!(separation.rem(2) != 0, "Separation must be an odd number");
        LinearCells {
            direction,
            separation,
            children,
            _phantom: Default::default(),
        }
    }

    pub fn horizontal(separation: usize, children: C) -> Self {
        Self::new(LayoutDirection::Horizontal, separation, children)
    }

    pub fn vertical(separation: usize, children: C) -> Self {
        Self::new(LayoutDirection::Vertical, separation, children)
    }
}

pub struct LinearCellsLineMapper<'a, BD: BoxDrawing> {
    direction: LayoutDirection,
    separation: usize,
    border_size: Size,
    builder: &'a mut CharImageBuilder,
    count: usize,
    _phantom: PhantomData<BD>,
}

impl<'a, C: CharImage, BD: BoxDrawing> Mapper<C> for LinearCellsLineMapper<'a, BD> {
    type Mapped = ();

    fn run(&mut self, t: C) -> Self::Mapped {
        if self.count == 0 {
            return;
        }

        let Size(w, h) = t.size();

        let half_sep = (self.separation - 1) / 2;

        self.builder.move_head(match self.direction {
            LayoutDirection::Horizontal => Offset(w as isize, 0),
            LayoutDirection::Vertical => Offset(0, h as isize),
        });

        self.builder.move_head(match self.direction {
            LayoutDirection::Horizontal => Offset(half_sep as isize, 0),
            LayoutDirection::Vertical => Offset(0, half_sep as isize),
        });

        self.builder.subimage(Terminated::new(
            match self.direction {
                LayoutDirection::Horizontal => BD::DOWN_AND_HORIZONTAL,
                LayoutDirection::Vertical => BD::VERTICAL_AND_RIGHT,
            },
            match self.direction {
                LayoutDirection::Horizontal => BD::UP_AND_HORIZONTAL,
                LayoutDirection::Vertical => BD::VERTICAL_AND_LEFT,
            },
            Line::<BD>::new(
                match self.direction {
                    LayoutDirection::Horizontal => LineDirection::Vertical,
                    LayoutDirection::Vertical => LineDirection::Horizontal,
                },
                match self.direction {
                    LayoutDirection::Horizontal => self.border_size.1,
                    LayoutDirection::Vertical => self.border_size.0,
                },
            ),
        ));

        self.builder.move_head(match self.direction {
            LayoutDirection::Horizontal => Offset(half_sep as isize, 0),
            LayoutDirection::Vertical => Offset(0, half_sep as isize),
        });

        self.builder.move_head(match self.direction {
            LayoutDirection::Horizontal => Offset(1, 0),
            LayoutDirection::Vertical => Offset(0, 1),
        });

        self.count -= 1;
    }
}

impl<BD, I, C> CharImage for LinearCells<BD, I, C>
where
    BD: BoxDrawing,
    I: Clone,
    C: LinearCellChildren<BD, I>,
    Border<BD, LinearLayout<I, C>>: CharImage,
{
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let border = Border::new(LinearLayout::new(
            self.direction,
            self.separation,
            self.children.clone(),
        ));

        let border_size = border.size();

        let mut builder: CharImageBuilder = border.commands().collect();

        builder.push_stack();
        {
            builder.move_head(match self.direction {
                LayoutDirection::Horizontal => Offset(1, 0),
                LayoutDirection::Vertical => Offset(0, 1),
            });

            self.children.clone().map(&mut LinearCellsLineMapper {
                direction: self.direction,
                separation: self.separation,
                builder: &mut builder,
                border_size,
                count: <C as ConsList>::Len::USIZE - 1,
                _phantom: Default::default(),
            });
        }
        builder.pop_stack();

        builder.finish().commands()
    }

    fn size(&self) -> crate::Size {
        Border::new(LinearLayout::new(
            self.direction,
            self.separation,
            self.children.clone(),
        ))
        .size()
    }
}

#[cfg(test)]
mod tests {
    use super::LinearCells;
    use crate::char_image::{text::TextLine, CharImage};
    use crate::utf8::box_drawing::Light;
    use crate::{char_plotter::CharPlotter, CharFramebuffer};
    use cons::list;

    #[test]
    fn test_linear_cells_horizontal() {
        println!();

        let image = LinearCells::<Light, _, _>::horizontal(
            3,
            list![
                TextLine::new("A"),
                TextLine::new("BC"),
                TextLine::new("DEF"),
                TextLine::new("GHIJ"),
                TextLine::new("KLMNO"),
            ],
        );

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }

    #[test]
    fn test_linear_cells_vertical() {
        println!();

        let image = LinearCells::<Light, _, _>::vertical(
            3,
            list![
                TextLine::new("A"),
                TextLine::new("BC"),
                TextLine::new("DEF"),
                TextLine::new("GHIJ"),
                TextLine::new("KLMNO"),
            ],
        );

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }
}
