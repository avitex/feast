mod error;
mod slice;

use std::fmt::Debug;

use crate::input::{self, Requirement, Input, InputSection, InputToken, Unexpected};

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
    fn input<'ii>(&'ii self) -> PassInput<'i, Self> {
        self.context().input()
    }

    /// With input result, mapping the error for a pass.
    fn with_error<O>(self, err: PassError<'i, Self>) -> PassResult<'i, Self, O> {
        Err((err, self))
    }

    /// With input result, mapping the error for a pass.
    fn with_input_result<O>(
        self,
        input_result: Result<O, PassInputError<'i, Self>>,
    ) -> PassResult<'i, Self, O> {
        match input_result {
            Ok(out) => Ok((out, self)),
            Err(err) => self.with_input_error(err),
        }
    }

    /// Create a pass error based on an input error.
    fn with_input_error<O>(self, err: PassInputError<'i, Self>) -> PassResult<'i, Self, O> {
        let err = <PassError<'i, Self> as Error<'i>>::from_input(self.context(), err);
        self.with_error(err)
    }

    /// Create a pass error based on an incomplete input error.
    fn with_input_error_incomplete<O>(
        self,
        requirement: Requirement,
    ) -> PassResult<'i, Self, O> {
        self.with_input_error(<PassInputError<'i, Self> as input::Error<'i>>::incomplete(
            requirement,
        ))
    }

    /// Create a pass error based on an unexpected input error.
    fn with_input_error_unexpected<O>(
        self,
        unexpected: Unexpected<'i, PassToken<'i, Self>>,
    ) -> PassResult<'i, Self, O> {
        self.with_input_error(<PassInputError<'i, Self> as input::Error<'i>>::unexpected(
            unexpected,
        ))
    }
}

pub type PassError<'p, P> = <P as Pass<'p>>::Error;
pub type PassContext<'p, P> = <P as Pass<'p>>::Context;
pub type PassInput<'p, P> = <PassContext<'p, P> as Context<'p>>::Input;
pub type PassToken<'p, P> = InputToken<'p, PassInput<'p, P>>;
pub type PassSection<'p, P> = InputSection<'p, PassInput<'p, P>>;
pub type PassResult<'p, P, O> = Result<(O, P), (PassError<'p, P>, P)>;
pub type PassInputError<'i, P> = <<P as Pass<'i>>::Error as Error<'i>>::InputError;

// pub trait PassWithToken<'p, T>: Pass<'p>
// where
//     T: Token,
//     PassInput<'p, Self>: Input<Token = T>,
// {
// }

// impl<'p, P, T> PassWithToken<'p, T> for P
// where
//     T: Token,
//     P: Pass<'p>,
//     PassInput<'p, P>: Input<Token = T>,
// {
// }
