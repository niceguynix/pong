mod game;
mod utils;

use utils::{document_get_element_by_id, log};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(str: String);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn load_mm() {
    let root = document_get_element_by_id("root")
        .expect("Element not found")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    utils::console_log!("Hi");
    let gamectx = game::Game::init(root);
    gamectx.start();
}







use std::rc::Rc;
use std::cell::RefCell;

struct Game2{
    x:i32
}
fn main() {
    let data = Rc::new(RefCell::new(Game2{x:1}));
    {
      let mut reference = data.borrow_mut();
      (*reference).x = 2;
    }
    println!("{}", data.borrow().x);
}