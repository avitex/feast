use failure::Fail;

use super::{Context, ContextToken};
use crate::input;

pub trait Error<C: Context>: Fail + PartialEq {
    type InputError: input::Error<ContextToken<C>>;

    // Create pass error from input error.
    fn from_input(ctx: C, err: Self::InputError) -> Self;
}

impl<C> Error<C> for VerboseError<C>
where
    C: Context + 'static,
{
    type InputError = input::VerboseError<ContextToken<C>>;

    fn from_input(ctx: C, err: Self::InputError) -> Self {
        VerboseError { ctx, input: err }
    }
}

#[derive(Fail, Debug, PartialEq)]
#[fail(display = "verbose error")]
pub struct VerboseError<C>
where
    C: Context + 'static,
{
    ctx: C,
    input: input::VerboseError<ContextToken<C>>,
}
