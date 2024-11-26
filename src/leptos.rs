use super::BaseTestWrapper;

/// Mounts a view into a new element on the dom and returns a [`BaseTestWrapper`] for working with it
pub fn mount_test<F, V>(f: F) -> BaseTestWrapper
where
    F: FnOnce() -> V + 'static,
    V: leptos::IntoView,
{
    use wasm_bindgen::JsCast as _;

    let document = leptos::document();
    let test_root_node = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_root_node);

    leptos::mount_to(test_root_node.clone().unchecked_into(), f);

    BaseTestWrapper::with_root(test_root_node)
}

/// Really the same as mount_test, but also adds an `I18nContextProvider` from the
/// caller crate.
#[expect(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! mount_i18n_test {
    ($view:expr) => {
        $crate::leptos::mount_test(move || {
            leptos::view! {
                <crate::i18n::I18nContextProvider>
                    {$view}
                </crate::i18n::I18nContextProvider>
            }
        })
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use leptos::view;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn actually_mounts_it() {
        let test_wrapper = mount_test(|| {
            view! { <span id="mounted-span">hi</span> }
        });

        test_wrapper.query("#mounted-span").assert_exists();
    }

    // obviously this test won't even compile since I don't have leptos_i18n in this
    // crate, but I used it to kinda see whether it's ok

    // #[wasm_bindgen_test]
    // fn mount_i18_doesnt_break_spectacularly() {
    //     let test_wrapper = mount_i18n_test!(|| {
    //         view! { <span id="mounted-span">hi</span> }
    //     });

    //     test_wrapper.query("#mounted-span").assert_exists();
    // }
}
