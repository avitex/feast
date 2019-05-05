use crate::input::{ExpectedHint, Input, UnexpectedToken};
use crate::pass::{Pass, PassInput, PassResult, PassToken};

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

pub fn in_byte_range<P>(
    start: u8,
    end: u8,
    description: &'static str,
) -> impl Fn(P) -> PassResult<P, u8>
where
    P: Pass,
    PassInput<P>: Input<Token = u8>,
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

pub fn ascii_digit<P>(pass: P) -> PassResult<P, u8>
where
    P: Pass,
    PassInput<P>: Input<Token = u8>,
{
    in_byte_range(b'0', b'9', "valid ascii digit")(pass)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pass::{SlicePass, SlicePassContext, VerboseError};

    type TestPass = SlicePass<'static, u8, VerboseError<SlicePassContext<'static, u8>>>;

    fn test_pass(input: &'static [u8]) -> TestPass {
        TestPass::from(input)
    }

    fn empty_pass() -> TestPass {
        test_pass(b"")
    }

    #[test]
    fn valid_ascii_digit() {
        let pass = test_pass(b"1");

        assert_eq!(ascii_digit(pass), Ok((b'1', empty_pass())));
    }
}
