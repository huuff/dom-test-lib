use super::empty::Empty;
use crate::{framework::Framework, wrapper::TestWrapper};

cfg_if::cfg_if! {
    if #[cfg(feature = "leptos")] {
        pub type BaseTestWrapper<Fw = crate::framework::leptos::Leptos> = TestWrapper<Empty, Fw>;
    } else {
        pub type BaseTestWrapper<Fw> = TestWrapper<Empty, Fw>;
    }
}

impl<Fw: Framework> BaseTestWrapper<Fw> {
    pub fn with_root(root: web_sys::Element, ctx: Fw::Context) -> Self {
        Self {
            root,
            state: Empty,
            _framework_ctx: ctx,
        }
    }
}
