use crate::input::{ExpectedHint, Input, UnexpectedToken};
use crate::pass::{Pass, PassResult};

fn one_if<P, F>(predictate: F, description: &'static str) -> impl Fn(P) -> PassResult<P, P::Token>
where
    P: Pass,
    F: Fn(P::Token) -> Result<P::Token, P::Token>,
{
    move |pass: P| {
        let (token, rest) = pass.input().split_first()?;
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
    P: Pass<Token = u8>,
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
    P: Pass<Token = u8>,
{
    in_byte_range(b'0', b'9', "valid ascii digit")(pass)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::pass::BasicByteSlicePass;

//     fn test_pass(input: &'static [u8]) -> impl Pass<Token = u8> {
//         BasicByteSlicePass {
//             input: input.into(),
//         }
//     }

//     fn empty_pass() -> impl Pass<Token = u8> {
//         test_pass(b"")
//     }

//     #[test]
//     fn valid_ascii_digit() {
//         let pass = test_pass(b"1");

//         assert_eq!(ascii_digit(pass), Ok((b'1', empty_pass())));
//     }
// }
