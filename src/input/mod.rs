mod error;
mod slice;
mod token;

use std::fmt::Debug;

pub use self::error::*;
pub use self::slice::*;
pub use self::token::*;

pub trait Input: Sized + Debug + PartialEq {
    type Token: Token;
    type Section: ExactSizeInput<Token = Self::Token>;

    /// Returns whether or not the input is empty.
    fn is_empty(&self) -> bool;

    /// Splits the first token from the input.
    fn split_first<E>(self) -> Result<(Self::Token, Self), E>
    where
        E: Error<Self::Token>;

    /// Splits an input in two, based on a predictate.
    /// Will fail if cannot split into a pair.
    fn split_pair<E, F>(self, pred: F) -> Result<(Self::Section, Self), E>
    where
        E: Error<Self::Token>,
        F: FnMut(&Self::Token) -> bool;

    /// Splits an input in two, from an exact size.
    fn split_at<E>(self, mid: usize) -> Result<(Self::Section, Self), E>
    where
        E: Error<Self::Token>;
}

pub trait ExactSizeInput: Input {
    /// Returns the length of the input.
    fn len(&self) -> usize;
}

pub type InputToken<I> = <I as Input>::Token;
