pub trait Capture {
    type Value;

    fn is_complete(&self) -> bool;

    fn is_indeterminate(&self) -> bool {
        return self.is_complete()
    }

    fn from_complete(value: Self::Value) -> Self;

    fn from_indeterminate(value: Self::Value) -> Self;

    fn into_value(self) -> Self::Value;
}

#[derive(Debug, PartialEq)]
pub struct StreamCapture<T> {
    value: T,
    complete: bool
}

impl<T> Capture for StreamCapture<T> {
    type Value = T;

    fn is_complete(&self) -> bool {
        return self.complete
    }

    fn from_complete(value: Self::Value) -> Self {
        Self { value, complete: true }
    }

    fn from_indeterminate(value: Self::Value) -> Self {
        Self { value, complete: false }
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}

/// Will always resolve to complete, even if created 
/// from an indeterminate capture.
#[derive(Debug, PartialEq)]
pub struct CompleteCapture<T> {
    value: T
}

impl<T> Capture for CompleteCapture<T> {
    type Value = T;

    fn is_complete(&self) -> bool {
        true
    }

    fn from_complete(value: Self::Value) -> Self {
        Self { value }
    }

    fn from_indeterminate(value: Self::Value) -> Self {
        Self::from_complete(value)
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}

impl<T> From<T> for CompleteCapture<T> {
    fn from(value: T) -> Self {
        Self::from_complete(value)
    }
}