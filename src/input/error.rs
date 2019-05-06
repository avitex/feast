use super::token::Token;

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
    fn unexpected(unexpected: UnexpectedToken<'a, Self::Token>) -> Self;

    /// Create a new incomplete error with a requirement.
    fn incomplete(requirement: CompletionRequirement) -> Self;
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedToken<'a, T: Token> {
    pub unexpected: T,
    pub expecting: ExpectedHint<'a, T>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpectedHint<'a, T: Token> {
    None,
    Tag(&'a [T]),
    Token(T),
    Description(&'a str),
    OneOf(&'a [ExpectedHint<'a, T>]),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompletionRequirement {
    Exact(usize),
    Between(usize, usize),
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VerboseError<'a, T: Token> {
    Incomplete(CompletionRequirement),
    Unexpected(UnexpectedToken<'a, T>),
}

impl<'a, T> Error<'a> for VerboseError<'a, T>
where
    T: Token,
{
    type Token = T;

    fn is_fatal(&self) -> bool {
        match self {
            VerboseError::Unexpected(_) => true,
            VerboseError::Incomplete(_) => false,
        }
    }

    fn unexpected(unexpected: UnexpectedToken<'a, Self::Token>) -> Self {
        VerboseError::Unexpected::<'a>(unexpected)
    }

    fn incomplete(requirement: CompletionRequirement) -> Self {
        VerboseError::Incomplete(requirement)
    }
}
