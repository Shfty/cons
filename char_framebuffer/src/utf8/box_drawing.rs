pub trait Lines: Copy {
    const HORIZONTAL: char;
    const VERTICAL: char;
}

pub trait Corners: Copy {
    const DOWN_AND_RIGHT: char;
    const DOWN_AND_LEFT: char;
    const UP_AND_RIGHT: char;
    const UP_AND_LEFT: char;
}

pub trait BoxDrawing: Copy + Lines + Corners {
    const VERTICAL_AND_RIGHT: char;
    const VERTICAL_AND_LEFT: char;
    const DOWN_AND_HORIZONTAL: char;
    const UP_AND_HORIZONTAL: char;

    const VERTICAL_AND_HORIZONTAL: char;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Arc;

impl Lines for Arc {
    const HORIZONTAL: char = '─';
    const VERTICAL: char = '│';
}

impl Corners for Arc {
    const DOWN_AND_RIGHT: char = '╭';
    const DOWN_AND_LEFT: char = '╮';
    const UP_AND_LEFT: char = '╯';
    const UP_AND_RIGHT: char = '╰';
}

impl BoxDrawing for Arc {
    const VERTICAL_AND_RIGHT: char = '├';
    const VERTICAL_AND_LEFT: char = '┤';
    const DOWN_AND_HORIZONTAL: char = '┬';
    const UP_AND_HORIZONTAL: char = '┴';

    const VERTICAL_AND_HORIZONTAL: char = '┼';
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Light;

impl Lines for Light {
    const HORIZONTAL: char = '─';
    const VERTICAL: char = '│';
}

impl Corners for Light {
    const DOWN_AND_RIGHT: char = '┌';
    const DOWN_AND_LEFT: char = '┐';
    const UP_AND_RIGHT: char = '└';
    const UP_AND_LEFT: char = '┘';
}

impl BoxDrawing for Light {
    const VERTICAL_AND_RIGHT: char = '├';
    const VERTICAL_AND_LEFT: char = '┤';
    const DOWN_AND_HORIZONTAL: char = '┬';
    const UP_AND_HORIZONTAL: char = '┴';

    const VERTICAL_AND_HORIZONTAL: char = '┼';
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Heavy;

impl Lines for Heavy {
    const HORIZONTAL: char = '━';
    const VERTICAL: char = '┃';
}

impl Corners for Heavy {
    const DOWN_AND_RIGHT: char = '┏';
    const DOWN_AND_LEFT: char = '┓';
    const UP_AND_RIGHT: char = '┗';
    const UP_AND_LEFT: char = '┛';
}

impl BoxDrawing for Heavy {
    const VERTICAL_AND_RIGHT: char = '┣';
    const VERTICAL_AND_LEFT: char = '┫';
    const DOWN_AND_HORIZONTAL: char = '┳';
    const UP_AND_HORIZONTAL: char = '┻';

    const VERTICAL_AND_HORIZONTAL: char = '╋';
}
