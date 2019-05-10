use super::Capture;
use std::fmt::Debug;

pub trait IntoBytes {
    type Bytes: AsRef<[u8]>;

    fn into_bytes<'a>(self) -> Self::Bytes;
}

pub trait Token: Sized + Capture + Debug + Clone + IntoBytes + Eq {
    fn byte_size() -> Option<usize>;
    fn is_ascii(&self) -> bool;
}

impl Token for u8 {
    #[inline]
    fn byte_size() -> Option<usize> {
        Some(1)
    }

    #[inline]
    fn is_ascii(&self) -> bool {
        return self.is_ascii();
    }
}

impl_complete_capture!(u8);

impl IntoBytes for u8 {
    type Bytes = [u8; 1];

    fn into_bytes<'a>(self) -> Self::Bytes {
        [self]
    }
}

impl Token for char {
    #[inline]
    fn byte_size() -> Option<usize> {
        Some(4)
    }

    #[inline]
    fn is_ascii(&self) -> bool {
        return self.is_ascii();
    }
}

impl_complete_capture!(char);

impl IntoBytes for char {
    type Bytes = [u8; 4];

    fn into_bytes(self) -> Self::Bytes {
        let mut bytes = [0u8; 4];
        self.encode_utf8(&mut bytes[..]);
        bytes
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenTag<'a, T: Token> {
    Token(T),
    Tag(&'a [T]),
}

impl<'a, T> TokenTag<'a, T>
where
    T: Token,
{
    pub fn is_token(&self) -> bool {
        match self {
            TokenTag::Token(_) => true,
            _ => false,
        }
    }
}

impl<'a, T> IntoBytes for TokenTag<'a, T>
where
    T: Token,
{
    type Bytes = Vec<u8>;

    fn into_bytes(self) -> Self::Bytes {
        match self {
            TokenTag::Tag(tag) => {
                let token_len = T::byte_size().unwrap_or(1);
                let mut bytes = Vec::with_capacity(token_len * tag.len());
                for token in tag.iter() {
                    bytes.extend_from_slice(token.clone().into_bytes().as_ref())
                }
                bytes
            }
            TokenTag::Token(token) => token.into_bytes().as_ref().into(),
        }
    }
}
