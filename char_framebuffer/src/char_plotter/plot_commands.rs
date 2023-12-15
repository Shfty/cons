use std::{fmt::Display, iter::FromIterator};

use crate::{
    char_image::{builder::CharImageBuilder, CharImage},
    CharFramebuffer, Position, Size,
};

use super::{CharPlotter, PlotCommand};

#[derive(Debug, Clone)]
pub struct PlotCommands {
    commands: Vec<PlotCommand>,
    size: Size,
}

impl PlotCommands {
    pub fn new(commands: Vec<PlotCommand>) -> Self {
        let size = Self::calculate_size(commands.clone().into_iter());
        PlotCommands { commands, size }
    }
}

impl CharImage for PlotCommands {
    type Iter = std::vec::IntoIter<PlotCommand>;

    fn commands(&self) -> Self::Iter {
        self.commands.clone().into_iter()
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl PlotCommands {
    fn calculate_size<I: Iterator<Item = PlotCommand>>(commands: I) -> Size {
        let mut size = Size(0usize, 0usize);

        CharPlotter::run_commands(commands, |position, _| {
            let Position(x, y) = position;
            let Size(sx, sy) = &mut size;

            *sx = (*sx).max(x + 1);
            *sy = (*sy).max(y + 1);
        });

        size
    }
}

impl Display for PlotCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = CharFramebuffer::new(self.size());
        CharPlotter::draw_image(self, |position, char| buf.set_char(position, char));
        writeln!(f, "{}", buf)?;
        Ok(())
    }
}

impl Into<CharImageBuilder> for PlotCommands {
    fn into(self: PlotCommands) -> CharImageBuilder {
        self.commands.into_iter().collect()
    }
}

impl FromIterator<PlotCommand> for PlotCommands {
    fn from_iter<T: IntoIterator<Item = PlotCommand>>(iter: T) -> Self {
        PlotCommands::new(iter.into_iter().collect())
    }
}
