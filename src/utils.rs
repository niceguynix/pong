use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub use console_log;

use crate::game;
use crate::game::Game;

pub fn document_get_element_by_id(id: &str) -> Option<web_sys::HtmlElement> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let val = document
        .get_element_by_id(id)?
        .dyn_into::<web_sys::HtmlElement>()
        .expect("not a valid html element");
    Some(val)
}

pub fn animate_limited<'a>(
    mut draw_frame: impl FnMut(&mut Game) + 'static,
    mut game: game::Game,
    max_fps: i32,
) {
    // Based on:
    // https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html#srclibrs

    // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    let animate_cb = Rc::new(RefCell::new(None));
    let animate_cb2 = animate_cb.clone();

    let timeout_cb = Rc::new(RefCell::new(None));
    let timeout_cb2 = timeout_cb.clone();

    let w = window();
    *timeout_cb2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        request_animation_frame(&w, animate_cb.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let w2 = window();
    *animate_cb2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw_frame(&mut game);
        set_timeout(&w2, timeout_cb.borrow().as_ref().unwrap(), 1000 / max_fps);
    }) as Box<dyn FnMut()>));

    request_animation_frame(&window(), animate_cb2.borrow().as_ref().unwrap());
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) -> i32 {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

fn set_timeout(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}

pub fn KeyBoardListener(callback: Closure<dyn FnMut(web_sys::KeyboardEvent)>) {
    let canvas = document_get_element_by_id("root")
        .expect("Canvas not found")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    //    canvas.add_event_listener_with_callback("onkeydown",callback.as_ref().unchecked_ref()).unwrap();
    window()
        .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
        .unwrap();
    callback.forget();
}
