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
