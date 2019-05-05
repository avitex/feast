use std::marker::PhantomData;

use super::{Error, Pass};
use crate::input::{SliceInput, Token};

#[derive(Debug, PartialEq)]
pub struct SlicePass<'a, T, E>
where
    T: Token,
    E: Error<T>,
{
    input: SliceInput<'a, T>,
    _err: PhantomData<E>,
}

impl<'a, T, E> Pass for SlicePass<'a, T, E>
where
    T: Token,
    E: Error<T>,
{
    type Error = E;
    type Input = SliceInput<'a, T>;

    fn input(&self) -> Self::Input {
        self.input.clone()
    }

    fn commit(mut self, rest: Self::Input) -> Self {
        self.input = rest;
        self
    }
}
