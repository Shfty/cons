use char_framebuffer::{
    char_image::{linear_layout::LayoutDirection, CharImage},
    char_plotter::CharPlotter,
    CharFramebuffer,
};

use super::to_char_image::ConsTreeToCharImage;

/// A `ConsTree` type that can draw itself into a `String` for debug visualization
pub trait ConsTreeDisplay<I> {
    fn draw(&self, cell_direction: LayoutDirection, branch_direction: LayoutDirection) -> String;
}

impl<T, I> ConsTreeDisplay<I> for T
where
    T: ConsTreeToCharImage<I>,
{
    fn draw(&self, cell_direction: LayoutDirection, branch_direction: LayoutDirection) -> String {
        let image = self.to_char_image(cell_direction, branch_direction);
        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        format!("{}", buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cons::list;

    #[test]
    fn test_cons_tree_draw() {
        println!();

        let cons_tree = list![list![1, 2.0, '3', "four"], list![5, 6.0, '7', "eight"]];

        println!(
            "Horizontal / Horizontal:\n{}",
            cons_tree.draw(LayoutDirection::Horizontal, LayoutDirection::Horizontal)
        );
        println!(
            "Horizontal / Vertical:\n{}",
            cons_tree.draw(LayoutDirection::Horizontal, LayoutDirection::Vertical)
        );
        println!(
            "Vertical / Horizontal:\n{}",
            cons_tree.draw(LayoutDirection::Vertical, LayoutDirection::Horizontal)
        );
        println!(
            "Vertical / Vertical:\n{}",
            cons_tree.draw(LayoutDirection::Vertical, LayoutDirection::Vertical)
        );
    }
}
