use super::mark::{Mark, Span};
use super::token::Token;

use std::fmt::Debug;

pub trait Error: Debug + PartialEq {
    type Mark: Mark;
    type Token: Token;

    /// If the error is fatal, we won't be able to try again.
    fn is_fatal(&self) -> bool;

    /// Create a new unexpected error with details.
    fn unexpected(unexpected: Unexpected<Self::Token, Self::Mark>) -> Self;

    /// Create a new incomplete error with a requirement.
    fn incomplete(requirement: Requirement) -> Self;
}

impl<T, M> Error for ErrorReason<T, M>
where
    M: Mark,
    T: Token,
{
    type Mark = M;
    type Token = T;

    fn is_fatal(&self) -> bool {
        self.is_fatal()
    }

    fn unexpected(unexpected: Unexpected<T, M>) -> Self {
        ErrorReason::Unexpected(unexpected)
    }

    /// Create a new incomplete error with a requirement.
    fn incomplete(requirement: Requirement) -> Self {
        ErrorReason::Incomplete(requirement)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorReason<T: Token, M: Mark> {
    Incomplete(Requirement),
    Unexpected(Unexpected<T, M>),
}

impl<T: Token, M: Mark> ErrorReason<T, M> {
    pub fn is_fatal(&self) -> bool {
        match self {
            ErrorReason::Incomplete(_) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Unexpected<T: Token, M: Mark> {
    pub unexpected: Span<M>,
    pub expecting: ExpectedHint<T>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpectedHint<T: Token> {
    None,
    Token(T),
    Tag(&'static [T]),
    Description(&'static str),
    OneOf(&'static [ExpectedHint<T>]),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Requirement {
    Exact(usize),
    Between(usize, usize),
    Unknown,
}
