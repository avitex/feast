use std::fmt::Debug;

/// A `Mark` represents a static position in an input,
/// and should be treated as an opaque type.
pub trait Mark: Debug + Copy + PartialEq {}

impl Mark for usize {}

#[derive(Debug, PartialEq, Clone)]
pub struct Span<M: Mark> {
    pub from: M,
    pub to: M,
}

impl<M> Span<M>
where
    M: Mark,
{
    pub fn new(from: M, to: M) -> Self {
        Self { from, to }
    }
}
