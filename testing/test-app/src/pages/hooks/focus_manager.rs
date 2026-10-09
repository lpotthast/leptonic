use std::sync::Arc;

use leptonic::{
    IntoAttrs,
    atoms::focus_scope::FocusScope,
    hooks::focus::{
        FocusManager, FocusManagerOptions, Focusability, create_focus_manager,
        use_focus_manager_context,
    },
};
use leptos::{prelude::*, web_sys};

use crate::pages::prevent_focus_steal;

#[component]
pub fn PageHookFocusManager() -> impl IntoView {
    let fm = create_focus_manager();
    let focus_manager = StoredValue::new(fm.focus_manager);

    let fm_tabbable = create_focus_manager();
    let fm_tabbable_mgr = StoredValue::new(fm_tabbable.focus_manager);

    let fm_radio = create_focus_manager();
    let fm_radio_mgr = StoredValue::new(fm_radio.focus_manager);

    let fm_radio_none = create_focus_manager();
    let fm_radio_none_mgr = StoredValue::new(fm_radio_none.focus_manager);

    let fm_radio_wrap = create_focus_manager();
    let fm_radio_wrap_mgr = StoredValue::new(fm_radio_wrap.focus_manager);

    let fm_vis = create_focus_manager();
    let fm_vis_mgr = StoredValue::new(fm_vis.focus_manager);

    let fm_inert = create_focus_manager();
    let fm_inert_mgr = StoredValue::new(fm_inert.focus_manager);

    view! {
        <div id="test-page-hook-focus-manager">
            <h1>"Focus Manager Hook Test Page"</h1>

            <section>
                <h2>"Basic Navigation"</h2>
                // Focus outside the scope, for `focus_next`/`focus_previous` from outside.
                <button id="test-fm-outside-external">"External (outside scope)"</button>
                <div id="test-fm-scope" {..fm.props.into_attrs()}>
                    <button id="test-fm-item-1">"Item 1"</button>
                    <button id="test-fm-item-2">"Item 2"</button>
                    <button id="test-fm-item-3">"Item 3"</button>
                </div>
                <FocusManagerControls focus_manager />
                <FocusManagerWrapControls focus_manager />
                <FocusManagerAcceptControls focus_manager />
            </section>

            <section>
                <h2>"Tabbable Filtering"</h2>
                <div id="test-fm-tabbable-scope" {..fm_tabbable.props.into_attrs()}>
                    <button id="test-fm-tabbable-item-1">"Tabbable Item 1"</button>
                    <button id="test-fm-tabbable-item-2" tabindex="-1">
                        "Tabbable Item 2 (tabindex=-1)"
                    </button>
                    <button id="test-fm-tabbable-item-3">"Tabbable Item 3"</button>
                </div>
                <FocusManagerTabbableControls focus_manager=fm_tabbable_mgr />
            </section>

            <section>
                <h2>"Radio Group (One Checked)"</h2>
                <div id="test-fm-radio-scope" {..fm_radio.props.into_attrs()}>
                    <button id="test-fm-radio-btn-before">"Before"</button>
                    <input type="radio" name="test-radio-group" id="test-fm-radio-a" value="a" />
                    <input
                        type="radio"
                        name="test-radio-group"
                        id="test-fm-radio-b"
                        value="b"
                        checked
                    />
                    <input type="radio" name="test-radio-group" id="test-fm-radio-c" value="c" />
                    <button id="test-fm-radio-btn-after">"After"</button>
                </div>
                <FocusManagerRadioControls focus_manager=fm_radio_mgr />
            </section>

            <section>
                <h2>"Radio Group (None Checked)"</h2>
                <div id="test-fm-radio-none-scope" {..fm_radio_none.props.into_attrs()}>
                    <button id="test-fm-radio-none-btn-before">"Before"</button>
                    <input
                        type="radio"
                        name="test-radio-none-group"
                        id="test-fm-radio-none-a"
                        value="a"
                    />
                    <input
                        type="radio"
                        name="test-radio-none-group"
                        id="test-fm-radio-none-b"
                        value="b"
                    />
                    <input
                        type="radio"
                        name="test-radio-none-group"
                        id="test-fm-radio-none-c"
                        value="c"
                    />
                    <button id="test-fm-radio-none-btn-after">"After"</button>
                </div>
                <FocusManagerRadioNoneControls focus_manager=fm_radio_none_mgr />
            </section>

            <section>
                <h2>"Radio Group (Wrap)"</h2>
                <div id="test-fm-radio-wrap-scope" {..fm_radio_wrap.props.into_attrs()}>
                    <input
                        type="radio"
                        name="test-radio-wrap-group"
                        id="test-fm-radio-wrap-a"
                        value="a"
                    />
                    <input
                        type="radio"
                        name="test-radio-wrap-group"
                        id="test-fm-radio-wrap-b"
                        value="b"
                        checked
                    />
                    <input
                        type="radio"
                        name="test-radio-wrap-group"
                        id="test-fm-radio-wrap-c"
                        value="c"
                    />
                </div>
                <FocusManagerRadioWrapControls focus_manager=fm_radio_wrap_mgr />
            </section>

            <section>
                <h2>"Visibility Filtering"</h2>
                <div id="test-fm-vis-scope" {..fm_vis.props.into_attrs()}>
                    <button id="test-fm-vis-item-1">"Visible 1"</button>
                    <button id="test-fm-vis-item-2" style="display:none">
                        "Hidden display:none"
                    </button>
                    <button id="test-fm-vis-item-3" hidden>
                        "Hidden attr"
                    </button>
                    <button id="test-fm-vis-item-4" style="visibility:hidden">
                        "Hidden visibility"
                    </button>
                    <button id="test-fm-vis-item-5">"Visible 2"</button>
                </div>
                <FocusManagerVisControls focus_manager=fm_vis_mgr />
            </section>

            <section>
                <h2>"Inert Filtering"</h2>
                <div id="test-fm-inert-scope" {..fm_inert.props.into_attrs()}>
                    <button id="test-fm-inert-item-1">"Item 1"</button>
                    <div inert>
                        <button id="test-fm-inert-item-2">"Item 2 (inert parent)"</button>
                    </div>
                    <button id="test-fm-inert-item-3">"Item 3"</button>
                </div>
                <FocusManagerInertControls focus_manager=fm_inert_mgr />
            </section>

            <ScopeManager />
            <ContainerGroups />
        </div>
    }
}

