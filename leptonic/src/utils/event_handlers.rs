use leptos::ev::*;
use std::rc::Rc;
use typed_builder::TypedBuilder;

#[derive(Clone, TypedBuilder)]
pub struct EventHandlers {
    #[builder(default, setter(strip_option))]
    pub on_click: Option<Rc<dyn Fn(MouseEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_pointer_down: Option<Rc<dyn Fn(PointerEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_pointer_up: Option<Rc<dyn Fn(PointerEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_pointer_enter: Option<Rc<dyn Fn(PointerEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_pointer_leave: Option<Rc<dyn Fn(PointerEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_key_down: Option<Rc<dyn Fn(KeyboardEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_focus: Option<Rc<dyn Fn(FocusEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_blur: Option<Rc<dyn Fn(FocusEvent)>>,
}

impl std::fmt::Debug for EventHandlers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandlers")
            .field("on_click", dbg(self.on_click.as_ref()))
            .field("on_pointer_down", dbg(self.on_pointer_down.as_ref()))
            .field("on_pointer_up", dbg(self.on_pointer_up.as_ref()))
            .field("on_pointer_enter", dbg(self.on_pointer_enter.as_ref()))
            .field("on_pointer_leave", dbg(self.on_pointer_leave.as_ref()))
            .field("on_key_down", dbg(self.on_key_down.as_ref()))
            .field("on_focus", dbg(self.on_focus.as_ref()))
            .field("on_blur", dbg(self.on_blur.as_ref()))
            .finish()
    }
}
fn dbg<E>(handler: Option<&Rc<dyn Fn(E)>>) -> &dyn std::fmt::Debug {
    handler.map(|_| &"Some(Fn)").unwrap_or(&"None")
}

impl EventHandlers {
    pub fn new() -> Self {
        Self {
            on_click: None,
            on_pointer_down: None,
            on_pointer_up: None,
            on_pointer_enter: None,
            on_pointer_leave: None,
            on_key_down: None,
            on_focus: None,
            on_blur: None,
        }
    }

    pub fn merge(mut self, other: EventHandlers) -> Self {
        self.on_click = merge(self.on_click, other.on_click);
        self.on_pointer_down = merge(self.on_pointer_down, other.on_pointer_down);
        self.on_pointer_up = merge(self.on_pointer_up, other.on_pointer_up);
        self.on_pointer_enter = merge(self.on_pointer_enter, other.on_pointer_enter);
        self.on_pointer_leave = merge(self.on_pointer_leave, other.on_pointer_leave);
        self.on_key_down = merge(self.on_key_down, other.on_key_down);
        self.on_focus = merge(self.on_focus, other.on_focus);
        self.on_blur = merge(self.on_blur, other.on_blur);
        self
    }
}

fn merge<E: Clone + 'static>(
    a: Option<Rc<dyn Fn(E)>>,
    b: Option<Rc<dyn Fn(E)>>,
) -> Option<Rc<dyn Fn(E)>> {
    match (a, b) {
        (None, None) => None,
        (None, Some(f)) => Some(f),
        (Some(f), None) => Some(f),
        (Some(f1), Some(f2)) => Some(Rc::new(move |e| {
            f1(e.clone());
            f2(e);
        })),
    }
}
