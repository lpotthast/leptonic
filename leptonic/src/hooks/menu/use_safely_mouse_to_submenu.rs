// Upstream: react-aria/src/menu/useSafelyMouseToSubmenu.ts @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/SubMenuTrigger.test.tsx @ 99e6102368
use leptos::prelude::*;

use crate::CapturedElement;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - The submenu's rectangle is read on every (throttled) pointer move instead of being cached and
//   updated by a resize observer: as cheap, and never stale (e.g. after the submenu flipped).
//
// =============================================================================

/// Input of [`use_safely_mouse_to_submenu`].
#[derive(Debug, Clone, Copy)]
pub struct UseSafelyMouseToSubmenuInput {
    /// The menu containing the submenu trigger.
    pub menu: CapturedElement,
    /// The submenu.
    pub submenu: CapturedElement,
    pub is_open: Signal<bool>,
    pub is_disabled: Signal<bool>,
}

/// Lets the pointer travel from a submenu trigger to its open submenu across other items of the
/// menu: while it moves towards the submenu, the menu ignores pointer events (so hovering another
/// item doesn't close the submenu). Once the pointer rests, the item under it gets hovered again.
pub fn use_safely_mouse_to_submenu(input: UseSafelyMouseToSubmenuInput) {
    #[cfg(feature = "ssr")]
    {
        let _ = input;
    }
    #[cfg(not(feature = "ssr"))]
    client::use_safely_mouse_to_submenu(input);
}

#[cfg(not(feature = "ssr"))]
mod client {
    use std::time::Duration;

    use leptos::{ev, prelude::*};
    use leptos_use::use_window;
    use send_wrapper::SendWrapper;
    use wasm_bindgen::JsCast;
    use web_sys::{DomRect, PointerEvent};

    use super::UseSafelyMouseToSubmenuInput;
    use crate::{
        hooks::focus::use_focus_visible::{Modality, use_interaction_modality},
        utils::{
            event_listeners::{Listener, listen_to},
            pointer_type::PointerType,
        },
    };

    /// Movements away from the submenu tolerated before the menu takes pointer events again.
    const ALLOWED_INVALID_MOVEMENTS: u32 = 2;
    /// The minimum time between two processed pointer moves.
    const THROTTLE_TIME: f64 = 50.0;
    /// When the pointer stops moving towards the submenu for this long, the menu takes pointer
    /// events again.
    const TIMEOUT_TIME: Duration = Duration::from_millis(1000);
    /// Widens the angle towards the submenu (15°).
    const ANGLE_PADDING: f64 = std::f64::consts::PI / 12.0;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Side {
        Left,
        Right,
    }

    /// The tracking of one pointer journey, reset when the pointer leaves the menu or rests.
    struct Tracking {
        previous: Option<(f64, f64)>,
        last_processed: f64,
        side: Option<Side>,
        movements_towards_submenu: u32,
        timeout: Option<TimeoutHandle>,
        auto_close_timeout: Option<TimeoutHandle>,
    }

    impl Tracking {
        fn new() -> Self {
            Self {
                previous: None,
                last_processed: 0.0,
                side: None,
                movements_towards_submenu: ALLOWED_INVALID_MOVEMENTS,
                timeout: None,
                auto_close_timeout: None,
            }
        }

        fn clear_timeouts(&mut self) {
            if let Some(timeout) = self.timeout.take() {
                timeout.clear();
            }
            if let Some(timeout) = self.auto_close_timeout.take() {
                timeout.clear();
            }
        }
    }

    fn contains_point(rect: &DomRect, x: f64, y: f64) -> bool {
        x >= rect.left() && x <= rect.right() && y >= rect.top() && y <= rect.bottom()
    }

