use std::marker::PhantomData;

use super::{Context, Error, Pass, PassInput};
use crate::input::{SliceInput, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct SlicePassContext<'i, T>
where
    T: Token,
{
    input: SliceInput<'i, T>,
}

impl<'i, T> Context<'i> for SlicePassContext<'i, T>
where
    T: Token,
{
    type Input = SliceInput<'i, T>;

    fn input(&self) -> Self::Input {
        self.input.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlicePass<'i, T, E>
where
    T: Token,
    E: Error<'i>,
{
    ctx: SlicePassContext<'i, T>,
    _err: PhantomData<E>,
}

impl<'i, T, E> Pass<'i> for SlicePass<'i, T, E>
where
    T: Token,
    E: Error<'i, Context = SlicePassContext<'i, T>>,
{
    type Error = E;
    type Context = SlicePassContext<'i, T>;

    fn context(&self) -> &Self::Context {
        &self.ctx
    }

    fn into_context(self) -> Self::Context {
        self.ctx
    }

    fn commit_input(mut self, rest: PassInput<'i, Self>) -> Self {
        self.ctx.input = rest;
        self
    }
}

impl<'i, T, E> From<&'i [T]> for SlicePass<'i, T, E>
where
    T: Token,
    E: Error<'i, Context = SlicePassContext<'i, T>>,
{
    fn from(slice: &'i [T]) -> Self {
        Self {
            ctx: SlicePassContext {
                input: SliceInput::from(slice),
            },
            _err: PhantomData::<E>,
        }
    }
}
