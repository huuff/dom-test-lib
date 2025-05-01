use web_sys::HtmlElement;

use crate::{change_evt, framework::Framework, wrapper::TestWrapper};

use super::Single;

impl<Fw: Framework> TestWrapper<Single<web_sys::HtmlInputElement>, Fw> {
    /// Sets the value of this input and dispatches `input` and `change` events
    pub async fn change_value(&self, new_val: &str) -> &Self {
        let target = &self.state.0;
        target.set_value(new_val);
        target.dispatch_event(&crate::change_evt()).unwrap();
        target.dispatch_event(&crate::input_evt()).unwrap();

        #[cfg(feature = "leptos")]
        leptos::task::tick().await;

        self
    }

    // MAYBE should be in "asserts"?
    pub fn assert_value_is(&self, expected: impl AsRef<str>) {
        assert_eq!(self.state.0.value(), expected.as_ref());
    }
}

impl<Fw: Framework> TestWrapper<Single<web_sys::HtmlSelectElement>, Fw> {
    /// Selects an option by value and ensures the change is appropriately propagated.
    ///
    /// panics if the option doesn't exist
    pub async fn select_opt(&self, val: &str) -> &Self {
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

        #[cfg(feature = "leptos")]
        leptos::task::tick().await;

        self
    }
}

impl<Fw: Framework, Elem: AsRef<HtmlElement>> TestWrapper<Single<Elem>, Fw> {
    pub async fn click(&self) -> &Self {
        let target: &HtmlElement = self.state.0.as_ref();

        target.click();

        #[cfg(feature = "leptos")]
        leptos::task::tick().await;

        self
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::framework::leptos::mount_test;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test(unsupported = tokio::test)]
    #[cfg_attr(not(target_family = "wasm"), ignore)]
    async fn selects_option() {
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

        select.select_opt("2").await;

        assert_eq!(select.value(), "2");
    }

    #[wasm_bindgen_test(unsupported = tokio::test)]
    #[cfg_attr(not(target_family = "wasm"), ignore)]
    async fn select_panics_on_not_found() {
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
            .select_opt("4")
            .await;
    }

    #[wasm_bindgen_test(unsupported = tokio::test)]
    #[cfg_attr(not(target_family = "wasm"), ignore)]
    async fn change_value() {
        // ARRANGE
        let change_called = Arc::new(Mutex::new(String::default()));
        let input_called = Arc::new(Mutex::new(String::default()));

        let change_called_copy = change_called.clone();
        let input_called_copy = input_called.clone();
        let wrapper = mount_test(|| {
            view! {
                <input
                    id="test"
                    type="text"
                    on:input=move |evt| {
                        let mut input_called = input_called_copy.lock().unwrap();
                        *input_called = event_target_value(&evt);
                    }
                    on:change=move |evt| {
                        let mut change_called = change_called_copy.lock().unwrap();
                        *change_called = event_target_value(&evt);
                    }
                />
            }
        });

        let input = wrapper
            .query_as::<web_sys::HtmlInputElement>("input")
            .assert_exists();

        // ACT
        input.change_value("newvalue").await;

        // ASSERT
        input.assert_value_is("newvalue");
        assert_eq!(*input_called.lock().unwrap(), "newvalue");
        assert_eq!(*change_called.lock().unwrap(), "newvalue");
    }

    #[wasm_bindgen_test(unsupported = tokio::test)]
    #[cfg_attr(not(target_family = "wasm"), ignore)]
    async fn click_clicks() {
        use std::sync::atomic::{AtomicBool, Ordering};

        // ARRANGE
        let clicked = Arc::new(AtomicBool::new(false));

        let clicked_clone = Arc::clone(&clicked);
        let wrapper = mount_test(|| {
            view! {
                <button
                    type="button"
                    on:click=move |_| clicked_clone.store(true, Ordering::Release)
                >
                    click me
                </button>
            }
        });

        assert!(
            !clicked.load(Ordering::Acquire),
            "sanity check failed: state is clicked before click event"
        );

        // ACT
        wrapper
            .query_as::<web_sys::HtmlButtonElement>("button")
            .assert_exists()
            .click()
            .await;

        // ASSERT
        assert!(clicked.load(Ordering::Acquire));
    }
}
