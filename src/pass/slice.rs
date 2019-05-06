use std::marker::PhantomData;

use super::{Context, Error, Pass, PassInput};
use crate::input::{SliceInput, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct SlicePassContext<'p, T>
where
    T: Token,
{
    input: SliceInput<'p, T>,
}

impl<'p, T> Context<'p> for SlicePassContext<'p, T>
where
    T: Token,
{
    type Input = SliceInput<'p, T>;

    fn input(&self) -> Self::Input {
        self.input.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlicePass<'p, T, E>
where
    T: Token,
    E: Error<'p>,
{
    ctx: SlicePassContext<'p, T>,
    _err: PhantomData<E>,
}

impl<'p, T, E> Pass<'p> for SlicePass<'p, T, E>
where
    T: Token,
    E: Error<'p, Context = SlicePassContext<'p, T>>,
{
    type Error = E;
    type Context = SlicePassContext<'p, T>;

    fn context(&self) -> &Self::Context {
        &self.ctx
    }

    fn into_context(self) -> Self::Context {
        self.ctx
    }

    fn commit(mut self, rest: PassInput<'p, Self>) -> Self {
        self.ctx.input = rest;
        self
    }
}

impl<'p, T, E> From<&'p [T]> for SlicePass<'p, T, E>
where
    T: Token,
    E: Error<'p, Context = SlicePassContext<'p, T>>,
{
    fn from(slice: &'p [T]) -> Self {
        Self {
            ctx: SlicePassContext {
                input: SliceInput::from(slice),
            },
            _err: PhantomData::<E>,
        }
    }
}
