pub trait CaptureBuilder {
    /// Build a capture from a complete value.
    fn from_determinate<C, V>(value: V) -> C
    where
        C: Capture<Value = V>;

    /// Build a capture from a indeterminate value.
    /// 
    /// Capture builders may internally resolve the value to be complete
    /// if the capture source itself is complete.
    fn from_indeterminate<C, V>(value: V) -> C
    where
        C: Capture<Value = V>;

    /// Map one capture to another.
    fn map<A, B>(parent: A) -> B
    where
        A: Capture,
        B: Capture,
        B::Value: From<A::Value>
    {
        if parent.is_complete() {
            Self::from_determinate(parent.into_value().into())
        } else {
            Self::from_indeterminate(parent.into_value().into())
        }
    }
}

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

    /// Map a capture to another.
    fn map<C, B>(self) -> C
    where
        Self: Sized,
        C: Capture,
        C::Value: From<Self::Value>,
        B: CaptureBuilder
    {
        B::map(self)
    }
}

/// A capture that can be indeterminate.
#[derive(Debug, PartialEq)]
pub struct StreamCapture<T> {
    value: T,
    complete: bool
}

impl<T> StreamCapture<T> {
    pub fn from_determinate(value: T) -> Self {
        Self { value, complete: true }
    }

    pub fn from_indeterminate(value: T) -> Self {
        Self { value, complete: false }
    }
}

impl<T> Capture for StreamCapture<T> {
    type Value = T;

    fn is_complete(&self) -> bool {
        return self.complete
    }

    fn resolve(&mut self) {
        self.complete = true;
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}

/// A capture that cannot be indeterminate.
#[derive(Debug, PartialEq)]
pub struct CompleteCapture<T> {
    value: T
}

impl<T> CompleteCapture<T> {
    pub fn from_determinate(value: T) -> Self {
        Self { value }
    }
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

impl<T> From<T> for CompleteCapture<T> {
    fn from(value: T) -> Self {
        Self::from_determinate(value)
    }
}