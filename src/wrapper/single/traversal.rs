use crate::{
    framework::Framework,
    wrapper::{maybe::Maybe, TestWrapper},
};

use super::Single;

// TODO test
impl<E: Into<web_sys::Element> + Clone, Fw: Framework> TestWrapper<Single<E>, Fw> {
    // TODO can I DRY these three?
    pub fn next_elem(&self) -> TestWrapper<Maybe<web_sys::Element>, Fw> {
        self.derive(|state| {
            let state_elem: web_sys::Element = state.0.clone().into();
            Maybe {
                elem: state_elem.next_element_sibling(),
                selector: String::from(""), // TODO: what do I put here?
            }
        })
    }

    pub fn prev_elem(&self) -> TestWrapper<Maybe<web_sys::Element>, Fw> {
        self.derive(|state| {
            let state_elem: web_sys::Element = state.0.clone().into();
            Maybe {
                elem: state_elem.previous_element_sibling(),
                selector: String::from(""), // TODO: what do I put here?
            }
        })
    }

    // TODO: shouldn't this be `parent_elem`?
    pub fn parent(&self) -> TestWrapper<Maybe<web_sys::Element>, Fw> {
        self.derive(|state| {
            let state_elem: web_sys::Element = state.0.clone().into();
            Maybe {
                elem: state_elem.parent_element(),
                selector: String::from(""), // TODO: what do I put here?
            }
        })
    }
}
