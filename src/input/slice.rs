use super::*;

use std::ops::Index;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SliceInput<'i, T: Token>(pub &'i [T]);

impl<'i, T: 'i> SliceInput<'i, T>
where
    T: Token,
{
    const EMPTY_INPUT: &'i [T] = &[];

    pub fn empty() -> Self {
        Self(Self::EMPTY_INPUT)
    }
}

impl<'i, T> Input<'i> for SliceInput<'i, T>
where
    T: Token,
{
    type Mark = usize;
    type Token = T;
    type Section = Self;
    type Marker = SliceMarker<'i, T>;

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn split_first<E>(self) -> Result<(Self::Token, Self), E>
    where
        E: Error<'i, Token = Self::Token>,
    {
        self.0
            .split_first()
            .map(|(token, rest)| (token.clone(), Self(rest)))
            .ok_or_else(|| E::incomplete(Requirement::Exact(1)))
    }

    fn split_at<E>(self, mid: usize) -> Result<(Self::Section, Self), E>
    where
        E: Error<'i, Token = Self::Token>,
    {
        if mid > self.len() {
            Err(E::incomplete(Requirement::Exact(mid - self.len())))
        } else {
            let (consumed, rest) = self.0.split_at(mid);
            Ok((Self(consumed), Self(rest)))
        }
    }

    fn split_mark<E>(self, mark: Self::Mark) -> Result<(Self::Section, Self), E>
    where
        E: Error<'i, Token = Self::Token>,
    {
        self.split_at(mark)
    }

    fn marker(&self) -> Self::Marker {
        SliceMarker::from(self.0)
    }
}

impl<'i, T> ExactSizeInput<'i> for SliceInput<'i, T>
where
    T: Token,
{
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'i, T> Index<usize> for SliceInput<'i, T>
where
    T: Token,
{
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<'i, T> AsRef<[T]> for SliceInput<'i, T>
where
    T: Token,
{
    fn as_ref<'b>(&'b self) -> &'b [T] {
        self.0
    }
}

impl<'i, T> From<&'i [T]> for SliceInput<'i, T>
where
    T: Token,
{
    fn from(slice: &'i [T]) -> Self {
        Self(slice)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct SliceMarker<'a, T: Token> {
    cursor: usize,
    items: &'a [T],
}

impl<'a, T> SliceMarker<'a, T>
where
    T: Token,
{
    pub fn at_end(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> InputMarker for SliceMarker<'a, T>
where
    T: Token,
{
    type Mark = usize;
    type Token = T;

    fn skip(&mut self, n: usize) -> bool {
        if self.len() > n {
            self.cursor += n;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<Self::Token> {
        if self.at_end() {
            None
        } else {
            Some(self.items[self.cursor].clone())
        }
    }

    fn child(&self) -> Self {
        self.clone()
    }

    fn mark(&self) -> Self::Mark {
        self.cursor
    }
}

impl<'a, T> Iterator for SliceMarker<'a, T>
where
    T: Token,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end() {
            None
        } else {
            let next = self.items[self.cursor].clone();
            self.cursor += 1;
            Some(next)
        }
    }
}

impl<'a, T> ExactSizeIterator for SliceMarker<'a, T>
where
    T: Token,
{
    fn len(&self) -> usize {
        self.items.len() - self.cursor
    }
}

impl<'i, T> From<&'i [T]> for SliceMarker<'i, T>
where
    T: Token,
{
    fn from(items: &'i [T]) -> Self {
        Self { cursor: 0, items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type MockToken = u8;
    type MockError = ErrorReason<'static, MockToken>;

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
            Err(ErrorReason::Incomplete(Requirement::Exact(1)))
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
            Err(ErrorReason::Incomplete(Requirement::Exact(1)))
        );
    }

    #[test]
    fn test_slice_input_marker() {
        assert_eq!(mock_slice_input().marker().next(), Some(b'h'));
    }

    // #[test]
    // fn test_slice_input_split_pair() {
    //     assert_eq!(
    //         mock_slice_input().split_pair::<MockError, _>(|t| *t == b':'),
    //         Ok((slice_input(b"hello"), slice_input(b"world")))
    //     );

    //     assert_eq!(
    //         mock_slice_input().split_pair::<MockError, _>(|t| *t == b'd'),
    //         Ok((slice_input(b"hello:worl"), empty_slice_input()))
    //     );

    //     assert_eq!(
    //         mock_slice_input().split_pair::<MockError, _>(|t| *t == b'h'),
    //         Ok((empty_slice_input(), slice_input(b"ello:world")))
    //     );

    //     assert_eq!(
    //         mock_slice_input().split_pair::<MockError, _>(|t| *t == b'?'),
    //         Err(ErrorReason::Incomplete(Requirement::Unknown))
    //     );
    // }
}
