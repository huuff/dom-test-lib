use wasm_bindgen::JsCast;

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
pub impl web_sys::HtmlSelectElement {
    /// Selects an option by value and ensures the change is appropriately propagated.
    ///
    /// panics if the option doesn't exist
    fn select_option(&self, val: &str) {
        let opts = self
            .query_selector_all("option")
            .unwrap()
            .to_elem_vec::<web_sys::HtmlOptionElement>();

        if !opts.iter().any(|opt| opt.value() == val) {
            panic!("option with value `{val}` not found");
        }

        self.set_value(val);
        let event_init = web_sys::EventInit::new();
        event_init.set_bubbles(true);
        self.dispatch_event(
            &web_sys::Event::new_with_event_init_dict("change", &event_init).unwrap(),
        )
        .unwrap();
    }
}

#[extend::ext(name = NodeListExt)]
pub impl web_sys::NodeList {
    fn to_elem_vec<Elem: wasm_bindgen::JsCast>(&self) -> Vec<Elem> {
        let mut res = Vec::with_capacity(self.length() as usize);

        for i in 0..self.length() {
            let elem = self.get(i).unwrap().unchecked_into::<Elem>();
            res.push(elem);
        }

        res
    }
}

#[macro_export]
macro_rules! dom {
    ($wrapper:ident, with $selector:literal) => {
        $wrapper.query_selector($selector).unwrap().unwrap()
    };
    ($wrapper:ident, with $selector:literal as <$elem:ident>) => {
        paste::paste! {
            $wrapper.query_selector($selector).unwrap().unwrap().unchecked_into::<web_sys::[<Html $elem:camel Element>]>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

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

        let select = dom!(test_wrapper, with "select" as <select>);
        assert_eq!(select.value(), "");

        select.select_option("2");

        assert_eq!(select.value(), "2");
    }

    #[should_panic]
    #[wasm_bindgen_test]
    fn panics_on_not_found() {
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

        let select = dom!(test_wrapper, with "select" as <select>);

        select.select_option("4");
    }
}
