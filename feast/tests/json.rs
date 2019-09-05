// use crate::*;
// use crate::pass::{SlicePass, SlicePassContext, VerboseError};

// use assert_matches::assert_matches;

// type JsonContext<'i> = SlicePassContext<'i, u8>;
// type JsonError<'i> = VerboseError<'i, JsonContext>;
// type JsonPass<'i> = SlicePass<'i, u8, JsonError>;

// fn json_parse<'i>(input: &'i [u8]) -> JsonPass<'i> {
//     // take whitespace
//     // "
//     // [
//     // {
//     // 1
//     // true
//     // false
//     // null
// }

// fn empty_pass() -> TestPass {
//     test_pass(b"")
// }

// #[test]
// fn test_valid_ascii_digit() {
//     let pass = test_pass(b"1");

//     assert_matches!(
//         ascii_digit(pass.clone()),
//         Ok((b'1', pass_out)) => {
//             assert_eq!(pass_out, empty_pass())
//         }
//     );

//     assert_matches!(
//         parse_ascii_digit(pass),
//         Ok((1, pass_out)) => {
//             assert_eq!(pass_out, empty_pass())
//         }
//     );
// }
