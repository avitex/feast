mod error;
mod slice;

use std::fmt::Debug;

use crate::input::{
    self, CompletionRequirement, Input, InputSection, InputToken, Token, UnexpectedToken,
};

pub use self::error::*;
pub use self::slice::*;

pub trait Context: Sized + Sync + Send + Debug + PartialEq {
    type Input: Input;

    fn input(&self) -> Self::Input;
}

pub trait Pass: Sized + Debug {
    type Context: Context;

    type Error: Error<Self::Context>;

    // Follow a sub pass with a different input.
    // fn sub<I: Input>(&self, input: I) -> Self;

    fn context(&self) -> &Self::Context;

    fn into_context(self) -> Self::Context;

    /// Get the input for this pass.
    fn input(&self) -> PassInput<Self> {
        self.context().input()
    }

    /// Commit the remaining input to be used, consuming the changes.
    fn commit(self, rest: PassInput<Self>) -> Self;

    /// With input result, mapping the error for a pass.
    fn with<O>(input_result: Result<O, PassInputError<Self>>, pass: Self) -> PassResult<Self, O> {
        match input_result {
            Ok(out) => Ok((out, pass)),
            Err(err) => Err(pass.input_error(err)),
        }
    }

    /// Create a pass error based on an input error.
    fn input_error(self, err: PassInputError<Self>) -> Self::Error {
        <PassError<Self> as Error<PassContext<Self>>>::from_input(self.into_context(), err)
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

    /// Returns whether or not the input is empty.
    #[inline]
    fn input_is_empty(&self) -> bool {
        self.input().is_empty()
    }

    /// Splits the first token from the input.
    #[inline]
    fn split_first(self) -> PassResult<Self, (PassToken<Self>, PassInput<Self>)> {
        Self::with(self.input().split_first(), self)
    }

    /// Splits an input in two, based on a predictate.
    /// Will fail if cannot split into a pair.
    #[inline]
    fn split_pair<E, F>(self, pred: F) -> PassResult<Self, (PassSection<Self>, PassInput<Self>)>
    where
        F: FnMut(&PassToken<Self>) -> bool,
    {
        Self::with(self.input().split_pair(pred), self)
    }

    /// Splits an input in two, from an exact size.
    #[inline]
    fn split_at<E>(self, mid: usize) -> PassResult<Self, (PassSection<Self>, PassInput<Self>)> {
        Self::with(self.input().split_at(mid), self)
    }
}

pub type PassContext<P> = <P as Pass>::Context;
pub type PassError<P> = <P as Pass>::Error;
pub type PassInput<P> = <PassContext<P> as Context>::Input;
pub type ContextToken<C> = InputToken<<C as Context>::Input>;
pub type PassToken<P> = InputToken<PassInput<P>>;
pub type PassSection<P> = InputSection<PassInput<P>>;
pub type PassResult<P, O> = Result<(O, P), PassError<P>>;
pub type PassInputError<P> = <<P as Pass>::Error as Error<PassContext<P>>>::InputError;

pub trait PassWithToken<T>: Pass
where
    T: Token,
    PassInput<Self>: Input<Token = T>,
{
}

impl<P, T> PassWithToken<T> for P
where
    T: Token,
    P: Pass,
    PassInput<P>: Input<Token = T>,
{
}
