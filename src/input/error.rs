use super::token::{Token, TokenTag};

use std::fmt::Debug;

pub trait Error<'a>: Debug + PartialEq {
    type Token: Token;

    /// If the error is fatal, we won't be able to try again.
    fn is_fatal(&self) -> bool;

    /// If is completable, perhaps we should try again?.
    fn is_completable(&self) -> bool {
        !self.is_fatal()
    }

    /// Create a new unexpected error with details.
    fn unexpected(unexpected: Unexpected<'a, Self::Token>) -> Self;

    /// Create a new incomplete error with a requirement.
    fn incomplete(requirement: Requirement) -> Self;
}

impl<'a, T> Error<'a> for ErrorReason<'a, T>
where
    T: Token,
{
    type Token = T;

    fn is_fatal(&self) -> bool {
        self.is_fatal()
    }

    fn unexpected(unexpected: Unexpected<'a, Self::Token>) -> Self {
        ErrorReason::Unexpected(unexpected)
    }

    /// Create a new incomplete error with a requirement.
    fn incomplete(requirement: Requirement) -> Self {
        ErrorReason::Incomplete(requirement)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorReason<'a, T: Token> {
    Incomplete(Requirement),
    Unexpected(Unexpected<'a, T>),
}

impl<'a, T: Token> ErrorReason<'a, T> {
    pub fn is_fatal(&self) -> bool {
        match self {
            ErrorReason::Incomplete(_) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Unexpected<'a, T: Token> {
    pub unexpected: TokenTag<'a, T>,
    pub expecting: ExpectedHint<'a, T>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpectedHint<'a, T: Token> {
    None,
    Token(T),
    Tag(&'a [T]),
    Description(&'a str),
    OneOf(&'a [ExpectedHint<'a, T>]),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Requirement {
    Exact(usize),
    Between(usize, usize),
    Unknown,
}
