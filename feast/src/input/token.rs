use std::slice;
use std::fmt::Debug;

use super::Capture;

/// A `Token` is the smallest unit of input, with a fixed size,
/// that can be copied without worrying about performance.
///
/// For zero-copy guarantees over input this is the largest type
/// that can and should be re-allocated.
pub trait Token: Sized + Debug + Copy + Eq + 'static {}

/// A `BytesToken` extends a standard `Token` with the guarantee
/// that it is a stable, fixed size representation of raw bytes.
pub trait BytesToken: Token {
    /// The bytes container for the token.
    type Bytes: AsRef<[u8]>;

    /// The fixed byte size of the token.
    fn byte_size() -> usize;

    /// Converts the token into raw bytes.
    fn into_bytes(self) -> Self::Bytes;
}

impl<T> Capture for T
where
    T: Token,
{
    type Value = T;

    /// A `Token` always returns true as it either is produced or
    /// not produced from input.
    #[inline]
    fn is_complete(&self) -> bool {
        true
    }

    /// It's already complete, resolving is a nop.
    #[inline]
    fn resolve(&mut self) {
        ()
    }

    /// Passes the token through.
    #[inline]
    fn into_value(self) -> Self::Value {
        self
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Token for u8 {}

impl BytesToken for u8 {
    type Bytes = [u8; 1];

    #[inline]
    fn byte_size() -> usize {
        1
    }

    #[inline]
    fn into_bytes<'a>(self) -> Self::Bytes {
        [self]
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Token for char {}

impl BytesToken for char {
    type Bytes = [u8; 4];

    #[inline]
    fn byte_size() -> usize {
        4
    }

    #[inline]
    fn into_bytes(self) -> Self::Bytes {
        let mut bytes = [0u8; 4];
        self.encode_utf8(&mut bytes[..]);
        bytes
    }
}

///////////////////////////////////////////////////////////////////////////////

/// A TokenTag is a container for either a token or a tag.
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

impl<'a, T> TokenTag<'a, T>
where
    T: BytesToken,
{
    /// The total token byte size.
    pub fn byte_size(&self) -> usize {
        if let TokenTag::Tag(tag) = self {
            T::byte_size() * tag.len()
        } else {
            T::byte_size()
        }
    }

    /// Writes the TokenTag value into a byte buffer.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough.
    pub fn copy_into_buf(&self, mut buf: &mut [u8]) {
        match self {
            TokenTag::Tag(tag) => {
                for token in tag.iter() {
                    let buf_slice = &mut buf[0..T::byte_size()];
                    buf_slice.copy_from_slice(token.clone().into_bytes().as_ref());
                    buf = &mut buf[T::byte_size()..];
                }
            }
            TokenTag::Token(token) => {
                let buf_slice = &mut buf[0..T::byte_size()];
                buf_slice.copy_from_slice(token.into_bytes().as_ref());
            }
        }
    }

    /// Converts the TagToken into bytes.
    pub fn to_bytes_vec(&self) -> Vec<u8> {
        let len = self.byte_size();
        let mut buf = Vec::with_capacity(len);
        unsafe {
            let buf_slice = slice::from_raw_parts_mut(buf.as_mut_ptr(), len);
            self.copy_into_buf(buf_slice);
            buf.set_len(len);
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_token_into_bytes() {
        let token = '?';
        assert_eq!(token.into_bytes().as_ref(), &[b'?', 0u8, 0u8, 0u8]);
    }

    #[test]
    fn test_byte_token_into_bytes() {
        let token = b'?';
        assert_eq!(token.into_bytes().as_ref(), &[b'?']);
    }

    #[test]
    fn test_token_tag_into_bytes() {
        let tag = TokenTag::Tag(&['o', 'k']);
        assert_eq!(tag.to_bytes_vec().as_slice(), &[b'o', 0u8, 0u8, 0u8, b'k', 0u8, 0u8, 0u8]);
    }
}