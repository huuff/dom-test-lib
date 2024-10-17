#[macro_export]
macro_rules! dom {
    ($wrapper:ident, with $selector:literal) => {
        $wrapper.query_selector($selector).unwrap().unwrap()
    };
    ($wrapper:ident, with $selector:literal as <$elem:ident>) => {
        paste::paste! {
            $wrapper.query_selector($selector).unwrap().unwrap().unchecked_into::<web_sys::[<Html $elem:camel Element>]>()
        }
    };
    ($wrapper:ident, with $selector:literal assert exists) => {
        assert!($wrapper.query_selector($selector).unwrap().is_some());
    };
    ($wrapper:ident, with $selector:literal assert not exists) => {
        assert!($wrapper.query_selector($selector).unwrap().is_none());
    };
    ($wrapper:ident, with $selector:literal assert text is $text:literal) => {
        assert_eq!($crate::dom!($wrapper, with $selector).text_content().unwrap(), $text);
    };
    ($wrapper:ident, with $selector:literal assert text contains $text:literal) => {
        assert!($crate::dom!($wrapper, with $selector).text_content().unwrap().contains($text));
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
        let test_wrapper = mount_test(|| {
            view! { <span id="existent">this exists</span> }
        });

        dom!(test_wrapper, with "#existent" assert exists);
        dom!(test_wrapper, with "#non-existent" assert not exists);
    }

    #[wasm_bindgen_test]
    fn assert_text() {
        let test_wrapper = mount_test(|| {
            view! { <span id="existent">this exists</span> }
        });

        dom!(test_wrapper, with "#existent" assert text is "this exists");
        dom!(test_wrapper, with "#existent" assert text contains "exists");
    }
}
