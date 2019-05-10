pub trait Capture {
    /// The value type captured.
    type Value;

    /// Returns the whether or not the capture is complete.
    ///
    /// Captures are complete when either created from a determinate value,
    /// or have been resolved from an indeterminate value.
    fn is_complete(&self) -> bool;

    /// Resolve the the value to be complete.
    fn resolve(&mut self);

    /// Consume the capture to get the value.
    fn into_value(self) -> Self::Value;

    // /// Map a capture to another.
    fn map<C, B>(self) -> C
    where
        Self: Sized,
        C: BuildableCapture,
        C::Value: From<Self::Value>,
    {
        if self.is_complete() {
            C::from_determinate(self.into_value().into())
        } else {
            C::from_indeterminate(self.into_value().into())
        }
    }
}

pub trait BuildableCapture: Capture {
    /// Build a capture from a complete value.
    fn from_determinate(value: Self::Value) -> Self;

    /// Build a capture from a indeterminate value.
    ///
    /// Capture may internally resolve the value to be complete
    /// if the capture source itself is complete.
    fn from_indeterminate(value: Self::Value) -> Self;
}

/// A capture that can be indeterminate.
#[derive(Debug, PartialEq)]
pub struct StreamCapture<T> {
    value: T,
    complete: bool,
}

impl<T> Capture for StreamCapture<T> {
    type Value = T;

    fn is_complete(&self) -> bool {
        return self.complete;
    }

    fn resolve(&mut self) {
        self.complete = true;
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}

impl<T> BuildableCapture for StreamCapture<T> {
    fn from_determinate(value: T) -> Self {
        Self {
            value,
            complete: true,
        }
    }

    fn from_indeterminate(value: T) -> Self {
        Self {
            value,
            complete: false,
        }
    }
}

/// A capture that cannot be indeterminate.
#[derive(Debug, PartialEq)]
pub struct CompleteCapture<T> {
    value: T,
}

impl<T> Capture for CompleteCapture<T> {
    type Value = T;

    fn is_complete(&self) -> bool {
        true
    }

    fn resolve(&mut self) {
        ()
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}

impl<T> BuildableCapture for CompleteCapture<T> {
    fn from_determinate(value: T) -> Self {
        Self::from(value)
    }

    fn from_indeterminate(value: T) -> Self {
        Self::from(value)
    }
}

impl<T> From<T> for CompleteCapture<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}
