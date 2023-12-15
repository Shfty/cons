use crate::Offset;

#[derive(Debug, Copy, Clone)]
pub enum PlotCommand {
    BeginImage,
    MoveHead(Offset),
    PlotChar(char),
    EndImage,
}
