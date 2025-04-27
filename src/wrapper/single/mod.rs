pub mod asserts;
pub mod interaction;
pub mod traversal;

use std::ops::Deref;

use crate::framework::Framework;

use super::{TestWrapper, TestWrapperState};

/// A wrapper that has selected a single element
pub struct Single<T>(pub(super) T);
impl<T> TestWrapperState for Single<T> {}

impl<T, Fw: Framework> Deref for TestWrapper<Single<T>, Fw> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state.0
    }
}
