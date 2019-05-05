use super::{Context, ContextToken};
use crate::input;

use std::fmt::Debug;

pub trait Error<'i>: Debug + 'i {
    type Context: Context<'i>;
    type InputError: input::Error<'i, Token = ContextToken<'i, Self::Context>>;

    // Create pass error from input error.
    fn from_input(ctx: Self::Context, err: Self::InputError) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct VerboseError<'i, C>
where
    C: Context<'i>,
{
    ctx: C,
    input: input::VerboseError<'i, ContextToken<'i, C>>,
}

impl<'i, C> Error<'i> for VerboseError<'i, C>
where
    C: Context<'i>,
{
    type Context = C;
    type InputError = input::VerboseError<'i, ContextToken<'i, C>>;

    fn from_input(ctx: Self::Context, err: Self::InputError) -> Self {
        VerboseError { ctx, input: err }
    }
}
