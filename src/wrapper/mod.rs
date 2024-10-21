mod empty;
mod maybe;
mod single;

use maybe::Maybe;

pub type BaseTestWrapper = TestWrapper<empty::Empty>;

pub struct TestWrapper<State: TestWrapperState> {
    root: web_sys::Element,
    state: State,
}

pub trait TestWrapperState {}

impl<T: TestWrapperState> TestWrapper<T> {
    /// Creates a new [`TestWrapper`] from this one while keeping the same root node
    fn derive<S: TestWrapperState>(&self, state_fn: impl Fn(&T) -> S) -> TestWrapper<S> {
        TestWrapper {
            root: self.root.clone(),
            state: state_fn(&self.state),
        }
    }

    /// Converts this [`TestWrapper`] into a new one while keeping the root node
    fn map<S: TestWrapperState>(self, state_fn: impl FnOnce(T) -> S) -> TestWrapper<S> {
        TestWrapper {
            root: self.root,
            state: state_fn(self.state),
        }
    }
}
