use wasm_bindgen::JsCast as _;

use super::{single::Single, Maybe, TestWrapper, TestWrapperState};

/// The initial state for a [`TestWrapper`]: no element has been selected yet
pub struct Empty;
impl TestWrapperState for Empty {}

impl TestWrapper<Empty> {
    pub fn with_root(root: web_sys::Element) -> Self {
        Self { root, state: Empty }
    }

    /// Tries to find an element by the given CSS selector
    pub fn query(&self, selector: &str) -> TestWrapper<Maybe<web_sys::Element>> {
        self.derive(|_| Maybe {
            elem: self.root.query_selector(selector).unwrap(),
            selector: selector.to_string(),
        })
    }

    /// Tries to find an element by the given CSS and tries to cast it to the expected element
    pub fn query_as<T: wasm_bindgen::JsCast>(&self, selector: &str) -> TestWrapper<Maybe<T>> {
        self.derive(|_| Maybe {
            elem: self
                .root
                .query_selector(selector)
                .unwrap()
                .map(|elem| elem.unchecked_into()),
            selector: selector.to_string(),
        })
    }

    /// Tries to find an element by the given CSS selector, panics if the element does not exist
    pub fn query_unchecked(&self, selector: &str) -> TestWrapper<Single<web_sys::Element>> {
        self.query(selector).assert_exists()
    }

    /// Tries to find an element by the given CSS selector, panics if the element does not exist
    pub fn query_as_unchecked<T: wasm_bindgen::JsCast>(
        &self,
        selector: &str,
    ) -> TestWrapper<Single<T>> {
        self.query_as(selector).assert_exists()
    }
}

// MAYBE docstrings?
macro_rules! impl_query_as {
    ($($name:ident => $ty:path),+ $(,)?) => {
        paste::paste! {
            impl TestWrapper<Empty> {
                $(
                    pub fn [<query_as_ $name>](&self, selector: &str) -> TestWrapper<Maybe<$ty>> {
                        self.query_as::<$ty>(selector)
                    }

                    pub fn [<query_as_ $name _unchecked>](&self, selector: &str) -> TestWrapper<Single<$ty>> {
                        self.query_as_unchecked::<$ty>(selector)
                    }
                )+
            }
        }
    };
}

impl_query_as!(
    select => web_sys::HtmlSelectElement,
    input => web_sys::HtmlInputElement,
    button => web_sys::HtmlButtonElement,
    label => web_sys::HtmlLabelElement,
);

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use crate::leptos::mount_test;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn query_as_input() {
        let wrapper = mount_test(|| {
            leptos::view! { <input id="input" value="test" /> }
        });

        let input = wrapper.query_as_input("input").assert_exists();

        assert_eq!(input.value(), "test");
    }
}
