mod plot_command;
mod plot_commands;

pub use plot_command::*;
pub use plot_commands::*;

use super::{char_image::CharImage, Offset, Position};

pub struct CharPlotter;

impl CharPlotter {
    pub fn draw_image<CI: CharImage, F: FnMut(Position, char)>(image: &CI, plot: F) {
        Self::run_commands(image.commands(), plot);
    }

    pub fn run_commands<I: Iterator<Item = PlotCommand>, F: FnMut(Position, char)>(
        commands: I,
        mut plot: F,
    ) {
        let mut stack = vec![Position(0, 0)];
        for command in commands {
            match command {
                PlotCommand::BeginImage => stack.push(*stack.last().unwrap()),
                PlotCommand::MoveHead(delta) => {
                    let Offset(dx, dy) = delta;
                    let Position(x, y) = stack.last_mut().unwrap();
                    *x = (*x as isize + dx) as usize;
                    *y = (*y as isize + dy) as usize;
                }
                PlotCommand::PlotChar(char) => plot(*stack.last().unwrap(), char),
                PlotCommand::EndImage => {
                    stack.pop();
                    assert!(
                        !stack.is_empty(),
                        "Can't run an EndImage command before a BeginImage has been issued"
                    );
                }
            }
        }
    }
}
