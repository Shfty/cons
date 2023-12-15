use std::fmt::Display;

use super::Mapper;

pub struct DisplayStringMapper;

impl<T> Mapper<T> for DisplayStringMapper
where
    T: Display,
{
    type Mapped = String;

    fn run(&mut self, t: T) -> Self::Mapped {
        format!("{}", t)
    }
}
