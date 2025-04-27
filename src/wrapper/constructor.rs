use super::empty::Empty;
use crate::{framework::Framework, wrapper::TestWrapper};

pub type BaseTestWrapper<Fw> = TestWrapper<Empty, Fw>;

impl<Fw: Framework> BaseTestWrapper<Fw> {
    pub fn with_root(root: web_sys::Element, ctx: Fw::Context) -> Self {
        Self {
            root,
            state: Empty,
            _framework_ctx: ctx,
        }
    }
}
