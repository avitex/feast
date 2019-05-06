use super::{Context, ContextToken};
use crate::input;

use std::fmt::Debug;

pub trait Error<'i>: Debug + 'i {
    type Context: Context<'i>;
    type InputError: input::Error<'i, Token = ContextToken<'i, Self::Context>>;

    // Create pass error from input error.
    fn from_input(ctx: &Self::Context, err: Self::InputError) -> Self;
}

#[derive(Clone, Debug, PartialEq)]
pub struct VerboseError<'i, C>
where
    C: Context<'i>,
{
    input: input::ErrorReason<'i, ContextToken<'i, C>>,
}

impl<'i, C> Error<'i> for VerboseError<'i, C>
where
    C: Context<'i>,
{
    type Context = C;
    type InputError = input::ErrorReason<'i, ContextToken<'i, C>>;

    fn from_input(_ctx: &Self::Context, err: Self::InputError) -> Self {
        VerboseError { input: err }
    }
}
