// Upstream: react-aria/src/virtualizer/useVirtualizerItem.ts @ 99e6102368
// Upstream: react-aria/src/virtualizer/VirtualizerItem.tsx @ 99e6102368
use leptos::prelude::*;

use super::LayoutInfo;
use crate::utils::{
    CapturedElement, i18n::use_direction, locale::WritingDirection, styles::Styles,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the item wrapper's styles (react-aria's `VirtualizerItem` renders the wrapper with
//   `layoutInfoToStyle`); the caller renders the wrapper (`role="presentation"`).
//
// =============================================================================

/// Input of [`use_virtualizer_item`].
pub struct UseVirtualizerItemInput {
    pub layout_info: Signal<LayoutInfo>,
    /// The layout info of the parent view (e.g. the section), for relative positions.
    pub parent: Signal<Option<LayoutInfo>>,
    /// Receives the measured size of the item (react-aria: the virtualizer's `updateItemSize`,
    /// e.g. `VirtualizerState::update_item_size`).
    pub update_item_size: Callback<(
        crate::hooks::collections::Key,
        crate::hooks::collections::Size,
    )>,
    /// Re-measure whenever the item's children resize (variable sizes that change after the
    /// first measurement).
    pub should_observe_item_size: bool,
}

/// Return value of [`use_virtualizer_item`].
pub struct UseVirtualizerItemReturn {
    /// The wrapper's styles: absolutely positioned at its layout info.
    pub styles: Signal<Styles>,
}

/// Positions an item of a virtualized collection and measures items of estimated size
/// (react-aria's `useVirtualizerItem` + `VirtualizerItem`).
pub fn use_virtualizer_item(
    input: UseVirtualizerItemInput,
    element: CapturedElement,
) -> UseVirtualizerItemReturn {
    let UseVirtualizerItemInput {
        layout_info,
        parent,
        update_item_size,
        should_observe_item_size,
    } = input;
    let direction = use_direction();

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;

        use crate::hooks::collections::Size;

        let update_size = move || {
            let Some(element) = element.get_untracked() else {
                return;
            };
            let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else {
                return;
            };
            // A hidden collection (display none) must not report 0, it wouldn't remeasure.
            if html.offset_width() == 0 && html.offset_height() == 0 {
                return;
            }
            // The intrinsic size: without the height the layout gave it.
            let style = html.style();
            let height = style.get_property_value("height").unwrap_or_default();
            let _ = style.set_property("height", "");
            let size = Size::new(
                f64::from(html.scroll_width()),
                f64::from(html.scroll_height()),
            );
            let _ = style.set_property("height", &height);
            let Some(info) = layout_info.try_get_untracked() else {
                return;
            };
            let _ = update_item_size.try_run((info.key, size));
        };
        // Items of estimated size are measured once rendered.
        Effect::new(move |_| {
            if layout_info.with(|info| info.estimated_size) && element.get().is_some() {
                update_size();
            }
        });
        if should_observe_item_size {
            // The wrapper's size is fixed by the layout: observe the children. The observer's
            // callback lives as long as the observer (rows mount and unmount while scrolling).
            type Observer = (
                web_sys::ResizeObserver,
                wasm_bindgen::closure::Closure<dyn Fn(js_sys::Array)>,
            );
            let observer = StoredValue::new_local(None::<Observer>);
            let disconnect = move || {
                if let Some(Some((observer, _callback))) = observer.try_update_value(Option::take) {
                    observer.disconnect();
                }
            };
            Effect::new(move |_| {
                disconnect();
                let Some(element) = element.get() else {
                    return;
                };
                let callback = wasm_bindgen::closure::Closure::<dyn Fn(js_sys::Array)>::new(
                    move |entries: js_sys::Array| {
                        if entries.length() > 0 {
                            update_size();
                        }
                    },
                );
                let Ok(resize_observer) =
                    web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref())
                else {
                    return;
                };
                let children = element.children();
                for index in 0..children.length() {
                    if let Some(child) = children.item(index) {
                        resize_observer.observe(&child);
                    }
                }
                observer.set_value(Some((resize_observer, callback)));
            });
            on_cleanup(disconnect);
        }
    }
    #[cfg(feature = "ssr")]
    let _ = (update_item_size, element, should_observe_item_size);

    let styles = Signal::derive(move || {
        layout_info.with(|info| {
            parent.with(|parent| layout_info_styles(info, direction.get(), parent.as_ref()))
        })
    });
    UseVirtualizerItemReturn { styles }
}

/// The styles placing an element at its layout info (react-aria's `layoutInfoToStyle`).
pub fn layout_info_styles(
    info: &LayoutInfo,
    direction: WritingDirection,
    parent: Option<&LayoutInfo>,
) -> Styles {
    let x_property = match direction {
        WritingDirection::Rtl => "right",
        WritingDirection::Ltr => "left",
    };
    // Relative to the parent, unless sticky in an overflowing parent.
    let relative_to = parent.filter(|parent| !(parent.allow_overflow && info.is_sticky));
    let top = info.rect.y - relative_to.map_or(0.0, |parent| parent.rect.y);
    let x = info.rect.x - relative_to.map_or(0.0, |parent| parent.rect.x);
    let px = |value: f64| value.is_finite().then(|| format!("{value}px"));
    Styles::new()
        .add_unchecked(
            "position",
            if info.is_sticky { "sticky" } else { "absolute" },
        )
        // Sticky elements flow: inline-block keeps them from pushing other sticky columns.
        .add_optional_unchecked("display", info.is_sticky.then_some("inline-block"))
        .add_unchecked(
            "overflow",
            if info.allow_overflow {
                "visible"
            } else {
                "hidden"
            },
        )
        .add_unchecked("opacity", info.opacity.to_string())
        .add_unchecked("z-index", info.z_index.to_string())
        .add_optional_unchecked("transform", info.transform.as_deref().map(str::to_owned))
        .add_unchecked("contain", "size layout style")
        .add_optional_unchecked("top", px(top))
        .add_optional_unchecked(x_property, px(x))
        .add_optional_unchecked("width", px(info.rect.width))
        .add_optional_unchecked("height", px(info.rect.height))
}
