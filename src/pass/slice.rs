use std::marker::PhantomData;

use super::{Error, Pass, PassInput};
use crate::input::{self, SliceInput, Token};

#[derive(Debug, PartialEq)]
pub struct SlicePass<'p, T, E>
where
    T: Token,
    E: Error<'p, Self>,
    E::InputError: input::Error<'p, Token = T>,
{
    input: SliceInput<'p, T>,
    _err: PhantomData<E>,
}

impl<'p, T, E> Pass<'p> for SlicePass<'p, T, E>
where
    T: Token,
    E: Error<'p, Self>,
    E::InputError: input::Error<'p, Token = T>,
{
    type Error = E;
    type Input = SliceInput<'p, T>;

    fn input(&self) -> Self::Input {
        self.input.clone()
    }

    fn commit(mut self, rest: PassInput<'p, Self>) -> Self {
        self.input = rest;
        self
    }
}

impl<'p, T, E> From<&'p [T]> for SlicePass<'p, T, E>
where
    T: Token,
    E: Error<'p, Self>,
    E::InputError: input::Error<'p, Token = T>,
{
    fn from(slice: &'p [T]) -> Self {
        Self {
            input: SliceInput::from(slice),
            _err: PhantomData::<E>,
        }
    }
}
