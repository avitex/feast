mod error;

use std::fmt::Debug;

use crate::input::{self, Input, Token, UnexpectedToken};

pub use self::error::*;

pub trait Pass: Sized + Debug + PartialEq {
    type Token: Token;
    type Input: Input<Token = Self::Token>;

    type Error: Error + From<Self::InputError>;
    type InputError: input::Error;

    fn input(&self) -> Self::Input;

    fn commit(self, rest: Self::Input) -> Self;

    fn input_error(self, err: Self::InputError) -> Self::Error;
    fn input_error_unexpected(self, unexpected: UnexpectedToken<Self::Token>) -> Self::Error;
}

pub type PassToken<P> = <P as Pass>::Token;
pub type PassInputError<P> = <P as Pass>::InputError;
pub type PassResult<P, O> = Result<(O, P), <P as Pass>::Error>;
