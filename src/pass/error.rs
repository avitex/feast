use failure::Fail;

use super::Pass;
use crate::input;

pub trait Error<T: input::Token>: Fail + PartialEq {
    type InputError: input::Error<T>;

    // Create pass error from input error.
    fn from_input<P>(pass: P, err: Self::InputError) -> Self
    where
        P: Pass;
}
