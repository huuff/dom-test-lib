use super::{single::Single, TestWrapper, TestWrapperState};

/// A wrapper in an indeterminate state: it may hold an element or it may not,
/// you have to assert on it to pass to a determinate state
pub struct Maybe<T> {
    /// The selector used for this wrapper, useful for printing error messages
    pub(super) selector: String,
    pub(super) elem: Option<T>,
}
impl<T> TestWrapperState for Maybe<T> {}

impl<T> TestWrapper<Maybe<T>> {
    /// Run an assertion that this element exists, and also promote this to
    /// a [`TestWrapper`] that ensures it's element exists
    pub fn assert_exists(self) -> TestWrapper<Single<T>> {
        assert!(
            self.state.elem.is_some(),
            "element with selector `{}` does not exist",
            self.state.selector
        );
        self.map(|maybe| Single(maybe.elem.unwrap()))
    }

    /// Runs an assertion that this element doesn't exist and consume the wrapper.
    pub fn assert_not_exists(self) {
        assert!(
            self.state.elem.is_none(),
            "element with selector `{}` actually exists",
            self.state.selector
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::leptos::mount_test;
    use leptos::view;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[should_panic(expected = "element with selector `#nonexistent` does not exist")]
    #[wasm_bindgen_test]
    fn assert_exist_panics() {
        let wrapper = mount_test(|| {
            view! { <span id="existent">This exists</span> }
        });

        wrapper.query("#nonexistent").assert_exists();
    }

    #[wasm_bindgen_test]
    fn assert_exists() {
        let wrapper = mount_test(|| {
            view! { <span id="existent">this exists</span> }
        });

        wrapper.query("#existent").assert_exists();
        wrapper.query("#non-existent").assert_not_exists();
    }
}
