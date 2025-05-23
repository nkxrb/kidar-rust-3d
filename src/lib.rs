extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
  // 在rust中使用JS函数
  pub fn alert(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  // 通过wasm_bindgen暴露出去，可在JS中调用
  alert(&format!("Hello, {}!", name));
}