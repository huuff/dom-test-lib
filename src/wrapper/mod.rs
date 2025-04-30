mod constructor;
mod empty;
mod many;
mod maybe;
mod single;

use crate::framework::Framework;
use maybe::Maybe;

pub use constructor::BaseTestWrapper;

pub struct TestWrapper<State: TestWrapperState, Fw: Framework> {
    root: web_sys::Element,
    state: State,
    _framework_ctx: Fw::Context,
}

pub trait TestWrapperState {}

impl<T: TestWrapperState, Fw: Framework> TestWrapper<T, Fw> {
    /// Creates a new [`TestWrapper`] from this one while keeping the same root node
    fn derive<S: TestWrapperState>(&self, state_fn: impl Fn(&T) -> S) -> TestWrapper<S, Fw> {
        TestWrapper {
            root: self.root.clone(),
            state: state_fn(&self.state),
            _framework_ctx: self._framework_ctx.clone(),
        }
    }

    /// Converts this [`TestWrapper`] into a new one while keeping the root node
    fn map<S: TestWrapperState>(self, state_fn: impl FnOnce(T) -> S) -> TestWrapper<S, Fw> {
        TestWrapper {
            root: self.root,
            state: state_fn(self.state),
            _framework_ctx: self._framework_ctx,
        }
    }
}
