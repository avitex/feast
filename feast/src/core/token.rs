use crate::input::{ExpectedHint, Input, Token, TokenTag, Unexpected};
use crate::pass::{Pass, PassInput, PassResult, PassToken};

pub fn token<'i, P, T>(token: T) -> impl Fn(P) -> PassResult<'i, P, T>
where
    P: Pass<'i>,
    T: Token,
    PassInput<'i, P>: Input<'i, Token = T>,
{
    take_token_if(move |input_token: &T| token == *input_token)
}

pub fn take_token<'i, P>() -> impl Fn(P) -> PassResult<'i, P, PassToken<'i, P>>
where
    P: Pass<'i>,
{
    move |pass: P| {
        let input = pass.input();
        let ((token, rest), pass) = pass.with_input_result(input.split_first())?;
        Ok((token, pass.commit(rest)))
    }
}

pub fn take_token_if<'i, P, F>(pred: F) -> impl Fn(P) -> PassResult<'i, P, PassToken<'i, P>>
where
    P: Pass<'i>,
    F: Fn(&PassToken<'i, P>) -> bool,
{
    move |pass: P| {
        let input = pass.input();
        let ((token, rest), pass) = pass.with_input_result(input.split_first())?;
        if pred(&token) {
            Ok((token, pass.commit(rest)))
        } else {
            Err(pass.with_input_error_unexpected(Unexpected {
                unexpected: TokenTag::Token(token),
                expecting: ExpectedHint::None,
            }))
        }
    }
}
