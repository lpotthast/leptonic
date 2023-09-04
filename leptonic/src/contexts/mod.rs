use std::rc::Rc;

pub mod global_click_event;
pub mod global_keyboard_event;
pub mod global_mouseup_event;
pub mod global_resize_event;
pub mod global_scroll_event;

pub type WasmClosure<T> = Rc<Box<wasm_bindgen::closure::Closure<dyn FnMut(T)>>>;
