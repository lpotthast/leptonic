use std::sync::atomic::{AtomicIsize, Ordering};

use educe::Educe;
use leptos::on_cleanup;
use leptos_reactive::{create_effect, MaybeSignal, SignalGet, SignalGetUntracked};
use leptos_use::{core::ElementMaybeSignal, use_document, use_window};
use typed_builder::TypedBuilder;

use crate::utils::attributes::Attributes;

static PREVENT_SCROLL_COUNT: AtomicIsize = AtomicIsize::new(0);

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct UsePreventScrollInput {
    #[builder(setter(into))]
    pub disabled: MaybeSignal<bool>,
}

#[derive(Debug)]
pub struct UsePreventScrollReturn {
    pub props: UsePreventScrollProps,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UsePreventScrollProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
}

pub fn use_prevent_scroll<El, T>(
    el: Option<El>,
    input: UsePreventScrollInput,
) -> UsePreventScrollReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + 'static,
    T: Into<web_sys::Element> + Clone + 'static,
{
    // TODO: Use create_effect to access el reactively?
    if let Some(el) = el {
        if let Some(el) = el.into().get_untracked() {
            let el: web_sys::Element = el.into();
        }
    }

    let style = move |width_with_scrollbar: f64, width_without_scrollbar: f64| {
        format!(
            "overflow: hidden; padding-right: {}px;",
            width_with_scrollbar - width_without_scrollbar
        )
    };

    let register = move || {
        PREVENT_SCROLL_COUNT.fetch_add(1, Ordering::Release);

        if let Some(window) = use_window().as_ref() {
            if let Some(doc) = window.document() {
                if let Some(root) = doc.document_element() {
                    root.set_attribute(
                        "style",
                        style(
                            window
                                .inner_width()
                                .map(|it| it.as_f64().unwrap_or(0.0))
                                .unwrap_or(0.0),
                            root.client_width() as f64,
                        )
                        .as_str(),
                    )
                    .expect("Being able to set style attribute.");
                }
            }
        }
    };

    let cleanup = || {
        let _prev = PREVENT_SCROLL_COUNT.fetch_sub(1, Ordering::Release);
        let remaining = PREVENT_SCROLL_COUNT.load(Ordering::Acquire);
        if remaining == 0 {
            if let Some(window) = use_window().as_ref() {
                if let Some(doc) = window.document() {
                    if let Some(root) = doc.document_element() {
                        root.set_attribute("style", "")
                            .expect("Being able to set style attribute.");
                    }
                }
            }
        }
    };

    let _effect = create_effect(move |last| {
        if let Some(Some(())) = last {
            cleanup();
        }

        if input.disabled.get() {
            None
        } else {
            register();
            Some(())
        }
    });

    on_cleanup(move || {
        cleanup();
    });

    UsePreventScrollReturn {
        props: UsePreventScrollProps {
            attrs: Attributes::new(),
        },
    }
}

fn get_scroll_parent(node: web_sys::Element, check_for_overflow: bool) -> Option<web_sys::Element> {
    let mut node = node.parent_element();
    while let Some(el) = &node {
        if !is_scrollable(el, check_for_overflow) {
            node = el.parent_element()
        }
    }
    node.or_else(|| {
        use_document().as_ref().and_then(|doc| {
            if let Some(scrolling_element) = doc.scrolling_element() {
                Some(scrolling_element)
            } else if let Some(document_element) = doc.document_element() {
                Some(document_element)
            } else {
                None
            }
        })
    })
}

fn is_scrollable(node: &web_sys::Element, check_for_overflow: bool) -> bool {
    if let Some(window) = use_window().as_ref() {
        if let Ok(Some(style)) = window.get_computed_style(node) {
            let o = style.get_property_value("overflow").ok();
            let ox = style.get_property_value("overflowX").ok();
            let oy = style.get_property_value("overflowY").ok();

            fn is_scroll(s: Option<&str>) -> bool {
                match s {
                    Some(s) => s.contains("auto") || s.contains("scroll"),
                    None => false,
                }
            }

            let mut is_scrollable =
                is_scroll(o.as_deref()) || is_scroll(ox.as_deref()) || is_scroll(oy.as_deref());
            if is_scrollable && check_for_overflow {
                is_scrollable = node.scroll_height() != node.client_height()
                    || node.scroll_width() != node.client_width();
            }
            is_scrollable
        } else {
            false
        }
    } else {
        false
    }
}
