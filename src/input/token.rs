use std::fmt::Debug;

pub trait Token: Debug + Clone + IntoBytes + Eq + PartialOrd {
    fn is_ascii(&self) -> bool;
}

impl Token for u8 {
    #[inline]
    fn is_ascii(&self) -> bool {
        return self.is_ascii();
    }
}

impl Token for char {
    #[inline]
    fn is_ascii(&self) -> bool {
        return self.is_ascii();
    }
}

pub trait IntoBytes {
    type Owned: AsRef<[u8]>;

    fn into_bytes(self) -> Self::Owned;
}

impl IntoBytes for u8 {
    type Owned = [u8; 1];

    fn into_bytes(self) -> Self::Owned {
        [self; 1]
    }
}

impl IntoBytes for char {
    type Owned = [u8; 4];

    fn into_bytes(self) -> Self::Owned {
        let mut bytes = [0u8; 4];
        self.encode_utf8(&mut bytes[..]);
        bytes
    }
}
