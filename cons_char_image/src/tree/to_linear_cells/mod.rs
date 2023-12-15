use char_framebuffer::{
    char_image::{
        linear_cells::{LinearCellChildren, LinearCells},
        linear_layout::LayoutDirection,
        text::TextLines,
    },
    utf8::box_drawing::BoxDrawing,
};
use cons::{
    mapper::{display_string::DisplayStringMapper, Mapper},
    tree::map::ConsTreeMap,
};

pub struct TextLinesMapper;

impl<T> Mapper<T> for TextLinesMapper
where
    T: ToString,
{
    type Mapped = TextLines<String>;

    fn run(&mut self, t: T) -> Self::Mapped {
        TextLines::new(t.to_string())
    }
}

pub trait ConsTreeIntoLinearCells<BD: BoxDrawing, I> {
    type ToLinearCells;

    fn into_linear_cells(self, direction: LayoutDirection, separation: usize) -> Self::ToLinearCells;
}

impl<BD, I, T> ConsTreeIntoLinearCells<BD, I> for T
where
    BD: BoxDrawing,
    T: ConsTreeMap<I, DisplayStringMapper>,
    T::Map: ConsTreeMap<I, TextLinesMapper>,
    <T::Map as ConsTreeMap<I, TextLinesMapper>>::Map: LinearCellChildren<BD, I>,
{
    type ToLinearCells = LinearCells<BD, I, <T::Map as ConsTreeMap<I, TextLinesMapper>>::Map>;

    fn into_linear_cells(self, direction: LayoutDirection, separation: usize) -> Self::ToLinearCells {
        LinearCells::new(
            direction,
            separation,
            self.map(&mut DisplayStringMapper).map(&mut TextLinesMapper),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ConsTreeIntoLinearCells;
    use char_framebuffer::{
        char_image::linear_layout::LayoutDirection, char_image::CharImage,
        char_plotter::CharPlotter, utf8::box_drawing::Light, CharFramebuffer,
    };
    use cons::list;

    #[test]
    fn test_cons_tree_to_linear_cells() {
        println!();

        let cons_tree = list![1, 2.0, '3', "four"];

        let image = ConsTreeIntoLinearCells::<Light, _>::into_linear_cells(
            cons_tree,
            LayoutDirection::Horizontal,
            1,
        );

        let mut buf = CharFramebuffer::new(image.size());
        CharPlotter::draw_image(&image, |position, char| buf.set_char(position, char));
        println!("{}", buf);
    }
}
