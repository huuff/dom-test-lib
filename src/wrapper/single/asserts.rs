use crate::{framework::Framework, wrapper::TestWrapper};

use super::Single;

impl<E: Into<web_sys::Element> + Clone, Fw: Framework> TestWrapper<Single<E>, Fw> {
    pub fn assert_text_is(&self, expected: &str) -> &Self {
        assert_eq!(
            self.state.0.clone().into().text_content().unwrap(),
            expected
        );
        self
    }

    pub fn assert_text_contains(&self, expected: &str) -> &Self {
        assert!(self
            .state
            .0
            .clone()
            .into()
            .text_content()
            .unwrap()
            .contains(expected));
        self
    }

    pub fn assert_class_contains(&self, expected: &str) -> &Self {
        let state_elem: web_sys::Element = self.state.0.clone().into();
        let classes = state_elem.get_attribute("class").unwrap_or_default();
        assert!(classes.contains(expected));
        self
    }

    pub fn assert_class_not_contains(&self, expected: &str) -> &Self {
        let state_elem: web_sys::Element = self.state.0.clone().into();
        let classes = state_elem.get_attribute("class").unwrap_or_default();
        assert!(classes.contains(expected));
        self
    }
}

#[cfg(all(test, target_family = "wasm"))]
mod tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    use crate::framework::leptos::mount_test;

    wasm_bindgen_test_configure!(run_in_browser);

    // TODO test class asserts
    #[wasm_bindgen_test]
    fn assert_text() {
        let wrapper = mount_test(|| {
            view! { <span id="existent">this exists</span> }
        });

        wrapper
            .query("#existent")
            .assert_exists()
            .assert_text_is("this exists");

        wrapper
            .query("#existent")
            .assert_exists()
            .assert_text_contains("exists");
    }
}
