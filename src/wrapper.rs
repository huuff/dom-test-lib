use std::{borrow::Borrow, fmt::Display, ops::Deref};

use wasm_bindgen::JsCast as _;

pub struct TestWrapper<State: TestWrapperState> {
    root: web_sys::Element,
    state: State,
}

pub trait TestWrapperState {}

pub struct Empty;
impl TestWrapperState for Empty {}

pub struct Maybe<T>(Option<T>);
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
        self.derive(|_| {
            Maybe(
                self.root
                    .query_selector(selector.borrow())
                    .unwrap_or_else(|_| panic!("no element found with selector `{selector}`")),
            )
        })
    }

    pub fn query_selector_as<T: wasm_bindgen::JsCast>(
        &self,
        selector: impl Borrow<str> + Display,
    ) -> TestWrapper<Maybe<T>> {
        self.derive(|_| {
            Maybe(
                self.root
                    .query_selector(selector.borrow())
                    .unwrap_or_else(|_| panic!("no element found with selector `{selector}`"))
                    .map(|elem| elem.unchecked_into()),
            )
        })
    }
}

impl<T> TestWrapper<Maybe<T>> {
    pub fn assert_exists(self) -> TestWrapper<Single<T>> {
        assert!(self.state.0.is_some());
        self.map(|maybe| Single(maybe.0.unwrap()))
    }

    pub fn assert_not_exists(self) {
        assert!(self.state.0.is_none());
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
        target.dispatch_event(&super::new_change_event()).unwrap();
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
    use leptos::view;
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

    #[should_panic]
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
}
