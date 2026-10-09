// Upstream: react-aria/src/virtualizer/useVirtualizerItem.ts @ 99e6102368
// Upstream: react-aria/src/virtualizer/VirtualizerItem.tsx @ 99e6102368
use leptos::prelude::*;

use super::LayoutInfo;
use crate::{
    CapturedElement,
    hooks::collections::{Key, Size},
    utils::{
        i18n::{WritingDirection, use_direction},
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the item wrapper's styles (react-aria's `VirtualizerItem` renders the wrapper with
//   `layoutInfoToStyle`); the caller renders the wrapper (`role="presentation"`) and captures it
//   (`element`), and inside it a content wrapper (`content`, styled with `content_styles`)
//   around the item: the content wrapper is what is measured and observed.
// - Items are measured through an [`ItemMeasurer`] shared by all items of a collection
//   ([`use_item_measurer`]; react-aria: each item calls `virtualizer.updateItemSize(key, size)`
//   itself), which reports an [`ItemSizeChange`].
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - The items waiting for their first measurement are measured together (in a microtask after
//   they rendered): their sizes are read one after another without writes in between, so the
//   browser lays out once for all of them (react-aria: each item measures itself in a layout
//   effect, clearing its height first, which forces a layout per item).
// - The size comes from the content wrapper (`display: flow-root`, its height is the item's
//   intrinsic height) instead of the wrapper with its height cleared (no style writes).
// - One `ResizeObserver` observes the content wrappers of all items (react-aria: one per item,
//   observing the wrapper's element children present at mount, so items rendering only text are
//   never observed: their TODO). It reports the sizes the browser just laid out.
//
// =============================================================================

/// An item's measured size (see [`UseItemMeasurerInput::update_item_size`]).
#[derive(Debug, Clone, PartialEq)]
pub struct ItemSizeChange {
    pub key: Key,
    pub size: Size,
}

/// Input of [`use_item_measurer`].
pub struct UseItemMeasurerInput {
    /// Receives the measured size of an item (react-aria: the virtualizer's `updateItemSize`,
    /// e.g. `VirtualizerState::update_item_size`).
    pub update_item_size: Callback<ItemSizeChange>,
    /// Re-measure items whenever their content resizes (variable sizes that change after the
    /// first measurement). Items of estimated size are always measured once rendered.
    pub should_observe_item_size: bool,
}

/// Measures the rendered items of one virtualized collection (see [`use_item_measurer`]).
#[derive(Clone, Copy)]
pub struct ItemMeasurer {
    #[cfg(not(feature = "ssr"))]
    update_item_size: Callback<ItemSizeChange>,
    should_observe_item_size: bool,
    #[cfg(not(feature = "ssr"))]
    state: StoredValue<client::MeasurerState, LocalStorage>,
}

impl std::fmt::Debug for ItemMeasurer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ItemMeasurer")
            .field("should_observe_item_size", &self.should_observe_item_size)
            .finish_non_exhaustive()
    }
}

/// Creates the measurer of a virtualized collection's items, shared by their
/// [`use_virtualizer_item`]s: items of estimated size are measured together once rendered, and
/// with `should_observe_item_size` one `ResizeObserver` re-measures items whose content resizes.
pub fn use_item_measurer(input: UseItemMeasurerInput) -> ItemMeasurer {
    let UseItemMeasurerInput {
        should_observe_item_size,
        ..
    } = input;
    #[cfg(not(feature = "ssr"))]
    {
        let state = StoredValue::new_local(client::MeasurerState::default());
        on_cleanup(move || {
            state.try_update_value(client::MeasurerState::disconnect);
        });
        ItemMeasurer {
            update_item_size: input.update_item_size,
            should_observe_item_size,
            state,
        }
    }
    #[cfg(feature = "ssr")]
    ItemMeasurer {
        should_observe_item_size,
    }
}

/// Input of [`use_virtualizer_item`].
pub struct UseVirtualizerItemInput {
    /// The item's wrapper, positioned at its layout info.
    pub element: CapturedElement,
    /// The wrapper inside it around the item's content (styled with
    /// [`UseVirtualizerItemReturn::content_styles`]), which is measured.
    pub content: CapturedElement,
    pub layout_info: Signal<LayoutInfo>,
    /// The layout info of the parent view (e.g. the section), for relative positions.
    pub parent: Signal<Option<LayoutInfo>>,
    /// The collection's measurer.
    pub measurer: ItemMeasurer,
}

/// Return value of [`use_virtualizer_item`].
pub struct UseVirtualizerItemReturn {
    /// The wrapper's styles: absolutely positioned at its layout info.
    pub styles: Signal<Styles>,
    /// The content wrapper's styles: a block formatting context (`display: flow-root`), so its
    /// height is the content's, margins included.
    pub content_styles: Styles,
}

/// Positions an item of a virtualized collection and measures items of estimated size
/// (react-aria's `useVirtualizerItem` + `VirtualizerItem`).
pub fn use_virtualizer_item(input: UseVirtualizerItemInput) -> UseVirtualizerItemReturn {
    let UseVirtualizerItemInput {
        element,
        content,
        layout_info,
        parent,
        measurer,
    } = input;
    let direction = use_direction();

    #[cfg(not(feature = "ssr"))]
    {
        // Registered once both wrappers are rendered; items of estimated size are measured then.
        let id = StoredValue::new(None::<u64>);
        Effect::new(move |_| {
            let estimated = layout_info.with(|info| info.estimated_size);
            let (Some(element), Some(content)) = (element.get(), content.get()) else {
                return;
            };
            let registered = id.get_value().or_else(|| {
                let registered = measurer.register(&element, &content, layout_info);
                id.set_value(registered);
                registered
            });
            if let Some(registered) = registered
                && estimated
            {
                measurer.request(registered);
            }
        });
        on_cleanup(move || {
            if let Some(Some(id)) = id.try_get_value() {
                measurer.unregister(id);
            }
        });
    }
    #[cfg(feature = "ssr")]
    let _ = (element, content, measurer);

    let styles = Signal::derive(move || {
        layout_info.with(|info| {
            parent.with(|parent| layout_info_styles(info, direction.get(), parent.as_ref()))
        })
    });
    UseVirtualizerItemReturn {
        styles,
        content_styles: Styles::new().add_unchecked("display", "flow-root"),
    }
}

#[cfg(not(feature = "ssr"))]
mod client {
    use std::collections::{HashMap, HashSet};

    use leptos::prelude::*;
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};

    use super::{ItemMeasurer, ItemSizeChange, LayoutInfo};
    use crate::hooks::collections::Size;

    /// A rendered item.
    struct Item {
        element: web_sys::HtmlElement,
        content: web_sys::HtmlElement,
        layout_info: Signal<LayoutInfo>,
    }

    type Observer = (web_sys::ResizeObserver, Closure<dyn Fn(js_sys::Array)>);

    #[derive(Default)]
    pub(super) struct MeasurerState {
        items: HashMap<u64, Item>,
        next_id: u64,
        /// Content wrapper → item id, for the observer's entries.
        ids: Option<js_sys::Map>,
        /// The items waiting for their first measurement, and whether it is scheduled.
        pending: HashSet<u64>,
        scheduled: bool,
        observer: Option<Observer>,
        /// Content wrappers to observe in the next frame, and whether that is scheduled.
        unobserved: Vec<web_sys::HtmlElement>,
        observe_scheduled: bool,
    }

    impl MeasurerState {
        pub(super) fn disconnect(&mut self) {
            if let Some((observer, _callback)) = self.observer.take() {
                observer.disconnect();
            }
        }
    }

    impl ItemMeasurer {
        /// Registers a rendered item; its id (`None` once the measurer is disposed).
        pub(super) fn register(
            self,
            element: &web_sys::Element,
            content: &web_sys::Element,
            layout_info: Signal<LayoutInfo>,
        ) -> Option<u64> {
            let element = element.dyn_ref::<web_sys::HtmlElement>()?;
            let content = content.dyn_ref::<web_sys::HtmlElement>()?;
            let (id, schedule_observing) = self.state.try_update_value(|state| {
                let id = state.next_id;
                state.next_id += 1;
                state
                    .ids
                    .get_or_insert_with(js_sys::Map::new)
                    .set(content, &JsValue::from_f64(id_to_f64(id)));
                let mut schedule_observing = false;
                if self.should_observe_item_size {
                    state.unobserved.push(content.clone());
                    schedule_observing = !std::mem::replace(&mut state.observe_scheduled, true);
                }
                state.items.insert(
                    id,
                    Item {
                        element: element.clone(),
                        content: content.clone(),
                        layout_info,
                    },
                );
                (id, schedule_observing)
            })?;
            if schedule_observing {
                // In the next frame: an element observed while the browser delivers resize
                // observations (rows rendered after a measurement) would be skipped, reported as
                // an error ("ResizeObserver loop completed with undelivered notifications").
                request_animation_frame(move || self.observe_new());
            }
            Some(id)
        }

        /// Observes the content wrappers registered since the last frame.
        fn observe_new(self) {
            let Some(observer) = self.observer() else {
                return;
            };
            self.state.try_update_value(|state| {
                state.observe_scheduled = false;
                let Some(ids) = &state.ids else {
                    return;
                };
                for content in state.unobserved.drain(..) {
                    // Still rendered.
                    if ids.has(&content) {
                        observer.observe(&content);
                    }
                }
            });
        }

        pub(super) fn unregister(self, id: u64) {
            self.state.try_update_value(|state| {
                state.pending.remove(&id);
                if let Some(item) = state.items.remove(&id) {
                    if let Some(ids) = &state.ids {
                        ids.delete(&item.content);
                    }
                    if let Some((observer, _)) = &state.observer {
                        observer.unobserve(&item.content);
                    }
                }
            });
        }

        /// Measures the item with the others waiting, after the current rendering.
        pub(super) fn request(self, id: u64) {
            let schedule = self
                .state
                .try_update_value(|state| {
                    state.pending.insert(id);
                    !std::mem::replace(&mut state.scheduled, true)
                })
                .unwrap_or(false);
            if schedule {
                queue_microtask(move || {
                    let ids = self
                        .state
                        .try_update_value(|state| {
                            state.scheduled = false;
                            std::mem::take(&mut state.pending)
                        })
                        .unwrap_or_default();
                    self.measure(ids);
                });
            }
        }

        /// Reads the sizes of `ids` (no writes in between: one layout), then reports them.
        fn measure(self, ids: impl IntoIterator<Item = u64>) {
            let changes: Vec<ItemSizeChange> = self
                .state
                .try_with_value(|state| {
                    ids.into_iter()
                        .filter_map(|id| state.items.get(&id))
                        .filter_map(|item| {
                            // A hidden collection (display none) must not report 0: it wouldn't
                            // remeasure once shown.
                            if item.element.offset_width() == 0 && item.element.offset_height() == 0
                            {
                                return None;
                            }
                            // The wrapper's overflow across (react-aria's `scrollWidth`), the
                            // content's height along.
                            let size = Size::new(
                                f64::from(item.element.scroll_width()),
                                f64::from(item.content.offset_height()),
                            );
                            let key = item
                                .layout_info
                                .try_with_untracked(|info| info.key.clone())?;
                            Some(ItemSizeChange { key, size })
                        })
                        .collect()
                })
                .unwrap_or_default();
            for change in changes {
                let _ = self.update_item_size.try_run(change);
            }
        }

        /// The observer of the items' content wrappers, created with the first item.
        fn observer(self) -> Option<web_sys::ResizeObserver> {
            if let Some(Some(observer)) = self.state.try_with_value(|state| {
                state
                    .observer
                    .as_ref()
                    .map(|(observer, _)| observer.clone())
            }) {
                return Some(observer);
            }
            let callback = Closure::<dyn Fn(js_sys::Array)>::new(move |entries: js_sys::Array| {
                let ids: Vec<u64> = self
                    .state
                    .try_with_value(|state| {
                        let Some(map) = &state.ids else {
                            return Vec::new();
                        };
                        entries
                            .iter()
                            .filter_map(|entry| {
                                let entry =
                                    entry.dyn_into::<web_sys::ResizeObserverEntry>().ok()?;
                                map.get(&entry.target()).as_f64().map(f64_to_id)
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                // The browser just laid out: reading the sizes costs no layout.
                self.measure(ids);
            });
            let observer = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()).ok()?;
            self.state.try_update_value(|state| {
                state.observer = Some((observer.clone(), callback));
            });
            Some(observer)
        }
    }

    // Ids stay far below 2^53: exact as JS numbers.
    #[allow(clippy::cast_precision_loss)]
    fn id_to_f64(id: u64) -> f64 {
        id as f64
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn f64_to_id(id: f64) -> u64 {
        id as u64
    }
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
