use failure::Fail;

use super::Pass;
use crate::input;

pub trait Error: Fail {
    fn from_input<P: Pass, E: input::Error>(pass: &P, err: &E) -> Self;
}
