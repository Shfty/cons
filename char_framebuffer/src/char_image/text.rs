use crate::{char_image::builder::CharImageBuilder, char_plotter::PlotCommand, Offset, Size};

use super::CharImage;

#[derive(Debug, Default, Copy, Clone)]
pub struct TextLine<T: ToString>(T);

impl<T: ToString> TextLine<T> {
    pub fn new(str: T) -> Self {
        TextLine(str)
    }
}

impl<T: ToString> CharImage for TextLine<T> {
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let line = self.0.to_string();
        let line = line.replace('\t', "    ");
        let mut builder = CharImageBuilder::default();
        builder.push_stack();
        for char in line.chars() {
            builder.plot_char(char);
            builder.move_head(Offset(1, 0));
        }
        builder.pop_stack();
        builder.finish().commands()
    }

    fn size(&self) -> Size {
        Size(self.0.to_string().len(), 1)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TextLines<T: ToString>(T);

impl<T: ToString> TextLines<T> {
    pub fn new(str: T) -> Self {
        TextLines(str)
    }
}

impl<T: ToString> CharImage for TextLines<T> {
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        let lines = self.0.to_string();
        let lines = lines.split('\n');

        let mut builder = CharImageBuilder::default();
        builder.push_stack();
        for line in lines {
            builder.subimage(TextLine(line));
            builder.move_head(Offset(0, 1));
        }
        builder.pop_stack();
        builder.finish().commands()
    }

    fn size(&self) -> Size {
        let lines = self.0.to_string();
        let width = lines
            .split('\n')
            .map(|line| line.replace('\t', "    "))
            .fold(0, |acc, next| acc.max(next.len()));
        let height = lines.chars().filter(|char| *char == '\n').count() + 1;
        Size(width, height)
    }
}
