pub mod utf8;
pub mod char_image;
pub mod char_plotter;

use std::fmt::{Display, Write};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(usize, usize);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size(usize, usize);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset(isize, isize);

#[derive(Debug, Default, Clone)]
pub struct CharFramebuffer {
    buf: Vec<Vec<char>>,
}

impl CharFramebuffer {
    pub fn new(Size(width, height): Size) -> Self {
        CharFramebuffer {
            buf: vec![vec![' '; width]; height],
        }
    }

    pub fn set_char(&mut self, Position(x, y): Position, char: char) {
        if y >= self.buf.len() {
            return;
        }

        let line = &mut self.buf[y];

        if x >= line.len() {
            return;
        }

        line[x] = char;
    }
}

impl Display for CharFramebuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.buf {
            f.write_str(&line.iter().collect::<String>())?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}
