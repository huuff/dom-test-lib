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
