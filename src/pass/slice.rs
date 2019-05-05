use std::marker::PhantomData;

use super::{Context, Error, Pass, PassInput};
use crate::input::{self, SliceInput, Token};

#[derive(Debug, PartialEq)]
pub struct SlicePass<'a, T, E>
where
    T: Token,
    E: Error<SlicePassContext<'a, T>>,
    E::InputError: input::Error<T>,
{
    ctx: SlicePassContext<'a, T>,
    _err: PhantomData<E>,
}

impl<'a, T, E> Pass for SlicePass<'a, T, E>
where
    T: Token,
    E: Error<SlicePassContext<'a, T>>,
    E::InputError: input::Error<T>,
{
    type Error = E;
    type Context = SlicePassContext<'a, T>;

    fn context(&self) -> &Self::Context {
        &self.ctx
    }

    fn into_context(self) -> Self::Context {
        self.ctx
    }

    fn commit(mut self, rest: PassInput<Self>) -> Self {
        self.ctx.input = rest;
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct SlicePassContext<'a, T>
where
    T: Token,
{
    input: SliceInput<'a, T>,
}

impl<'a, T> Context for SlicePassContext<'a, T>
where
    T: Token,
{
    type Input = SliceInput<'a, T>;

    fn input(&self) -> Self::Input {
        self.input.clone()
    }
}

impl<'a, T, E> From<&'a [T]> for SlicePass<'a, T, E>
where
    T: Token,
    E: Error<SlicePassContext<'a, T>>,
    E::InputError: input::Error<T>,
{
    fn from(slice: &'a [T]) -> Self {
        Self {
            ctx: SlicePassContext {
                input: SliceInput::from(slice),
            },
            _err: PhantomData::<E>,
        }
    }
}
