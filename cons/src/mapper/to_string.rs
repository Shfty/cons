use super::Mapper;

pub struct ToStringMapper;

impl<T> Mapper<T> for ToStringMapper
where
    T: ToString,
{
    type Mapped = String;

    fn run(&mut self, t: T) -> Self::Mapped {
        t.to_string()
    }
}
