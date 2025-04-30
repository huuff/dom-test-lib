use crate::framework::Framework;

use super::{TestWrapper, TestWrapperState};

/// The state for a [`TestWrapper`] where several elements (or none) may have been selected
pub struct Many<T> {
    pub(crate) elems: Vec<T>,
}

impl<T> TestWrapperState for Many<T> {}

impl<Fw: Framework, T> TestWrapper<Many<T>, Fw> {
    pub fn len(&self) -> usize {
        self.state.elems.len()
    }
}
