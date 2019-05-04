#[derive(Debug, PartialEq)]
pub struct BasicByteSlicePass<'a> {
    pub input: SliceInput<'a, u8>,
}

impl<'a> Pass for BasicByteSlicePass<'a> {
    type Token = u8;
    type Input = SliceInput<'a, Self::Token>;

    type Error = SilentError;
    type InputError = SilentError;

    fn input(&self) -> Self::Input {
        self.input.clone()
    }

    fn commit(self, rest: Self::Input) -> Self {
        Self { input: rest }
    }

    fn input_error(self, _err: Self::InputError) -> Self::Error {
        SilentError
    }

    fn input_error_unexpected(self, _unexpected: UnexpectedToken<Self::Token>) -> Self::Error {
        SilentError
    }
}