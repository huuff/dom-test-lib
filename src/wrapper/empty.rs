use wasm_bindgen::JsCast as _;

use crate::util::NodeListExt;

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

    // MAYBE should also be in some other sorts of wrappers, not just empty
    /// Tries to find an element that contains exactly the given test
    ///
    /// This function is recursive! Hopefully your DOM isn't infinitely deep :^)
    pub fn find_by_text_exact(&self, text: &str) -> TestWrapper<Maybe<web_sys::Element>> {
        self.derive(|_| Maybe {
            elem: recursive_find_by_text_exact(
                self.root.clone().dyn_into::<web_sys::Node>().unwrap(),
                text,
            ),
            selector: format!("<text={text}>"),
        })
    }

    pub fn find_by_text_exact_as<Target: wasm_bindgen::JsCast>(
        &self,
        text: &str,
    ) -> TestWrapper<Maybe<Target>> {
        self.derive(|_| Maybe {
            elem: recursive_find_by_text_exact::<Target>(
                self.root.clone().dyn_into::<web_sys::Node>().unwrap(),
                text,
            ),
            selector: format!("<text={text}>"),
        })
    }
}

fn recursive_find_by_text_exact<Target: wasm_bindgen::JsCast>(
    root: web_sys::Node,
    needle: &str,
) -> Option<Target> {
    let children = root.child_nodes();

    if let Ok(root_as) = root.clone().dyn_into::<Target>() {
        if children.length() == 1
            && children.get(0).unwrap().has_type::<web_sys::Text>()
            && root.text_content().is_some_and(|text| text == needle)
        {
            return Some(root_as);
        }
    }

    for child in children.into_iterator() {
        if let Some(matching_el) = recursive_find_by_text_exact(child, needle) {
            return Some(matching_el);
        }
    }

    None
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

    #[wasm_bindgen_test]
    fn find_by_text_exact() {
        let wrapper = mount_test(|| {
            leptos::view! {
                <main>
                    <p id="nontarget1">Not the target</p>
                    <div id="targetcontainer">
                        <span id="found" class="found">
                            Target 123
                        </span>
                    </div>
                    <p id="nontarget2">Not the target</p>
                </main>
            }
        });

        let target = wrapper.find_by_text_exact("Target 123").assert_exists();

        assert_eq!(target.id(), "found");
    }
}
