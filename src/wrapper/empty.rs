use wasm_bindgen::JsCast as _;

use super::{Maybe, TestWrapper, TestWrapperState};

/// The initial state for a [`TestWrapper`]: no element has been selected yet
pub struct Empty;
impl TestWrapperState for Empty {}

impl TestWrapper<Empty> {
    pub fn with_root(root: web_sys::Element) -> Self {
        Self { root, state: Empty }
    }

    pub fn query(&self, selector: &str) -> TestWrapper<Maybe<web_sys::Element>> {
        self.derive(|_| Maybe {
            elem: self.root.query_selector(selector).unwrap(),
            selector: selector.to_string(),
        })
    }

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
}

macro_rules! impl_query_as {
    ($($name:ident => $ty:path),+ $(,)?) => {
        paste::paste! {
            impl TestWrapper<Empty> {
                $(
                    pub fn [<query_as_ $name>](&self, selector: &str) -> TestWrapper<Maybe<$ty>> {
                        self.query_as::<$ty>(selector)
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

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn query_as_input() {
        let wrapper = crate::mount_test(|| {
            leptos::view! { <input id="input" value="test" /> }
        });

        let input = wrapper.query_as_input("input").assert_exists();

        assert_eq!(input.value(), "test");
    }
}