#[component]
fn FocusManagerControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let default_opts = || FocusManagerOptions {
        wrap: false,
        focusability: Focusability::Focusable,
        from: None,
        accept: None,
    };

    let on_focus_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(default_opts());
        });
    };

    let on_focus_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(default_opts());
        });
    };

    let on_focus_first = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_first(default_opts());
        });
    };

    let on_focus_last = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_last(default_opts());
        });
    };

    view! {
        <div id="test-fm-controls">
            <button id="test-fm-focus-next" on:mousedown=prevent_focus_steal on:click=on_focus_next>
                "Focus Next"
            </button>
            <button id="test-fm-focus-prev" on:mousedown=prevent_focus_steal on:click=on_focus_prev>
                "Focus Previous"
            </button>
            <button
                id="test-fm-focus-first"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_first
            >
                "Focus First"
            </button>
            <button id="test-fm-focus-last" on:mousedown=prevent_focus_steal on:click=on_focus_last>
                "Focus Last"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerWrapControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let wrap_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: true,
                focusability: Focusability::Focusable,
                from: None,
                accept: None,
            });
        });
    };

    let wrap_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: true,
                focusability: Focusability::Focusable,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-wrap-controls">
            <button
                id="test-fm-wrap-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=wrap_next
            >
                "Wrap Next"
            </button>
            <button
                id="test-fm-wrap-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=wrap_prev
            >
                "Wrap Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerTabbableControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let tabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Tabbable,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Tabbable,
                from: None,
                accept: None,
            });
        });
    };

    let nontabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Focusable,
                from: None,
                accept: None,
            });
        });
    };

    let nontabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Focusable,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-tabbable-controls">
            <button
                id="test-fm-tabbable-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_next
            >
                "Tabbable Next"
            </button>
            <button
                id="test-fm-tabbable-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_prev
            >
                "Tabbable Prev"
            </button>
            <button
                id="test-fm-nontabbable-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=nontabbable_next
            >
                "Non-Tabbable Next"
            </button>
            <button
                id="test-fm-nontabbable-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=nontabbable_prev
            >
                "Non-Tabbable Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerAcceptControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let reject_item_2: Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync> =
        Arc::new(|el: &web_sys::Element| el.id() != "test-fm-item-2");

    let reject_filter = StoredValue::new(reject_item_2);

    let accept_next = move |_| {
        let filter = reject_filter.with_value(Arc::clone);
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Focusable,
                from: None,
                accept: Some(filter),
            });
        });
    };

    let accept_prev = move |_| {
        let filter = reject_filter.with_value(Arc::clone);
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Focusable,
                from: None,
                accept: Some(filter),
            });
        });
    };

    view! {
        <div id="test-fm-accept-controls">
            <button
                id="test-fm-accept-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=accept_next
            >
                "Accept Next"
            </button>
            <button
                id="test-fm-accept-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=accept_prev
            >
                "Accept Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerRadioControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let tabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Tabbable,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Tabbable,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-radio-controls">
            <button
                id="test-fm-radio-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_next
            >
                "Radio Next"
            </button>
            <button
                id="test-fm-radio-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_prev
            >
                "Radio Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerRadioNoneControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let tabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Tabbable,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                focusability: Focusability::Tabbable,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-radio-none-controls">
            <button
                id="test-fm-radio-none-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_next
            >
                "Radio None Next"
            </button>
            <button
                id="test-fm-radio-none-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_prev
            >
                "Radio None Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerRadioWrapControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let tabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: true,
                focusability: Focusability::Tabbable,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: true,
                focusability: Focusability::Tabbable,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-radio-wrap-controls">
            <button
                id="test-fm-radio-wrap-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_next
            >
                "Radio Wrap Next"
            </button>
            <button
                id="test-fm-radio-wrap-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_prev
            >
                "Radio Wrap Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerVisControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let default_opts = || FocusManagerOptions {
        wrap: false,
        focusability: Focusability::Focusable,
        from: None,
        accept: None,
    };

    let on_focus_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(default_opts());
        });
    };

    let on_focus_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(default_opts());
        });
    };

    view! {
        <div id="test-fm-vis-controls">
            <button
                id="test-fm-vis-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_next
            >
                "Vis Next"
            </button>
            <button
                id="test-fm-vis-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_prev
            >
                "Vis Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerInertControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let default_opts = || FocusManagerOptions {
        wrap: false,
        focusability: Focusability::Focusable,
        from: None,
        accept: None,
    };

    let on_focus_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(default_opts());
        });
    };

    let on_focus_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(default_opts());
        });
    };

    view! {
        <div id="test-fm-inert-controls">
            <button
                id="test-fm-inert-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_next
            >
                "Inert Next"
            </button>
            <button
                id="test-fm-inert-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_prev
            >
                "Inert Prev"
            </button>
        </div>
    }
}

