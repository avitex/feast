// pub struct VerbosePass<I: Input> {
//     input: I
// }

// impl<'a, I> Pass for &'a mut VerbosePass<I>
// where
//     I: Input,
//     I::Token: 'static
// {
//     type Token = I::Token;
//     type Input = &'a I;

//     type Error = VerbosePassError<Self::Token>;
//     type InputError = VerboseInputError<Self::Token>;

//     fn input(self) -> Self::Input {
//         self.input
//     }

//     fn split_one(self) -> Result<(Self::Token, Self), Self::Error> {

//     }

//     fn input_error(self, err: Self::InputError) -> Self::Error {

//     }

//     fn input_error_unexpected(self, unexpected: UnexpectedToken<Self::Token>) -> Self::Error {

//     }
// }