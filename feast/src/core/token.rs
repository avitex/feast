use crate::input::{ExpectedHint, Input, Token, TokenTag, Unexpected};
use crate::pass::{Pass, PassInput, PassResult, PassToken};

pub fn token<'p, P, T>(token: T) -> impl Fn(P) -> PassResult<'p, P, T>
where
    P: Pass<'p>,
    T: Token,
    PassInput<'p, P>: Input<'p, Token = T>,
{
    take_token_if(move |input_token: &T| token == *input_token)
}

pub fn take_token<'p, P>() -> impl Fn(P) -> PassResult<'p, P, PassToken<'p, P>>
where
    P: Pass<'p>,
{
    move |pass: P| {
        let input = pass.input();
        let ((token, rest), pass) = pass.with_input_result(input.split_first())?;
        Ok((token, pass.commit(rest)))
    }
}

pub fn take_token_if<'p, P, F>(pred: F) -> impl Fn(P) -> PassResult<'p, P, PassToken<'p, P>>
where
    P: Pass<'p>,
    F: Fn(&PassToken<'p, P>) -> bool,
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
