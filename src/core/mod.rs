mod hinting;

use crate::input::{ExpectedHint, Input, InputMarker, Requirement, Token, TokenTag, Unexpected};
use crate::pass::{Pass, PassInput, PassResult, PassSection, PassToken};

pub use self::hinting::*;

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

pub fn token<'p, P, T>(token: T) -> impl Fn(P) -> PassResult<'p, P, T>
where
    P: Pass<'p>,
    T: Token + 'p,
    PassInput<'p, P>: Input<'p, Token = T>,
{
    take_one_if(move |input_token: &T| token == *input_token)
}

////////////////////////////////////////////////////////////////////////////////

pub fn take_one<'p, P>() -> impl Fn(P) -> PassResult<'p, P, PassToken<'p, P>>
where
    P: Pass<'p>,
{
    move |pass: P| {
        let input = pass.input();
        let ((token, rest), pass) = pass.with_input_result(input.split_first())?;
        Ok((token, pass.commit(rest)))
    }
}

pub fn take<'p, P>(n: usize) -> impl Fn(P) -> PassResult<'p, P, PassSection<'p, P>>
where
    P: Pass<'p>,
{
    move |pass: P| {
        let input = pass.input();
        let ((taken, rest), pass) = pass.with_input_result(input.split_at(n))?;
        Ok((taken, pass.commit(rest)))
    }
}

pub fn take_until<'p, P, F>(pred: F) -> impl Fn(P) -> PassResult<'p, P, PassSection<'p, P>>
where
    P: Pass<'p>,
    F: Fn(&PassToken<'p, P>) -> bool,
{
    move |pass: P| {
        let input = pass.input();
        let mut marker = input.marker();
        loop {
            match marker.next() {
                Some(ref token) if pred(token) => {
                    let mark = marker.mark();
                    let ((taken, rest), pass) = pass.with_input_result(input.split_mark(mark))?;
                    return Ok((taken, pass.commit(rest)));
                }
                Some(_) => continue,
                None => break,
            }
        }
        pass.with_input_error_incomplete(Requirement::Unknown)
    }
}

pub fn take_one_if<'p, P, F>(pred: F) -> impl Fn(P) -> PassResult<'p, P, PassToken<'p, P>>
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
            pass.with_input_error_unexpected(Unexpected {
                unexpected: TokenTag::Token(token),
                expecting: ExpectedHint::None,
            })
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn in_range<'p, P, T>(start: T, end: T) -> impl Fn(P) -> PassResult<'p, P, T>
where
    P: Pass<'p>,
    T: Token + PartialOrd + 'p,
    PassInput<'p, P>: Input<'p, Token = T>,
{
    take_one_if(move |token: &T| start <= *token && *token <= end)
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
