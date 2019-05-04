use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SliceInput<'a, T: Token>(pub &'a [T]);

impl<'a, T: 'a> SliceInput<'a, T>
where
    T: Token,
{
    const EMPTY_INPUT: &'a [T] = &[];

    pub fn empty() -> Self {
        Self(Self::EMPTY_INPUT)
    }
}

impl<'a, T> Input for SliceInput<'a, T>
where
    T: Token,
{
    type Token = T;
    type Section = Self;

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn split_first<E>(self) -> Result<(Self::Token, Self), E>
    where
        E: Error,
    {
        self.0
            .split_first()
            .map(|(token, rest)| (token.clone(), Self(rest)))
            .ok_or_else(|| E::incomplete(CompletionRequirement::Exact(1)))
    }

    fn split_pair<E, F>(self, pred: F) -> Result<(Self::Section, Self), E>
    where
        E: Error,
        F: FnMut(&Self::Token) -> bool,
    {
        let mut iter = self.0.splitn(2, pred);
        match (iter.next(), iter.next()) {
            (Some(consumed), Some(rest)) => Ok((Self(consumed), Self(rest))),
            _ => Err(E::incomplete(CompletionRequirement::Unknown)),
        }
    }

    fn split_at<E>(self, mid: usize) -> Result<(Self::Section, Self), E>
    where
        E: Error,
    {
        if mid > self.len() {
            Err(E::incomplete(CompletionRequirement::Exact(
                mid - self.len(),
            )))
        } else {
            let (consumed, rest) = self.0.split_at(mid);
            Ok((Self(consumed), Self(rest)))
        }
    }
}

impl<'a, T> ExactSizeInput for SliceInput<'a, T>
where
    T: Token,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> AsRef<[T]> for SliceInput<'a, T>
where
    T: Token,
{
    fn as_ref<'b>(&'b self) -> &'b [T] {
        self.0
    }
}

impl<'a, T> From<&'a [T]> for SliceInput<'a, T>
where
    T: Token,
{
    fn from(slice: &'a [T]) -> Self {
        Self(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type MockToken = u8;
    type MockError = VerboseError<MockToken>;

    const MOCK_DATA: &[u8] = b"hello:world";

    fn slice_input(input: &'static [u8]) -> SliceInput<'static, MockToken> {
        SliceInput::from(input)
    }

    fn mock_slice_input() -> SliceInput<'static, MockToken> {
        slice_input(MOCK_DATA)
    }

    fn empty_slice_input() -> SliceInput<'static, MockToken> {
        SliceInput::empty()
    }

    #[test]
    fn test_slice_input_split_first() {
        assert_eq!(
            mock_slice_input().split_first::<MockError>(),
            Ok((b'h', slice_input(b"ello:world")))
        );

        assert_eq!(
            empty_slice_input().split_first::<MockError>(),
            Err(VerboseError::Incomplete(CompletionRequirement::Exact(1)))
        );
    }

    #[test]
    fn test_slice_input_split_at() {
        assert_eq!(
            mock_slice_input().split_at::<MockError>(6),
            Ok((slice_input(b"hello:"), slice_input(b"world")))
        );

        assert_eq!(
            mock_slice_input().split_at::<MockError>(MOCK_DATA.len()),
            Ok((mock_slice_input(), empty_slice_input()))
        );

        assert_eq!(
            mock_slice_input().split_at::<MockError>(MOCK_DATA.len() + 1),
            Err(VerboseError::Incomplete(CompletionRequirement::Exact(1)))
        );
    }

    #[test]
    fn test_slice_input_split_pair() {
        assert_eq!(
            mock_slice_input().split_pair::<MockError, _>(|t| *t == b':'),
            Ok((slice_input(b"hello"), slice_input(b"world")))
        );

        assert_eq!(
            mock_slice_input().split_pair::<MockError, _>(|t| *t == b'd'),
            Ok((slice_input(b"hello:worl"), empty_slice_input()))
        );

        assert_eq!(
            mock_slice_input().split_pair::<MockError, _>(|t| *t == b'h'),
            Ok((empty_slice_input(), slice_input(b"ello:world")))
        );

        assert_eq!(
            mock_slice_input().split_pair::<MockError, _>(|t| *t == b'?'),
            Err(VerboseError::Incomplete(CompletionRequirement::Unknown))
        );
    }
}
