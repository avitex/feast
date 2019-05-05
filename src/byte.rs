use crate::input::Input;
use crate::pass::{Pass, PassInput, PassResult};

use crate::core::*;

pub fn ascii_digit<P>(pass: P) -> PassResult<P, u8>
where
    P: Pass,
    PassInput<P>: Input<Token = u8>,
{
    in_range(b'0', b'9', "valid ascii digit")(pass)
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
