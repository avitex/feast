use super::{Pass, PassToken};
use crate::input;

use std::fmt::Debug;

pub trait Error<'p>: Debug + 'p {
    type Pass: Pass<'p>;
    type InputError: input::Error<'p, Token = PassToken<'p, Self::Pass>>;

    // Create pass error from input error.
    fn from_input(pass: Self::Pass, err: Self::InputError) -> Self;
}

impl<'p, P> Error<'p> for VerboseError<'p, P>
where
    P: Pass<'p> + 'p,
{
    type Pass = P;
    type InputError = input::VerboseError<'p, PassToken<'p, P>>;

    fn from_input(pass: P, err: Self::InputError) -> Self {
        VerboseError { pass, input: err }
    }
}

#[derive(Debug, PartialEq)]
pub struct VerboseError<'p, P>
where
    P: Pass<'p>,
{
    pass: P,
    input: input::VerboseError<'p, PassToken<'p, P>>,
}
