// Upstream: react-aria/src/utils/useViewportSize.ts @ 99e6102368
// Upstream: react-aria/test/utils/useViewportSize.ssr.test.tsx @ 99e6102368
//! The size of the visual viewport.
use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - The early update when the iOS keyboard closes (WebKit fires the visual viewport's resize only
//   after the animation; react-aria anticipates it on blur with `runAfterKeyboard`).
//
// =============================================================================

/// A viewport size, in CSS pixels.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ViewportSize {
    pub width: f64,
    pub height: f64,
}

/// The size of the visual viewport without scrollbars, following resizes (also the on-screen
/// keyboard opening) but not pinch zooming. Zero during server-side rendering and before the
/// first effect.
pub fn use_viewport_size() -> Signal<ViewportSize> {
    let size = RwSignal::new(ViewportSize::default());

    #[cfg(not(feature = "ssr"))]
    {
        use crate::utils::event_listeners::listen_to;

        let update = move || {
            let Some(window) = leptos_use::use_window().as_ref().cloned() else {
                return;
            };
            let visual_viewport = window.visual_viewport();
            // Ignore updates when zoomed.
            if visual_viewport.as_ref().is_some_and(|vv| vv.scale() > 1.0) {
                return;
            }
            let new = viewport_size(&window);
            if size.try_get_untracked().is_some_and(|size| size != new) {
                size.set(new);
            }
        };
        Effect::new(move |_| update());
        if let Some(window) = leptos_use::use_window().as_ref().cloned() {
            let target: web_sys::EventTarget = match window.visual_viewport() {
                Some(vv) => vv.into(),
                None => window.into(),
            };
            let listener = listen_to(
                &target,
                leptos::ev::resize,
                false,
                move |_: web_sys::UiEvent| {
                    update();
                },
            );
            let listener = StoredValue::new_local(Some(listener));
            on_cleanup(move || {
                let _ = listener.try_update_value(Option::take);
            });
        }
    }

    size.into()
}

/// The viewport size without the scrollbar.
#[cfg(not(feature = "ssr"))]
fn viewport_size(window: &web_sys::Window) -> ViewportSize {
    let client = window
        .document()
        .and_then(|d| d.document_element())
        .map(|el| (f64::from(el.client_width()), f64::from(el.client_height())));
    let (client_width, client_height) = client.unwrap_or_default();
    match window.visual_viewport() {
        // The natural size, unaffected by pinch zooming. The visual viewport's width may include
        // the scrollbar gutter: the document element's never does.
        Some(vv) => ViewportSize {
            width: (vv.width() * vv.scale()).min(client_width),
            height: vv.height() * vv.scale(),
        },
        None => ViewportSize {
            width: client_width,
            height: client_height,
        },
    }
}
