use std::{borrow::Borrow, fmt::Display, ops::Deref};

use wasm_bindgen::JsCast as _;

pub struct TestWrapper<State: TestWrapperState> {
    root: web_sys::Element,
    state: State,
}

pub trait TestWrapperState {}

pub struct Empty;
impl TestWrapperState for Empty {}
pub type BaseTestWrapper = TestWrapper<Empty>;

pub struct Maybe<T> {
    /// the selector used for this wrapper, useful for printing error messages
    selector: String,
    elem: Option<T>,
}
impl<T> TestWrapperState for Maybe<T> {}

pub struct Single<T>(T);
impl<T> TestWrapperState for Single<T> {}

impl TestWrapper<Empty> {
    pub fn with_root(root: web_sys::Element) -> Self {
        Self { root, state: Empty }
    }

    pub fn query_selector(
        &self,
        selector: impl Borrow<str> + Display,
    ) -> TestWrapper<Maybe<web_sys::Element>> {
        self.derive(|_| Maybe {
            elem: self.root.query_selector(selector.borrow()).unwrap(),
            selector: selector.to_string(),
        })
    }

    pub fn query_selector_as<T: wasm_bindgen::JsCast>(
        &self,
        selector: impl Borrow<str> + Display,
    ) -> TestWrapper<Maybe<T>> {
        self.derive(|_| Maybe {
            elem: self
                .root
                .query_selector(selector.borrow())
                .unwrap()
                .map(|elem| elem.unchecked_into()),
            selector: selector.to_string(),
        })
    }
}

impl<T> TestWrapper<Maybe<T>> {
    pub fn assert_exists(self) -> TestWrapper<Single<T>> {
        assert!(
            self.state.elem.is_some(),
            "element with selector `{}` does not exist",
            self.state.selector
        );
        self.map(|maybe| Single(maybe.elem.unwrap()))
    }

    pub fn assert_not_exists(self) {
        assert!(
            self.state.elem.is_none(),
            "element with selector `{}` actually exists",
            self.state.selector
        );
    }
}

impl<E: Into<web_sys::Element> + Clone> TestWrapper<Single<E>> {
    pub fn assert_text_is(&self, expected: &str) {
        assert_eq!(
            self.state.0.clone().into().text_content().unwrap(),
            expected
        );
    }

    pub fn assert_text_contains(&self, expected: &str) {
        assert!(self
            .state
            .0
            .clone()
            .into()
            .text_content()
            .unwrap()
            .contains(expected));
    }

    // TODO can I DRY these three?
    pub fn next_elem(&self) -> TestWrapper<Maybe<web_sys::Element>> {
        self.derive(|state| {
            let state_elem: web_sys::Element = state.0.clone().into();
            Maybe {
                elem: state_elem.next_element_sibling(),
                selector: String::from(""), // TODO: what do I put here?
            }
        })
    }

    pub fn prev_elem(&self) -> TestWrapper<Maybe<web_sys::Element>> {
        self.derive(|state| {
            let state_elem: web_sys::Element = state.0.clone().into();
            Maybe {
                elem: state_elem.previous_element_sibling(),
                selector: String::from(""), // TODO: what do I put here?
            }
        })
    }

    pub fn parent(&self) -> TestWrapper<Maybe<web_sys::Element>> {
        self.derive(|state| {
            let state_elem: web_sys::Element = state.0.clone().into();
            Maybe {
                elem: state_elem.parent_element(),
                selector: String::from(""), // TODO: what do I put here?
            }
        })
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
        target.dispatch_event(&super::change_evt()).unwrap();
    }
}

impl<T> Deref for TestWrapper<Single<T>> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state.0
    }
}

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use leptos::view;
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen_test::*;

    use crate::mount_test;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn assert_exists() {
        let wrapper = mount_test(|| {
            view! { <span id="existent">this exists</span> }
        });

        wrapper.query_selector("#existent").assert_exists();
        wrapper.query_selector("#non-existent").assert_not_exists();
    }

    #[wasm_bindgen_test]
    fn assert_text() {
        let wrapper = mount_test(|| {
            view! { <span id="existent">this exists</span> }
        });

        wrapper
            .query_selector("#existent")
            .assert_exists()
            .assert_text_is("this exists");

        wrapper
            .query_selector("#existent")
            .assert_exists()
            .assert_text_contains("exists");
    }

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
            .query_selector_as::<web_sys::HtmlSelectElement>("select")
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
            .query_selector_as::<web_sys::HtmlSelectElement>("select")
            .assert_exists()
            .select_opt("4");
    }

    #[should_panic(expected = "element with selector `#nonexistent` does not exist")]
    #[wasm_bindgen_test]
    fn assert_exist_panics() {
        let wrapper = mount_test(|| {
            view! { <span id="existent">This exists</span> }
        });

        wrapper.query_selector("#nonexistent").assert_exists();
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
            .query_selector_as::<web_sys::HtmlInputElement>("input")
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
