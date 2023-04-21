use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn wkdfunc(s: &str);
}

#[wasm_bindgen]
pub fn wkdfunc(zeichenfolge: &str) -> r {
    let mut r = String::new();
    r = &zeichenfolge;
    return r
}
