mod capture;
mod error;
mod mark;
mod slice;
mod token;

use std::fmt::Debug;
use std::ops::Index;

pub use self::capture::*;
pub use self::error::*;
pub use self::mark::*;
//pub use self::slice::*;
pub use self::token::*;

pub trait InputIterator<T: Token, M: Mark>: Iterator<Item = T> {
    fn mark(&self) -> M;
}

// impl<I, T> InputIterator<T> for I
// where
//     T: Token,
//     I: InputMarker + Iterator<Item = T>,
// {
// }

// Section

pub trait Input<'i>: Sized + Debug {
    type Error: Error;

    type Mark: Mark;
    /// The smallest unit of data the input provides.
    type Token: Token;
    /// A section, or sequence of tokens from the input.
    type Section: ExactSizeInput<'i, Token = Self::Token, Mark = Self::Mark>;

    // type Iterator: InputIterator<Self::Token, Self::Mark>;

    /// Returns whether or not the input is empty.
    fn is_empty(&self) -> bool;

    /// Splits the first token from the input.
    fn split_first<E>(self) -> Result<(Self::Token, Self), E>
    where
        E: Error<Token = Self::Token, Mark = Self::Mark>;

    /// Splits an input in two, from an exact size.
    fn split_at<E>(self, mid: usize) -> Result<(Self::Section, Self), E>
    where
        E: Error<Token = Self::Token, Mark = Self::Mark>;

    fn split_mark<E>(self, mark: Self::Mark) -> Result<(Self::Section, Self), E>
    where
        E: Error<Token = Self::Token, Mark = Self::Mark>;

    // fn iter(&self) -> Self::Iterator;
}

pub trait ExactSizeInput<'i>:
    Input<'i> + Capture + Index<usize, Output = InputToken<'i, Self>>
{
    /// Returns the length of the input.
    fn len(&self) -> usize;
}

/// Helper to reference an Input's `Token` type.
pub type InputToken<'i, I> = <I as Input<'i>>::Token;

/// Helper to reference an Input's `Error` type.
pub type InputError<'i, I> = <I as Input<'i>>::Error;

/// Helper to reference an Input's `Section` type.
pub type InputSection<'i, I> = <I as Input<'i>>::Section;

/// Helper to reference an Input's `Mark` type.
pub type InputMark<'i, I> = <I as Input<'i>>::Mark;
