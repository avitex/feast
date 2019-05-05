use crate::input::{ExpectedHint, Input, Token, UnexpectedToken};
use crate::pass::{Pass, PassError, PassInput, PassResult, PassSection, PassToken};

pub fn tag<P, T>(tag: &'static [T]) -> impl Fn(P) -> PassResult<P, PassSection<P>>
where
    P: Pass,
    T: Token,
    PassInput<P>: Input<Token = T>,
{
    move |pass: P| {
        let tag_len = tag.len();
        let ((input_tag, rest), pass) = pass.split_at::<PassError<P>>(tag_len)?;
        for i in 0..tag_len {
            if tag[i] == input_tag[i] {
                continue;
            } else {
                return Err(pass.input_error_unexpected(UnexpectedToken {
                    unexpected: input_tag[i].clone(),
                    expecting: ExpectedHint::Tag(tag),
                }));
            }
        }
        Ok((input_tag, pass.commit(rest)))
    }
}

pub fn one_if<P, F>(
    predictate: F,
    description: &'static str,
) -> impl Fn(P) -> PassResult<P, PassToken<P>>
where
    P: Pass,
    F: Fn(PassToken<P>) -> Result<PassToken<P>, PassToken<P>>,
{
    move |pass: P| {
        let ((token, rest), pass) = pass.split_first()?;
        match predictate(token) {
            Ok(token) => Ok((token, pass.commit(rest))),
            Err(token) => Err(pass.input_error_unexpected(UnexpectedToken {
                unexpected: token,
                expecting: ExpectedHint::Description(description),
            })),
        }
    }
}

pub fn token<P, T>(token: T, description: &'static str) -> impl Fn(P) -> PassResult<P, T>
where
    P: Pass,
    T: Token,
    PassInput<P>: Input<Token = T>,
{
    let predictate = move |input_token| {
        if token == input_token {
            Ok(input_token)
        } else {
            Err(input_token)
        }
    };
    one_if(predictate, description)
}

pub fn in_range<P, T>(start: T, end: T, description: &'static str) -> impl Fn(P) -> PassResult<P, T>
where
    P: Pass,
    T: Token,
    PassInput<P>: Input<Token = T>,
{
    let predictate = move |token| {
        if start <= token && token <= end {
            Ok(token)
        } else {
            Err(token)
        }
    };
    one_if(predictate, description)
}
