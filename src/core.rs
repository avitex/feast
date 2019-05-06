use crate::input::{ExpectedHint, Input, Token, UnexpectedToken};
use crate::pass::{Pass, PassError, PassInput, PassResult, PassSection, PassToken};

pub fn tag<'p, P, T>(tag: &'p [T]) -> impl Fn(P) -> PassResult<'p, P, PassSection<'p, P>>
where
    P: Pass<'p>,
    T: Token,
    PassInput<'p, P>: Input<'p, Token = T>,
{
    move |pass: P| {
        let tag_len = tag.len();
        let ((input_tag, rest), pass) = pass.split_at::<PassError<'p, P>>(tag_len)?;
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

pub fn one_if<'p, P, F>(
    predictate: F,
    description: &'static str,
) -> impl Fn(P) -> PassResult<'p, P, PassToken<'p, P>>
where
    P: Pass<'p>,
    F: Fn(PassToken<'p, P>) -> Result<PassToken<'p, P>, PassToken<'p, P>>,
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

pub fn token<'p, P, T>(token: T, description: &'static str) -> impl Fn(P) -> PassResult<'p, P, T>
where
    P: Pass<'p>,
    T: Token + 'p,
    PassInput<'p, P>: Input<'p, Token = T>,
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

pub fn in_range<'p, P, T>(
    start: T,
    end: T,
    description: &'static str,
) -> impl Fn(P) -> PassResult<'p, P, T>
where
    P: Pass<'p>,
    T: Token + 'p,
    PassInput<'p, P>: Input<'p, Token = T>,
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

pub fn peek<'p, P, F, O>(inner: F) -> impl Fn(P) -> PassResult<'p, P, O>
where
    P: Pass<'p>,
    F: Fn(P) -> PassResult<'p, P, O>,
{
    move |pass: P| {
        let input = pass.input();
        match inner(pass) {
            Ok((out, pass)) => Ok((out, pass.commit(input))),
            err => err,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::byte::*;
    use crate::pass::{SlicePass, SlicePassContext, VerboseError};

    use assert_matches::assert_matches;

    type TestContext = SlicePassContext<'static, u8>;
    type TestError = VerboseError<'static, TestContext>;
    type TestPass = SlicePass<'static, u8, TestError>;

    fn test_pass(input: &'static [u8]) -> TestPass {
        TestPass::from(input)
    }

    // fn empty_pass() -> TestPass {
    //     test_pass(b"")
    // }

    #[test]
    fn test_peek_simple() {
        let pass = test_pass(b"1");

        assert_matches!(
            peek(ascii_digit)(pass.clone()),
            Ok((b'1', pass_out)) => {
                assert_eq!(pass_out, pass);
            }
        );
    }

    #[test]
    fn test_peek_tag() {
        let raw = &b"hello"[..];
        let pass = test_pass(raw);
        let input_tag = tag(raw);

        assert_matches!(
            peek(input_tag)(pass.clone()),
            Ok((input_out, pass_out)) => {
                assert_eq!(input_out, raw.into());
                assert_eq!(pass_out, pass);
            }
        );
    }
}
