#[extend::ext(name = NodeListExt)]
pub impl web_sys::NodeList {
    fn to_elem_vec<Elem: wasm_bindgen::JsCast>(&self) -> Vec<Elem> {
        use wasm_bindgen::JsCast as _;

        let mut res = Vec::with_capacity(self.length() as usize);

        for i in 0..self.length() {
            let elem = self.get(i).unwrap().unchecked_into::<Elem>();
            res.push(elem);
        }

        res
    }
}

/// Awaits a small amount of time so hopefully effects will have run.
///
/// I think this isn't necessary for leptos 0.7 since it exposes
/// a `tick()` fn, but it seems necessary for leptos 0.6 since I
/// copied it off one of their examples.
pub async fn next_tick() {
    gloo_timers::future::TimeoutFuture::new(25).await;
}
