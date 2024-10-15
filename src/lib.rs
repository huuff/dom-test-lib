/// Awaits a small amount of time so hopefully effects will have run.
///
/// I think this isn't necessary for leptos 0.7 since it exposes
/// a `tick()` fn, but it seems necessary for leptos 0.6 since I
/// copied it off one of their examples.
pub async fn next_tick() {
    gloo_timers::future::TimeoutFuture::new(25).await;
}

/// Creates, mounts into the document and returns a new element that can be used for a test
///
/// Using this element we can add any test elements to it without contaminating the global document
/// and keep the test self-enclosed.
pub fn mount_test_wrapper() -> web_sys::Element {
    let document = leptos::document();
    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);
    test_wrapper
}

#[extend::ext(name = HtmlSelectElementExt)]
impl web_sys::HtmlSelectElement {
    /// Selects an option by value and ensures the change is appropriately propagated.
    ///
    /// panics if the option doesn't exist
    fn select_option(&self, val: &str) {
        self.set_value(val);
        let event_init = web_sys::EventInit::new();
        event_init.set_bubbles(true);
        self.dispatch_event(
            &web_sys::Event::new_with_event_init_dict("change", &event_init).unwrap(),
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    use web_sys::HtmlSelectElement;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn selects_option() {
        let test_wrapper = mount_test_wrapper();

        mount_to(test_wrapper.clone().unchecked_into(), || {
            view! {
                <select>
                    <option value="">none</option>
                    <option value="1">first</option>
                    <option value="2">second</option>
                    <option value="3">third</option>
                </select>
            }
        });

        let select = test_wrapper
            .query_selector("select")
            .unwrap()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>();
        assert_eq!(select.value(), "");

        select.select_option("2");

        assert_eq!(select.value(), "2");
    }
}
