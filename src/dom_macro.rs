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
        assert!($wrapper.query_selector($selector).unwrap().is_some())
    };
    ($wrapper:ident, with $selector:literal assert not exists) => {
        assert!($wrapper.query_selector($selector).unwrap().is_none())
    };
}
