pub mod asserts;
pub mod interaction;
pub mod traversal;

use std::ops::Deref;

use super::{TestWrapper, TestWrapperState};

/// A wrapper that has selected a single element
pub struct Single<T>(pub(super) T);
impl<T> TestWrapperState for Single<T> {}

impl<T> Deref for TestWrapper<Single<T>> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state.0
    }
}
