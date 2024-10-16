/// Creates, mounts a view into the document for testing. Returns an element that contains the view.
///
/// This helps keeping the view self-contained and tries to prevent interacting with other elements.
pub fn mount_test<F, V>(f: F) -> web_sys::Element
where
    F: FnOnce() -> V + 'static,
    V: leptos::IntoView,
{
    use wasm_bindgen::JsCast as _;

    let document = leptos::document();
    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    leptos::mount_to(test_wrapper.clone().unchecked_into(), f);

    test_wrapper
}

#[cfg(test)]
mod test {
    use crate::dom;

    use super::*;
    use leptos::view;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn actually_mounts_it() {
        let test_wrapper = mount_test(|| {
            view! { <span id="mounted-span">hi</span> }
        });

        dom!(test_wrapper, with "#mounted-span" assert exists);
    }
}
