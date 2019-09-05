mod capture;
mod error;
mod slice;
mod token;

use std::fmt::Debug;
use std::ops::Index;

pub use self::capture::*;
pub use self::error::*;
pub use self::slice::*;
pub use self::token::*;

pub trait InputMarker {
    type Mark;

    fn mark(&self) -> Self::Mark;
}

pub trait InputIterator<T: Token>: InputMarker + Iterator<Item = T> {}

impl<I, T> InputIterator<T> for I
where
    T: Token,
    I: InputMarker + Iterator<Item = T>,
{
}

pub trait Input<'i>: Sized + Debug {
    type Mark;
    /// The smallest unit of data the input provides.
    type Token: Token;
    /// A section, or sequence of tokens from the input.
    type Section: ExactSizeInput<'i, Token = Self::Token>;

    type Iterator: InputIterator<Self::Token> + InputMarker<Mark = Self::Mark>;

    /// Returns whether or not the input is empty.
    fn is_empty(&self) -> bool;

    /// Splits the first token from the input.
    fn split_first<E>(self) -> Result<(Self::Token, Self), E>
    where
        E: Error<'i, Token = Self::Token>;

    /// Splits an input in two, from an exact size.
    fn split_at<E>(self, mid: usize) -> Result<(Self::Section, Self), E>
    where
        E: Error<'i, Token = Self::Token>;

    fn split_mark<E>(self, mark: Self::Mark) -> Result<(Self::Section, Self), E>
    where
        E: Error<'i, Token = Self::Token>;

    fn iter(&self) -> Self::Iterator;
}

pub trait ExactSizeInput<'i>:
    Input<'i> + Capture + Index<usize, Output = InputToken<'i, Self>>
{
    /// Returns the length of the input.
    fn len(&self) -> usize;
}

/// Helper to reference an Input's `Token` type.
pub type InputToken<'i, I> = <I as Input<'i>>::Token;

/// Helper to reference an Input's `Section` type.
pub type InputSection<'i, I> = <I as Input<'i>>::Section;
