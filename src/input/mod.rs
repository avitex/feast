mod error;
mod slice;
mod token;

use std::fmt::Debug;
use std::ops::Index;

pub use self::error::*;
pub use self::slice::*;
pub use self::token::*;

pub trait Input<'i>: Sized + Debug {
    /// The smallest unit of data the input provides.
    type Token: Token + 'i;

    /// A section, or sequence of tokens from the input.
    type Section: ExactSizeInput<'i, Token = Self::Token>;

    /// Returns whether or not the input is empty.
    fn is_empty(&self) -> bool;

    /// Splits the first token from the input.
    fn split_first<E>(self) -> Result<(Self::Token, Self), E>
    where
        E: Error<'i, Token = Self::Token>;

    /// Splits an input in two, based on a predictate.
    /// Will fail if cannot split into a pair.
    fn split_pair<E, F>(self, pred: F) -> Result<(Self::Section, Self), E>
    where
        E: Error<'i, Token = Self::Token>,
        F: FnMut(&Self::Token) -> bool;

    /// Splits an input in two, from an exact size.
    fn split_at<E>(self, mid: usize) -> Result<(Self::Section, Self), E>
    where
        E: Error<'i, Token = Self::Token>;
}

pub trait ExactSizeInput<'i>: Input<'i> + Index<usize, Output = InputToken<'i, Self>> {
    /// Returns the length of the input.
    fn len(&self) -> usize;
}

/// Helper to reference an Input's `Token` type.
pub type InputToken<'i, I> = <I as Input<'i>>::Token;

/// Helper to reference an Input's `Section` type.
pub type InputSection<'i, I> = <I as Input<'i>>::Section;
