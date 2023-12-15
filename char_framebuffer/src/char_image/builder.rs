use std::{iter::FromIterator, ops::{Deref, DerefMut}};

use crate::{Offset, char_plotter::{PlotCommand, PlotCommands}};

use super::{CharImage};

#[derive(Debug, Default, Clone)]
pub struct CharImageBuilder(Vec<PlotCommand>);

impl CharImageBuilder {
    pub fn subimage<CI: CharImage>(&mut self, image: CI) -> &mut Self {
        self.extend(image.commands());
        self
    }

    pub fn push_stack(&mut self) -> &mut Self {
        self.push(PlotCommand::BeginImage);
        self
    }

    pub fn pop_stack(&mut self) -> &mut Self {
        self.push(PlotCommand::EndImage);
        self
    }

    pub fn plot_char(&mut self, char: char) -> &mut Self {
        self.push(PlotCommand::PlotChar(char));
        self
    }

    pub fn move_head(&mut self, delta: Offset) -> &mut Self {
        self.push(PlotCommand::MoveHead(delta));
        self
    }

    pub fn finish(self) -> PlotCommands {
        PlotCommands::new(self.0)
    }
}

impl Deref for CharImageBuilder {
    type Target = Vec<PlotCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharImageBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<PlotCommand> for CharImageBuilder {
    fn from_iter<T: IntoIterator<Item = PlotCommand>>(iter: T) -> Self {
        CharImageBuilder(iter.into_iter().collect())
    }
}