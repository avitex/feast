use super::token::Token;
use failure::Fail;

pub trait Error<T: Token>: Fail + PartialEq {
    //type Token: Token;

    /// If the error is fatal, we won't be able to try again.
    fn is_fatal(&self) -> bool;

    /// If is completable, perhaps we should try again?.
    fn is_completable(&self) -> bool {
        !self.is_fatal()
    }

    /// Create a new unexpected error with details.
    fn unexpected(unexpected: UnexpectedToken<T>) -> Self;

    /// Create a new incomplete error with a requirement.
    fn incomplete(requirement: CompletionRequirement) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct UnexpectedToken<T: Token> {
    pub unexpected: T,
    pub expecting: ExpectedHint<T>,
}

#[derive(Debug, PartialEq)]
pub enum ExpectedHint<T: Token> {
    None,
    Token(T),
    Description(&'static str),
    OneOf(&'static [ExpectedHint<T>]),
}

#[derive(Debug, PartialEq)]
pub enum CompletionRequirement {
    Exact(usize),
    Between(usize, usize),
    Unknown,
}

#[derive(Fail, Debug, PartialEq)]
pub enum VerboseError<T: Token> {
    #[fail(display = "incomplete input")]
    Incomplete(CompletionRequirement),
    #[fail(display = "unexpected token")]
    Unexpected(UnexpectedToken<T>),
}

impl<T> Error<T> for VerboseError<T>
where
    T: Token,
{
    fn is_fatal(&self) -> bool {
        match self {
            VerboseError::Unexpected(_) => true,
            VerboseError::Incomplete(_) => false,
        }
    }

    fn unexpected(unexpected: UnexpectedToken<T>) -> Self {
        VerboseError::Unexpected(unexpected)
    }

    fn incomplete(requirement: CompletionRequirement) -> Self {
        VerboseError::Incomplete(requirement)
    }
}
