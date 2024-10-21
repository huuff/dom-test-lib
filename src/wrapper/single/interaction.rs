use crate::{change_evt, wrapper::TestWrapper};

use super::Single;

impl TestWrapper<Single<web_sys::HtmlInputElement>> {
    /// Sets the value of this input and dispatches `input` and `change` events
    pub fn change_value(&self, new_val: &str) -> &Self {
        let target = &self.state.0;
        target.set_value(new_val);
        target.dispatch_event(&crate::change_evt()).unwrap();
        target.dispatch_event(&crate::input_evt()).unwrap();
        self
    }

    pub fn assert_value_is(&self, expected: impl AsRef<str>) {
        assert_eq!(self.state.0.value(), expected.as_ref());
    }
}

impl TestWrapper<Single<web_sys::HtmlSelectElement>> {
    /// Selects an option by value and ensures the change is appropriately propagated.
    ///
    /// panics if the option doesn't exist
    pub fn select_opt(&self, val: &str) {
        use crate::util::NodeListExt as _;

        let target = &self.state.0;
        let opts = target
            .query_selector_all("option")
            .unwrap()
            .to_elem_vec::<web_sys::HtmlOptionElement>();

        if !opts.iter().any(|opt| opt.value() == val) {
            panic!("option with value `{val}` not found");
        }

        target.set_value(val);
        target.dispatch_event(&change_evt()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use leptos::view;
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen_test::*;

    use crate::mount_test;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn selects_option() {
        let wrapper = mount_test(|| {
            view! {
                <select>
                    <option value="">none</option>
                    <option value="1">first</option>
                    <option value="2">second</option>
                    <option value="3">third</option>
                </select>
            }
        });

        let select = wrapper
            .query_as::<web_sys::HtmlSelectElement>("select")
            .assert_exists();

        assert_eq!(select.value(), "");

        select.select_opt("2");

        assert_eq!(select.value(), "2");
    }

    #[should_panic(expected = "option with value `4` not found")]
    #[wasm_bindgen_test]
    fn select_panics_on_not_found() {
        let wrapper = mount_test(|| {
            view! {
                <select>
                    <option value="">none</option>
                    <option value="1">first</option>
                    <option value="2">second</option>
                    <option value="3">third</option>
                </select>
            }
        });

        wrapper
            .query_as::<web_sys::HtmlSelectElement>("select")
            .assert_exists()
            .select_opt("4");
    }

    #[wasm_bindgen_test]
    fn change_value() {
        use std::sync::atomic::{AtomicBool, Ordering};
        // ARRANGE
        let change_called = Arc::new(AtomicBool::new(false));
        let input_called = Arc::new(AtomicBool::new(false));

        let change_called_copy = change_called.clone();
        let change_fn = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
            change_called_copy.store(true, Ordering::Release);
        });

        let input_called_copy = input_called.clone();
        let input_fn = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
            input_called_copy.store(true, Ordering::Release);
        });

        let wrapper = mount_test(|| {
            view! { <input id="test" type="text" /> }
        });

        let input = wrapper
            .query_as::<web_sys::HtmlInputElement>("input")
            .assert_exists();

        input.set_onchange(Some(change_fn.as_ref().unchecked_ref()));
        input.set_oninput(Some(input_fn.as_ref().unchecked_ref()));

        // ACT
        input.change_value("newvalue");

        // ASSERT
        input.assert_value_is("newvalue");
        assert!(change_called.load(Ordering::Acquire));
        assert!(input_called.load(Ordering::Acquire));
    }
}
