mod error;
mod slice;

use std::fmt::Debug;

use crate::input::{
    self, CompletionRequirement, Input, InputSection, InputToken, UnexpectedToken,
};

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
    fn with<O>(
        input_result: Result<O, PassInputError<'i, Self>>,
        pass: Self,
    ) -> PassResult<'i, Self, O> {
        match input_result {
            Ok(out) => Ok((out, pass)),
            Err(err) => Err(pass.input_error(err)),
        }
    }

    /// Create a pass error based on an input error.
    fn input_error(self, err: PassInputError<'i, Self>) -> Self::Error {
        <PassError<'i, Self> as Error<'i>>::from_input(self.into_context(), err)
    }

    /// Create a pass error based on an incomplete input error.
    fn input_error_incomplete(self, requirement: CompletionRequirement) -> Self::Error {
        self.input_error(
            <PassInputError<'i, Self> as input::Error<'i>>::incomplete(requirement),
        )
    }

    /// Create a pass error based on an unexpected input error.
    fn input_error_unexpected(
        self,
        unexpected: UnexpectedToken<'i, PassToken<'i, Self>>,
    ) -> Self::Error {
        self.input_error(
            <PassInputError<'i, Self> as input::Error<'i>>::unexpected(unexpected),
        )
    }

    /// Returns whether or not the input is empty.
    #[inline]
    fn input_is_empty(&'i self) -> bool {
        self.input().is_empty()
    }

    /// Splits the first token from the input.
    #[inline]
    fn split_first(self) -> PassResult<'i, Self, (PassToken<'i, Self>, PassInput<'i, Self>)> {
        <Self as Pass<'i>>::with(self.input().split_first(), self)
    }

    /// Splits an input in two, based on a predictate.
    /// Will fail if cannot split into a pair.
    #[inline]
    fn split_pair<E, F>(
        self,
        pred: F,
    ) -> PassResult<'i, Self, (PassSection<'i, Self>, PassInput<'i, Self>)>
    where
        F: FnMut(&PassToken<'i, Self>) -> bool,
    {
        <Self as Pass<'i>>::with(self.input().split_pair(pred), self)
    }

    /// Splits an input in two, from an exact size.
    #[inline]
    fn split_at<E>(
        self,
        mid: usize,
    ) -> PassResult<'i, Self, (PassSection<'i, Self>, PassInput<'i, Self>)> {
        <Self as Pass<'i>>::with(self.input().split_at(mid), self)
    }
}

pub type PassError<'p, P> = <P as Pass<'p>>::Error;
pub type PassContext<'p, P> = <P as Pass<'p>>::Context;
pub type PassInput<'p, P> = <PassContext<'p, P> as Context<'p>>::Input;
pub type PassToken<'p, P> = InputToken<'p, PassInput<'p, P>>;
pub type PassSection<'p, P> = InputSection<'p, PassInput<'p, P>>;
pub type PassResult<'p, P, O> = Result<(O, P), PassError<'p, P>>;
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
