mod error;
mod slice;

use std::fmt::Debug;

use crate::input::{self, Input, InputSection, InputToken, Requirement, Unexpected};

pub use self::error::*;
pub use self::slice::*;

pub trait Context<'i>: Sized + Debug + 'i {
    type Input: Input<'i>;

    fn input(&self) -> Self::Input;
}

type ContextInput<'i, C> = <C as Context<'i>>::Input;
type ContextToken<'i, C> = InputToken<'i, ContextInput<'i, C>>;

pub trait Pass<'i>: Sized + Debug + 'i {
    type Context: Context<'i>;

    type Error: Error<'i, Context = Self::Context>;

    fn context(&self) -> &Self::Context;

    fn into_context(self) -> Self::Context;

    /// Commit the remaining input to be used, consuming the changes.
    fn commit(self, rest: PassInput<'i, Self>) -> Self;

    /// Get the input for this pass.
    fn input(&self) -> PassInput<'i, Self> {
        self.context().input()
    }

    /// With input result, mapping the error for a pass.
    fn with_input_result<O>(
        self,
        input_result: Result<O, PassInputError<'i, Self>>,
    ) -> PassResult<'i, Self, O> {
        match input_result {
            Ok(out) => Ok((out, self)),
            Err(err) => Err(self.with_input_error(err)),
        }
    }

    /// Create a pass error based on an input error.
    fn with_input_error(self, err: PassInputError<'i, Self>) -> (PassError<'i, Self>, Self) {
        (
            <PassError<'i, Self> as Error<'i>>::from_input(self.context(), err),
            self,
        )
    }

    /// Create a pass error based on an incomplete input error.
    fn with_input_error_incomplete(self, requirement: Requirement) -> (PassError<'i, Self>, Self) {
        self.with_input_error(<PassInputError<'i, Self> as input::Error<'i>>::incomplete(
            requirement,
        ))
    }

    /// Create a pass error based on an unexpected input error.
    fn with_input_error_unexpected(
        self,
        unexpected: Unexpected<'i, PassToken<'i, Self>>,
    ) -> (PassError<'i, Self>, Self) {
        self.with_input_error(<PassInputError<'i, Self> as input::Error<'i>>::unexpected(
            unexpected,
        ))
    }
}

pub type PassError<'i, P> = <P as Pass<'i>>::Error;
pub type PassContext<'i, P> = <P as Pass<'i>>::Context;
pub type PassInput<'i, P> = <PassContext<'i, P> as Context<'i>>::Input;
pub type PassToken<'i, P> = InputToken<'i, PassInput<'i, P>>;
pub type PassSection<'i, P> = InputSection<'i, PassInput<'i, P>>;
pub type PassResult<'i, P, O> = Result<(O, P), (PassError<'i, P>, P)>;
pub type PassInputError<'i, P> = <<P as Pass<'i>>::Error as Error<'i>>::InputError;

// pub trait PassWithToken<'i, T>: Pass<'i>
// where
//     T: Token,
//     PassInput<'i, Self>: Input<Token = T>,
// {
// }

// impl<'i, P, T> PassWithToken<'i, T> for P
// where
//     T: Token,
//     P: Pass<'i>,
//     PassInput<'i, P>: Input<Token = T>,
// {
// }
