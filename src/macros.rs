macro_rules! impl_complete_capture {
    ($ty:ty) => {
        impl Capture for $ty {
            type Value = Self;

            fn is_complete(&self) -> bool {
                true
            }

            fn resolve(&mut self) {
                ()
            }

            fn into_value(self) -> Self::Value {
                self
            }
        }
    };
}
