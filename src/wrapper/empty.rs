use wasm_bindgen::JsCast as _;

use crate::{framework::Framework, util::NodeListExt};

use super::{many::Many, single::Single, Maybe, TestWrapper, TestWrapperState};

/// The initial state for a [`TestWrapper`]: no element has been selected yet
pub struct Empty;
impl TestWrapperState for Empty {}

// MAYBE I should remove the `query_*_unchecked` methods because "unchecked" usually means it's not safe, which is not the case since they just do an assertion. The names are confusing, and letting the user make the assertion themselves is no big hassle anyway
// MAYBE querying should also be in some other sorts of wrappers, not just empty
impl<Fw: Framework> TestWrapper<Empty, Fw> {
    /// Tries to find an element by the given CSS selector
    pub fn query(&self, selector: &str) -> TestWrapper<Maybe<web_sys::Element>, Fw> {
        self.derive(|_| Maybe {
            elem: self.root.query_selector(selector).unwrap(),
            selector: selector.to_string(),
        })
    }

    /// Tries to find an element by the given CSS and tries to cast it to the expected element
    pub fn query_as<T: wasm_bindgen::JsCast>(&self, selector: &str) -> TestWrapper<Maybe<T>, Fw> {
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
    pub fn query_unchecked(&self, selector: &str) -> TestWrapper<Single<web_sys::Element>, Fw> {
        self.query(selector).assert_exists()
    }

    /// Tries to find an element by the given CSS selector, panics if the element does not exist
    pub fn query_as_unchecked<T: wasm_bindgen::JsCast>(
        &self,
        selector: &str,
    ) -> TestWrapper<Single<T>, Fw> {
        self.query_as(selector).assert_exists()
    }

    /// Finds all elements that match the given CSS selector.
    pub fn query_all(&self, selector: &str) -> TestWrapper<Many<web_sys::Element>, Fw> {
        self.derive(|_| Many {
            elems: self
                .root
                .query_selector_all(selector)
                .expect("couldn't select nodes")
                .to_elem_vec(),
        })
    }

    /// Finds al elements that match the given CSS selector and tries to cast them into the expected element type
    pub fn query_all_as<T: wasm_bindgen::JsCast>(
        &self,
        selector: &str,
    ) -> TestWrapper<Many<T>, Fw> {
        self.derive(|_| Many {
            elems: self
                .root
                .query_selector_all(selector)
                .expect("couldn't select nodes")
                .to_elem_vec(),
        })
    }

    // MAYBE find by text should be somewhere else
    /// Tries to find an element that contains exactly the given test
    ///
    /// This function is recursive! Hopefully your DOM isn't infinitely deep :^)
    pub fn find_by_text_exact(&self, text: &str) -> TestWrapper<Maybe<web_sys::Element>, Fw> {
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
    ) -> TestWrapper<Maybe<Target>, Fw> {
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
            impl<Fw: Framework> TestWrapper<Empty, Fw> {
                $(
                    pub fn [<query_as_ $name>](&self, selector: &str) -> TestWrapper<Maybe<$ty>, Fw> {
                        self.query_as::<$ty>(selector)
                    }

                    pub fn [<query_as_ $name _unchecked>](&self, selector: &str) -> TestWrapper<Single<$ty>, Fw> {
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

#[cfg(all(test, target_family = "wasm"))]
mod tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    use crate::framework::leptos::mount_test;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn query_as_input() {
        let wrapper = mount_test(|| {
            view! { <input id="input" value="test" /> }
        });

        let input = wrapper.query_as_input("input").assert_exists();

        assert_eq!(input.value(), "test");
    }

    #[wasm_bindgen_test]
    fn find_by_text_exact() {
        let wrapper = mount_test(|| {
            view! {
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

    #[wasm_bindgen_test]
    fn query_all_spans() {
        let wrapper = mount_test(|| {
            view! {
                <div>
                    <span>Span 1</span>
                    <button>A button</button>
                    <span>Span 2</span>
                    <span>Span 3</span>
                </div>
            }
        });

        let result = wrapper.query_all_as::<web_sys::HtmlSpanElement>("span");

        assert_eq!(result.len(), 3);
    }
}
