use super::{State, StateInputMark, StateInputToken};
use crate::input;

use std::fmt::Debug;

pub trait Error<'i>: Sized + Debug {
    type State: State<'i>;
    type InputError: input::Error<
        Token = StateInputToken<'i, Self::State>,
        Mark = StateInputMark<'i, Self::State>,
    >;

    fn from_input_error(state: &Self::State, err: Self::InputError) -> Self;
}

#[derive(Clone, Debug, PartialEq)]
pub struct VerboseError<'i, S>
where
    S: State<'i>,
{
    input: input::ErrorReason<StateInputToken<'i, S>, StateInputMark<'i, S>>,
}

impl<'i, S> Error<'i> for VerboseError<'i, S>
where
    S: State<'i>,
{
    type State = S;
    type InputError = input::ErrorReason<StateInputToken<'i, S>, StateInputMark<'i, S>>;

    fn from_input_error(_state: &Self::State, err: Self::InputError) -> Self {
        VerboseError { input: err }
    }
}