    pub(super) fn use_safely_mouse_to_submenu(input: UseSafelyMouseToSubmenuInput) {
        let UseSafelyMouseToSubmenuInput {
            menu,
            submenu,
            is_open,
            is_disabled,
        } = input;
        let modality = use_interaction_modality();
        let prevent_pointer_events = RwSignal::new(false);
        let tracking: StoredValue<SendWrapper<std::cell::RefCell<Tracking>>> =
            StoredValue::new(SendWrapper::new(std::cell::RefCell::new(Tracking::new())));
        let reset = move || {
            prevent_pointer_events.set(false);
            tracking.with_value(|t| {
                let mut t = t.borrow_mut();
                t.movements_towards_submenu = ALLOWED_INVALID_MOVEMENTS;
                t.previous = None;
            });
        };

        // While the pointer heads for the submenu, the menu doesn't take pointer events.
        Effect::new(move || {
            let prevent = prevent_pointer_events.get();
            if let Some(menu) = menu.get()
                && let Some(menu) = menu.dyn_ref::<web_sys::HtmlElement>()
            {
                let style = menu.style();
                let _ = if prevent {
                    style.set_property("pointer-events", "none")
                } else {
                    style.remove_property("pointer-events").map(|_| ())
                };
            }
        });

        let listeners: StoredValue<Vec<SendWrapper<Listener>>> = StoredValue::new(Vec::new());
        let stop = move || {
            listeners.update_value(Vec::clear);
            tracking.with_value(|t| {
                let mut t = t.borrow_mut();
                t.clear_timeouts();
                t.movements_towards_submenu = ALLOWED_INVALID_MOVEMENTS;
                // The submenu may open on the other side next time.
                t.side = None;
            });
        };
        on_cleanup(stop);

        Effect::new(move || {
            stop();
            let (Some(menu_el), Some(_)) = (menu.get(), submenu.get()) else {
                reset();
                return;
            };
            if is_disabled.get() || !is_open.get() || modality.get() != Some(Modality::Pointer) {
                reset();
                return;
            }
            let Some(window) = use_window().as_ref().cloned() else {
                return;
            };
            let menu_el = (*menu_el).clone();
            let on_pointer_move = move |e: PointerEvent| {
                if matches!(PointerType::of(&e), PointerType::Touch | PointerType::Pen) {
                    return;
                }
                let Some(submenu_rect) = submenu
                    .get_untracked()
                    .map(|el| el.get_bounding_client_rect())
                else {
                    return;
                };
                let now = js_sys::Date::now();
                let (x, y) = (e.client_x(), e.client_y());
                let towards = tracking.with_value(|t| {
                    let mut t = t.borrow_mut();
                    // Throttle.
                    if now - t.last_processed < THROTTLE_TIME {
                        return None;
                    }
                    t.clear_timeouts();
                    let Some((previous_x, previous_y)) = t.previous else {
                        t.previous = Some((x, y));
                        return None;
                    };
                    let side = *t.side.get_or_insert(if x > submenu_rect.right() {
                        Side::Left
                    } else {
                        Side::Right
                    });
                    Some((previous_x, previous_y, side))
                });
                let Some((previous_x, previous_y, side)) = towards else {
                    return;
                };

                // Outside of the parent menu.
                if !contains_point(&menu_el.get_bounding_client_rect(), x, y) {
                    reset();
                    return;
                }

                // Whether the pointer moves towards the submenu: the angle of its movement lies
                // between the angles from its previous position to the submenu's top and bottom.
                let to_submenu_x = match side {
                    Side::Right => submenu_rect.left() - previous_x,
                    Side::Left => previous_x - submenu_rect.right(),
                };
                let angle_top =
                    (previous_y - submenu_rect.top()).atan2(to_submenu_x) + ANGLE_PADDING;
                let angle_bottom =
                    (previous_y - submenu_rect.bottom()).atan2(to_submenu_x) - ANGLE_PADDING;
                let delta_x = match side {
                    Side::Left => -(x - previous_x),
                    Side::Right => x - previous_x,
                };
                let angle_pointer = (previous_y - y).atan2(delta_x);
                let is_moving_towards = angle_pointer < angle_top && angle_pointer > angle_bottom;

                let movements = tracking.with_value(|t| {
                    let mut t = t.borrow_mut();
                    t.movements_towards_submenu = if is_moving_towards {
                        (t.movements_towards_submenu + 1).min(ALLOWED_INVALID_MOVEMENTS)
                    } else {
                        t.movements_towards_submenu.saturating_sub(1)
                    };
                    t.last_processed = now;
                    t.previous = Some((x, y));
                    t.movements_towards_submenu
                });
                prevent_pointer_events.set(movements >= ALLOWED_INVALID_MOVEMENTS);

                // When the pointer rests on its way, the menu takes pointer events again and the
                // element under the pointer gets hovered (it may close the submenu).
                if is_moving_towards {
                    let menu_el = menu_el.clone();
                    let timeout = set_timeout_with_handle(
                        move || {
                            reset();
                            let menu_el = menu_el.clone();
                            let auto_close = set_timeout_with_handle(
                                move || {
                                    let Some(document) = menu_el.owner_document() else {
                                        return;
                                    };
                                    #[allow(clippy::cast_possible_truncation)]
                                    let target = document.element_from_point(x as f32, y as f32);
                                    if let Some(target) = target
                                        && menu_el.contains(Some(&target))
                                    {
                                        let init = web_sys::PointerEventInit::new();
                                        init.set_bubbles(true);
                                        init.set_cancelable(true);
                                        if let Ok(event) = PointerEvent::new_with_event_init_dict(
                                            "pointerover",
                                            &init,
                                        ) {
                                            let _ = target.dispatch_event(&event);
                                        }
                                    }
                                },
                                Duration::from_millis(100),
                            )
                            .ok();
                            tracking.with_value(|t| t.borrow_mut().auto_close_timeout = auto_close);
                        },
                        TIMEOUT_TIME,
                    )
                    .ok();
                    tracking.with_value(|t| t.borrow_mut().timeout = timeout);
                }
            };
            let on_pointer_down = move |e: PointerEvent| {
                // Pressing while the menu ignores pointer events would move focus to whatever is
                // behind it.
                if prevent_pointer_events.get_untracked() {
                    e.prevent_default();
                }
            };
            listeners.update_value(|listeners| {
                listeners.push(SendWrapper::new(listen_to(
                    &window,
                    ev::pointermove,
                    false,
                    on_pointer_move,
                )));
                listeners.push(SendWrapper::new(listen_to(
                    &window,
                    ev::pointerdown,
                    true,
                    on_pointer_down,
                )));
            });
        });
    }
}