/// A `FocusScope`'s own focus manager (`use_focus_manager_context`): its items move focus with the
/// arrow keys (FocusScope.test.js, "focus manager").
#[component]
fn ScopeManager() -> impl IntoView {
    view! {
        <section>
            <h2>"FocusScope's manager"</h2>
            <FocusScope>
                <ScopeItem id="test-fm-own-1" />
                <ScopeItem id="test-fm-own-2" />
                <ScopeItem id="test-fm-own-3" />
            </FocusScope>
        </section>
    }
}

/// ArrowRight focuses the next element of the scope, ArrowLeft the previous one, both wrapping.
#[component]
fn ScopeItem(id: &'static str) -> impl IntoView {
    let manager = use_focus_manager_context().expect("inside a FocusScope");
    let wrap = || FocusManagerOptions {
        wrap: true,
        ..FocusManagerOptions::default()
    };
    view! {
        <div
            id=id
            role="button"
            tabindex="0"
            on:keydown=move |e: web_sys::KeyboardEvent| match e.key().as_str() {
                "ArrowRight" => {
                    manager.focus_next(wrap());
                }
                "ArrowLeft" => {
                    manager.focus_previous(wrap());
                }
                _ => {}
            }
        >
            {id}
        </div>
    }
}

/// FocusScope.test.js "... accounting for container elements within the scope": a pointerdown on
/// a group focuses the next tabbable element from the group, i.e. its first tabbable item.
#[component]
fn ContainerGroups() -> impl IntoView {
    view! {
        <section>
            <h2>"Container elements"</h2>
            <FocusScope>
                <ContainerGroup id="test-fm-group-1">
                    <div id="test-fm-group-item-1" role="button" tabindex="-1"></div>
                    <div id="test-fm-group-item-2" role="button" tabindex="0"></div>
                    <div role="button" style="display: none"></div>
                </ContainerGroup>
                <ContainerGroup id="test-fm-group-2">
                    <div role="button" style="visibility: hidden"></div>
                    <div role="button" style="visibility: collapse"></div>
                    <div id="test-fm-group-item-3" role="button" tabindex="0"></div>
                </ContainerGroup>
            </FocusScope>
        </section>
    }
}

#[component]
fn ContainerGroup(id: &'static str, children: Children) -> impl IntoView {
    let manager = use_focus_manager_context().expect("inside a FocusScope");
    view! {
        <div
            id=id
            role="group"
            on:pointerdown=move |e: web_sys::PointerEvent| {
                // From the event's target (the group itself in the test), as upstream.
                manager.focus_next(FocusManagerOptions {
                    from: e
                        .target()
                        .and_then(|target| {
                            wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(target).ok()
                        }),
                    focusability: Focusability::Tabbable,
                    ..FocusManagerOptions::default()
                });
            }
        >
            {children()}
        </div>
    }
}
