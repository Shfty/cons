use std::marker::PhantomData;

use cons::{list::ConsList, mapper::Mapper, tree::map::ConsTreeMap};

use crate::{char_plotter::PlotCommand, Offset, Size};

use super::{char_image_cons_tree::CharImageConsTree, CharImage};

use crate::char_image::builder::CharImageBuilder;

#[derive(Debug, Copy, Clone)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

pub trait LinearLayoutChildren<I>:
    Clone
    + ConsList
    + CharImageConsTree<I>
    + for<'a> ConsTreeMap<I, LinearLayoutCharImageMapper<'a>>
    + ConsTreeMap<I, LinearLayoutSizeMapper>
{
}

impl<I, T> LinearLayoutChildren<I> for T where
    T: Clone
        + ConsList
        + CharImageConsTree<I>
        + for<'a> ConsTreeMap<I, LinearLayoutCharImageMapper<'a>>
        + ConsTreeMap<I, LinearLayoutSizeMapper>
{
}

/// Mapper for building child images as part of a `Layout`
pub struct LinearLayoutCharImageMapper<'a> {
    direction: LayoutDirection,
    separation: usize,
    pos: usize,
    builder: &'a mut CharImageBuilder,
}

impl<'a, T: CharImage> Mapper<T> for LinearLayoutCharImageMapper<'a> {
    type Mapped = ();

    fn run(&mut self, t: T) -> Self::Mapped {
        self.builder.push_stack();
        {
            self.builder.move_head(match self.direction {
                LayoutDirection::Horizontal => Offset(self.pos as isize, 0),
                LayoutDirection::Vertical => Offset(0, self.pos as isize),
            });

            let child_size = t.size();
            self.pos += match self.direction {
                LayoutDirection::Horizontal => child_size.0,
                LayoutDirection::Vertical => child_size.1,
            };

            self.pos += self.separation;

            self.builder.subimage(t);
        }
        self.builder.pop_stack();
    }
}

/// Mapper for calculating child size as part of a `Layout`
pub struct LinearLayoutSizeMapper {
    direction: LayoutDirection,
    separation: usize,
    size: Size,
}

impl<T: CharImage> Mapper<T> for LinearLayoutSizeMapper {
    type Mapped = ();

    fn run(&mut self, t: T) -> Self::Mapped {
        let Size(w, h) = &mut self.size;
        let Size(cw, ch) = t.size();
        match self.direction {
            LayoutDirection::Horizontal => {
                *w += cw + self.separation;
                *h = (*h).max(ch);
            }
            LayoutDirection::Vertical => {
                *h += ch + self.separation;
                *w = (*w).max(cw);
            }
        }
    }
}

/// `CharImage` implementor that arranges child images along an axis
#[derive(Debug, Copy, Clone)]
pub struct LinearLayout<I, C: CharImageConsTree<I>> {
    direction: LayoutDirection,
    separation: usize,
    children: C,
    _phantom: PhantomData<I>,
}

impl<I, C: CharImageConsTree<I>> LinearLayout<I, C> {
    pub fn new(direction: LayoutDirection, separation: usize, children: C) -> Self {
        LinearLayout {
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

impl<I, C> CharImage for LinearLayout<I, C>
where
    C: LinearLayoutChildren<I>,
{
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let mut builder = CharImageBuilder::default();

        self.children.clone().map(&mut LinearLayoutCharImageMapper {
            pos: 0,
            direction: self.direction,
            separation: self.separation,
            builder: &mut builder,
        });

        builder.finish().commands()
    }

    fn size(&self) -> Size {
        let mut mapper = LinearLayoutSizeMapper {
            direction: self.direction,
            separation: self.separation,
            size: Size(0, 0),
        };

        self.children.clone().map(&mut mapper);

        match self.direction {
            LayoutDirection::Horizontal => mapper.size.0 -= self.separation - 1,
            LayoutDirection::Vertical => mapper.size.1 -= self.separation - 1,
        }

        match self.direction {
            LayoutDirection::Horizontal => mapper.size.0 -= 1,
            LayoutDirection::Vertical => mapper.size.1 -= 1,
        }

        mapper.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char_image::text::TextLine;
    use crate::{char_plotter::CharPlotter, CharFramebuffer};
    use cons::list;

    #[test]
    fn test_linear_layout_horizontal() {
        println!();

        let image = LinearLayout::horizontal(
            1,
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
    fn test_linear_layout_vertical() {
        println!();

        let image = LinearLayout::vertical(
            1,
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
