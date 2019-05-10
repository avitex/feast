mod error;
mod slice;
mod token;
mod capture;

use std::fmt::Debug;
use std::ops::Index;

pub use self::error::*;
pub use self::slice::*;
pub use self::token::*;
pub use self::capture::*;

pub trait InputMarker: Iterator {
    type Mark;
    type Token: Token;

    fn skip(&mut self, n: usize) -> bool;

    fn peek(&self) -> Option<Self::Token>;
    
    fn child(&self) -> Self;

    fn mark(&self) -> Self::Mark;
}

pub trait Input<'i>: Sized + Debug {
    type Mark;
    /// The smallest unit of data the input provides.
    type Token: Token;
    /// A section, or sequence of tokens from the input.
    type Section: ExactSizeInput<'i, Token = Self::Token>;

    type Marker: InputMarker<Mark = Self::Mark, Token = Self::Token> + Iterator<Item = Self::Token>;

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

    fn marker(&self) -> Self::Marker;
}

pub trait ExactSizeInput<'i>: Input<'i> + Index<usize, Output = InputToken<'i, Self>> {
    /// Returns the length of the input.
    fn len(&self) -> usize;
}

/// Helper to reference an Input's `Token` type.
pub type InputToken<'i, I> = <I as Input<'i>>::Token;

/// Helper to reference an Input's `Section` type.
pub type InputSection<'i, I> = <I as Input<'i>>::Section;
