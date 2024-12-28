use std::sync::Arc;

pub mod global_click_event;
pub mod global_keyboard_event;
pub mod global_mouseup_event;
pub mod global_pointer_event;
pub mod global_resize_event;
pub mod global_scroll_event;

pub type WasmClosure<T> = Option<Arc<Box<wasm_bindgen::closure::Closure<dyn FnMut(T)>>>>;
