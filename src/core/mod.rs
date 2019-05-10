mod hinting;
mod input;
mod token;

use crate::input::{ExpectedHint, Input, Token, TokenTag, Unexpected};
use crate::pass::{Pass, PassInput, PassResult, PassSection};

pub use self::hinting::*;
pub use self::input::*;
pub use self::token::*;

pub fn tag<'p, P, T>(tag: &'p [T]) -> impl Fn(P) -> PassResult<'p, P, PassSection<'p, P>>
where
    P: Pass<'p>,
    T: Token,
    PassInput<'p, P>: Input<'p, Token = T>,
{
    move |pass: P| {
        let tag_len = tag.len();
        let input = pass.input();
        let ((input_tag, rest), pass) = pass.with_input_result(input.split_at(tag_len))?;
        for i in 0..tag_len {
            if tag[i] == input_tag[i] {
                continue;
            } else {
                return pass.with_input_error_unexpected(Unexpected {
                    unexpected: TokenTag::Token(input_tag[i].clone()),
                    expecting: ExpectedHint::Tag(tag),
                });
            }
        }
        Ok((input_tag, pass.commit(rest)))
    }
}

pub fn in_range<'p, P, T>(start: T, end: T) -> impl Fn(P) -> PassResult<'p, P, T>
where
    P: Pass<'p>,
    T: Token + PartialOrd + 'p,
    PassInput<'p, P>: Input<'p, Token = T>,
{
    take_token_if(move |token: &T| start <= *token && *token <= end)
}

pub fn or<'p, P, A, B, O>(a: A, b: B) -> impl Fn(P) -> PassResult<'p, P, O>
where
    P: Pass<'p>,
    A: Fn(P) -> PassResult<'p, P, O>,
    B: Fn(P) -> PassResult<'p, P, O>,
{
    move |pass: P| {
        match a(pass) {
            Err((_err_a, pass)) => match b(pass) {
                Err((err_b, pass)) => {
                    // TODO: Better or error
                    Err((err_b, pass))
                }
                ok => ok,
            },
            ok => ok,
        }
    }
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

pub fn map<'p, P, F, FO, M, O>(inner: F, mapper: M) -> impl Fn(P) -> PassResult<'p, P, O>
where
    P: Pass<'p>,
    F: Fn(P) -> PassResult<'p, P, FO>,
    M: Fn(FO) -> O,
{
    move |pass: P| match inner(pass) {
        Ok((val, pass)) => Ok((mapper(val), pass)),
        Err(err) => Err(err),
    }
}

pub fn and_then<'p, P, F, FO, T, O>(inner: F, then: T) -> impl Fn(P) -> PassResult<'p, P, O>
where
    P: Pass<'p>,
    F: Fn(P) -> PassResult<'p, P, FO>,
    T: Fn((FO, P)) -> PassResult<'p, P, O>,
{
    move |pass: P| match inner(pass) {
        Ok((val, pass)) => then((val, pass)),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ascii::*;
    use crate::pass::{SlicePass, SlicePassContext, VerboseError};

    use assert_matches::assert_matches;

    type TestContext = SlicePassContext<'static, u8>;
    type TestError = VerboseError<'static, TestContext>;
    type TestPass = SlicePass<'static, u8, TestError>;

    fn test_pass(input: &'static [u8]) -> TestPass {
        TestPass::from(input)
    }

    fn empty_pass() -> TestPass {
        test_pass(b"")
    }

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
    fn test_map_simple() {
        let pass = test_pass(b"1");

        assert_matches!(
            map(ascii_digit, |digit| digit as char)(pass.clone()),
            Ok(('1', pass_out)) => {
                assert_eq!(pass_out, empty_pass());
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
