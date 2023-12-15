pub mod debug_string;
pub mod display_string;
pub mod to_string;

/// Trait for mapping generically over values
pub trait Mapper<T> {
    type Mapped;

    fn run(&mut self, t: T) -> Self::Mapped;
}
