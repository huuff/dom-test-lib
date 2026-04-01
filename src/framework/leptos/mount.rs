use std::sync::Arc;

use super::implementation::Leptos;
use crate::{framework::leptos::implementation::ErasedDestructor, wrapper::BaseTestWrapper};
use leptos::prelude::*;

/// Mounts a view into a new element on the dom and returns a [`BaseTestWrapper`] for working with it
pub fn mount_test<F, V>(f: F) -> BaseTestWrapper<Leptos>
where
    F: FnOnce() -> V + 'static,
    V: IntoView,
    <V as Render>::State: 'static,
{
    use wasm_bindgen::JsCast as _;

    let document = document();
    let test_root_node = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_root_node);

    let handle = mount_to(test_root_node.clone().unchecked_into(), f);

    BaseTestWrapper::with_root(
        test_root_node,
        Arc::new(handle) as Arc<dyn ErasedDestructor>,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use leptos::view;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test(unsupported = test)]
    #[cfg_attr(not(target_family = "wasm"), ignore)]
    fn actually_mounts_it() {
        let test_wrapper = mount_test(|| {
            view! { <span id="mounted-span">hi</span> }
        });

        test_wrapper.query("#mounted-span").assert_exists();
    }
}
