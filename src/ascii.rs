use crate::input::Input;
use crate::pass::{Pass, PassInput, PassResult};

use crate::core::*;

pub fn ascii_alpha_lower<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(in_range(b'a', b'z'), "valid ascii alpha lower")(pass)
}

pub fn ascii_alpha_upper<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(in_range(b'A', b'Z'), "valid ascii alpha upper")(pass)
}

pub fn ascii_alpha<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(or(ascii_alpha_lower, ascii_alpha_upper), "valid ascii alpha")(pass)
}

pub fn ascii_alphanum<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(or(ascii_alpha, ascii_digit), "valid ascii alphanum")(pass)
}

pub fn ascii_digit<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(in_range(b'0', b'9'), "valid ascii digit")(pass)
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_valid_ascii_digit() {
        let pass = test_pass(b"1");

        assert_matches!(
            ascii_digit(pass),
            Ok((b'1', pass_out)) => {
                assert_eq!(pass_out, empty_pass())
            }
        );
    }
}
