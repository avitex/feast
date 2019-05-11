use crate::input::Input;
use crate::pass::{Pass, PassInput, PassResult};

use crate::core::*;

/// Consumes a byte if it is an ASCII lowercase character`[a-z]`.
pub fn ascii_lowercase<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(
        take_token_if(u8::is_ascii_lowercase),
        "valid ascii lowercase",
    )(pass)
}

/// Consumes a byte if it is an ASCII uppercase character `[A-Z]`.
pub fn ascii_uppercase<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(
        take_token_if(u8::is_ascii_uppercase),
        "valid ascii uppercase",
    )(pass)
}

/// Consumes a byte if it is an ASCII alphabetic character `[a-zA-Z]`.
pub fn ascii_alphabetic<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(
        take_token_if(u8::is_ascii_alphabetic),
        "valid ascii alphabetic",
    )(pass)
}

/// Consumes a byte if it is an ASCII decimal digit `[0-9]`.
pub fn ascii_digit<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(
        take_token_if(u8::is_ascii_digit),
        "valid ascii decimal digit",
    )(pass)
}

/// Consumes a byte if it is an ASCII hexadecimal digit `[0-9A-Fa-f]`.
pub fn ascii_hexdigit<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(
        take_token_if(u8::is_ascii_hexdigit),
        "valid ascii hex digit",
    )(pass)
}

/// Consumes a byte if it is an ASCII alphanumeric character `[a-zA-Z0-9]`.
pub fn ascii_alphanumeric<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    hint(
        take_token_if(u8::is_ascii_alphanumeric),
        "valid ascii alphanumeric",
    )(pass)
}

/// Parses a ASCII decimal digit.
pub fn parse_ascii_digit<'p, P>(pass: P) -> PassResult<'p, P, u8>
where
    P: Pass<'p>,
    PassInput<'p, P>: Input<'p, Token = u8>,
{
    map(ascii_digit, |t| t - b'0')(pass)
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
            ascii_digit(pass.clone()),
            Ok((b'1', pass_out)) => {
                assert_eq!(pass_out, empty_pass())
            }
        );

        assert_matches!(
            parse_ascii_digit(pass),
            Ok((1, pass_out)) => {
                assert_eq!(pass_out, empty_pass())
            }
        );
    }
}
