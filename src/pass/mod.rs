mod slice;
mod error;

use std::fmt::Debug;

use crate::input::{self, CompletionRequirement, Input, InputToken, UnexpectedToken};

pub use self::slice::*;
pub use self::error::*;

pub trait Pass: Sized + Debug + PartialEq {
    type Input: Input;

    type Error: Error<InputToken<Self::Input>>;

    // Follow a sub pass with a different input.
    // fn sub<I: Input>(&self, input: I) -> Self;

    /// Get the input for this pass.
    fn input(&self) -> Self::Input;

    /// Commit the remaining input to be used, consuming the changes.
    fn commit(self, rest: Self::Input) -> Self;

    /// With input result, mapping the error for a pass.
    fn with<O>(self, input_result: Result<O, PassInputError<Self>>) -> PassResult<Self, O> {
        match input_result {
            Ok(out) => Ok((out, self)),
            Err(err) => Err(self.input_error(err)),
        }
    }

    /// Create a pass error based on an input error.
    fn input_error(self, err: PassInputError<Self>) -> Self::Error {
        <Self::Error as Error<PassToken<Self>>>::from_input(self, err)
    }

    /// Create a pass error based on an incomplete input error.
    fn input_error_incomplete(self, requirement: CompletionRequirement) -> Self::Error {
        self.input_error(
            <PassInputError<Self> as input::Error<PassToken<Self>>>::incomplete(requirement),
        )
    }

    /// Create a pass error based on an unexpected input error.
    fn input_error_unexpected(self, unexpected: UnexpectedToken<PassToken<Self>>) -> Self::Error {
        self.input_error(
            <PassInputError<Self> as input::Error<PassToken<Self>>>::unexpected(unexpected),
        )
    }
}

pub type PassError<P> = <P as Pass>::Error;
pub type PassInput<P> = <P as Pass>::Input;
pub type PassToken<P> = InputToken<<P as Pass>::Input>;
pub type PassResult<P, O> = Result<(O, P), PassError<P>>;
pub type PassInputError<P> = <<P as Pass>::Error as Error<PassToken<P>>>::InputError;
