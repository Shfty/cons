use std::fmt::Debug;

use super::Mapper;

pub struct DebugStringMapper;

impl<T> Mapper<T> for DebugStringMapper
where
    T: Debug,
{
    type Mapped = String;

    fn run(&mut self, t: T) -> Self::Mapped {
        format!("{:?}", t)
    }
}
