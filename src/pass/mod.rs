mod error;

use std::fmt::Debug;

use crate::input::{self, Input, Token, UnexpectedToken, CompletionRequirement};

pub use self::error::*;

pub trait Pass: Sized + Debug + PartialEq {
    type Token: Token;
    type Input: Input<Token = Self::Token>;

    type Error: Error + From<Self::InputError>;
    type InputError: input::Error<Token = Self::Token>;

    /// Follow a sub pass with a different input.
    fn sub<P: Pass, I: Input>(&self, input: &I) -> P;

    /// Get the input for this pass.
    fn input(&self) -> Self::Input;

    /// Commit the remaining input to be used, consuming the changes.
    fn commit(self, rest: Self::Input) -> Self;

    /// Create a pass error based on an input error.
    fn input_error(self, err: Self::InputError) -> Self::Error;

    /// Create a pass error based on an incomplete input error.
    fn input_error_incomplete(self, requirement: CompletionRequirement) -> Self::Error {
        self.input_error(<Self::InputError as input::Error>::incomplete(requirement))
    }

    /// Create a pass error based on an unexpected input error.
    fn input_error_unexpected(self, unexpected: UnexpectedToken<Self::Token>) -> Self::Error {
        self.input_error(<Self::InputError as input::Error>::unexpected(unexpected))
    }
}

pub type PassToken<P> = <P as Pass>::Token;
pub type PassInputError<P> = <P as Pass>::InputError;
pub type PassResult<P, O> = Result<(O, P), <P as Pass>::Error>;
