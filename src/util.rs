// TODO: A specific module for this
#[extend::ext(name = NodeListExt)]
pub impl web_sys::NodeList {
    fn to_elem_vec<Elem: wasm_bindgen::JsCast>(&self) -> Vec<Elem> {
        use wasm_bindgen::JsCast as _;

        let mut res = Vec::with_capacity(self.length() as usize);

        for i in 0..self.length() {
            let elem = self.get(i).unwrap().dyn_into::<Elem>().unwrap_or_else(|_| {
                panic!(
                    "some node was not an instance of {}",
                    std::any::type_name::<Elem>()
                )
            });
            res.push(elem);
        }

        res
    }

    fn into_iterator(self) -> NodeListIter {
        NodeListIter {
            nodes: self,
            idx: 0,
        }
    }
}

pub struct NodeListIter {
    nodes: web_sys::NodeList,
    idx: usize,
}

impl Iterator for NodeListIter {
    type Item = web_sys::Node;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.nodes.get(self.idx.try_into().unwrap());
        self.idx += 1;
        res
    }
}
