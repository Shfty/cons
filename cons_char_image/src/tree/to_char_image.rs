use std::fmt::Display;

use char_framebuffer::{char_image::{CharImage, border::Border, cons_tree::{single_cell_display, ConsCellImage}, linear_layout::LayoutDirection, padding::Padding, text::TextLines}, utf8::box_drawing::Light};

use cons::{
    cell::{Cons, ConsCell},
    single::Single,
};

use cons::tree::{Branch, ConsTree, Leaf};

/// A `ConsTree` type that can recursively convert itself into a `CharImage`
pub trait ConsTreeToCharImage<I>: ConsTree<I> {
    type ToCharImage: CharImage + Clone;

    fn to_char_image(
        &self,
        cell_direction: LayoutDirection,
        branch_direction: LayoutDirection,
    ) -> Self::ToCharImage;
}

impl<L, R, CAR, CDR> ConsTreeToCharImage<Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: ConsTreeToCharImage<L>,
    CDR: ConsTreeToCharImage<R>,
{
    type ToCharImage = ConsCellImage<CAR::ToCharImage, CDR::ToCharImage>;

    fn to_char_image(
        &self,
        cell_direction: LayoutDirection,
        branch_direction: LayoutDirection,
    ) -> Self::ToCharImage {
        ConsCellImage::new(
            self.car().to_char_image(cell_direction, branch_direction),
            self.cdr().to_char_image(cell_direction, branch_direction),
            cell_direction,
            branch_direction,
        )
    }
}

impl<CAR> ConsTreeToCharImage<Leaf> for Single<CAR>
where
    CAR: Display,
{
    type ToCharImage = Border<Light, Padding<TextLines<String>>>;

    fn to_char_image(&self, _: LayoutDirection, _: LayoutDirection) -> Self::ToCharImage {
        single_cell_display(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use char_framebuffer::{char_plotter::CharPlotter, CharFramebuffer};
    use cons::list;

    #[test]
    fn test_cons_tree_to_char_image() {
        println!();

        let cons_tree
        = list![
            list![
                1,
                list!["foo", "bar", list![7, 8]],
                2,
                list!["foo", list![7, list![7, 8]], "baz"],
                4,
                list!["five", "six", list![7, 8]]
            ],
            "five",
            list!['3', '4', '5'],
            "four"
        ];

        let image = cons_tree.to_char_image(LayoutDirection::Vertical, LayoutDirection::Vertical);

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }
}
