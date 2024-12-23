use leptos::ev::*;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct EventHandlers {
    #[builder(default, setter(strip_option))]
    pub on_key_up: Option<Box<dyn FnMut(KeyboardEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_key_down: Option<Box<dyn FnMut(KeyboardEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_click: Option<Box<dyn FnMut(MouseEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_pointer_down: Option<Box<dyn FnMut(PointerEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_pointer_up: Option<Box<dyn FnMut(PointerEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_pointer_enter: Option<Box<dyn FnMut(PointerEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_pointer_leave: Option<Box<dyn FnMut(PointerEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_focus: Option<Box<dyn FnMut(FocusEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_blur: Option<Box<dyn FnMut(FocusEvent)>>,
    #[builder(default, setter(strip_option))]
    pub on_wheel: Option<Box<dyn FnMut(WheelEvent)>>,
}

impl std::fmt::Debug for EventHandlers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandlers")
            .field("on_key_up", dbg(self.on_key_up.as_ref()))
            .field("on_key_down", dbg(self.on_key_down.as_ref()))
            .field("on_click", dbg(self.on_click.as_ref()))
            .field("on_pointer_down", dbg(self.on_pointer_down.as_ref()))
            .field("on_pointer_up", dbg(self.on_pointer_up.as_ref()))
            .field("on_pointer_enter", dbg(self.on_pointer_enter.as_ref()))
            .field("on_pointer_leave", dbg(self.on_pointer_leave.as_ref()))
            .field("on_focus", dbg(self.on_focus.as_ref()))
            .field("on_blur", dbg(self.on_blur.as_ref()))
            .field("on_wheel", dbg(self.on_wheel.as_ref()))
            .finish()
    }
}
fn dbg<E>(handler: Option<&Box<dyn FnMut(E)>>) -> &dyn std::fmt::Debug {
    handler.map(|_| &"Some(Fn)").unwrap_or(&"None")
}

impl EventHandlers {
    pub fn merge(mut self, other: EventHandlers) -> Self {
        self.on_key_up = merge(self.on_key_up, other.on_key_up);
        self.on_key_down = merge(self.on_key_down, other.on_key_down);
        self.on_click = merge(self.on_click, other.on_click);
        self.on_pointer_down = merge(self.on_pointer_down, other.on_pointer_down);
        self.on_pointer_up = merge(self.on_pointer_up, other.on_pointer_up);
        self.on_pointer_enter = merge(self.on_pointer_enter, other.on_pointer_enter);
        self.on_pointer_leave = merge(self.on_pointer_leave, other.on_pointer_leave);
        self.on_focus = merge(self.on_focus, other.on_focus);
        self.on_blur = merge(self.on_blur, other.on_blur);
        self.on_wheel = merge(self.on_wheel, other.on_wheel);
        self
    }

    pub fn merge_opt(self, other: Option<EventHandlers>) -> Self {
        if let Some(handlers) = other {
            self.merge(handlers)
        } else {
            self
        }
    }
}

fn merge<E: Clone + 'static>(
    a: Option<Box<dyn FnMut(E)>>,
    b: Option<Box<dyn FnMut(E)>>,
) -> Option<Box<dyn FnMut(E)>> {
    match (a, b) {
        (None, None) => None,
        (None, Some(f)) => Some(f),
        (Some(f), None) => Some(f),
        (Some(mut f1), Some(mut f2)) => Some(Box::new(move |e| {
            f1(e.clone());
            f2(e);
        })),
    }
}

impl IntoIterator for EventHandlers {
    type Item = EventHandlerFn;

    type IntoIter = EventHandlersIterator;

    fn into_iter(self) -> Self::IntoIter {
        EventHandlersIterator {
            handlers: [
                self.on_key_up.map(|it| EventHandlerFn::Keyup(it)),
                self.on_key_down.map(|it| EventHandlerFn::Keydown(it)),
                self.on_click.map(|it| EventHandlerFn::Click(it)),
                self.on_pointer_down
                    .map(|it| EventHandlerFn::Pointerdown(it)),
                self.on_pointer_up.map(|it| EventHandlerFn::Pointerup(it)),
                self.on_pointer_enter
                    .map(|it| EventHandlerFn::Pointerenter(it)),
                self.on_pointer_leave
                    .map(|it| EventHandlerFn::Pointerleave(it)),
                self.on_focus.map(|it| EventHandlerFn::Focus(it)),
                self.on_blur.map(|it| EventHandlerFn::Blur(it)),
                self.on_wheel.map(|it| EventHandlerFn::Wheel(it)),
            ],
            index: 0,
        }
    }
}

#[allow(missing_debug_implementations)]
pub struct EventHandlersIterator {
    handlers: [Option<EventHandlerFn>; 10],
    index: usize,
}

impl Iterator for EventHandlersIterator {
    type Item = EventHandlerFn;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.handlers.len() {
            if let Some(handler) = self.handlers[self.index].take() {
                self.index += 1;
                return Some(handler);
            } else {
                self.index += 1;
            }
        }
        None
    }
}
