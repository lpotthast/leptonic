use std::sync::Arc;

use leptonic::hooks::{
    FocusManager, FocusManagerOptions, IntoAttrs, UseFocusManagerInput, use_focus_manager,
};
use leptos::{prelude::*, web_sys};

use crate::pages::prevent_focus_steal;

#[component]
pub fn PageHookFocusManager() -> impl IntoView {
    let fm = use_focus_manager(UseFocusManagerInput::default());
    let focus_manager = StoredValue::new(fm.focus_manager);

    let fm_wrap = use_focus_manager(UseFocusManagerInput::default());
    let fm_wrap_mgr = StoredValue::new(fm_wrap.focus_manager);

    let fm_tabbable = use_focus_manager(UseFocusManagerInput::default());
    let fm_tabbable_mgr = StoredValue::new(fm_tabbable.focus_manager);

    let fm_accept = use_focus_manager(UseFocusManagerInput::default());
    let fm_accept_mgr = StoredValue::new(fm_accept.focus_manager);

    let fm_radio = use_focus_manager(UseFocusManagerInput::default());
    let fm_radio_mgr = StoredValue::new(fm_radio.focus_manager);

    let fm_radio_none = use_focus_manager(UseFocusManagerInput::default());
    let fm_radio_none_mgr = StoredValue::new(fm_radio_none.focus_manager);

    let fm_radio_wrap = use_focus_manager(UseFocusManagerInput::default());
    let fm_radio_wrap_mgr = StoredValue::new(fm_radio_wrap.focus_manager);

    let fm_vis = use_focus_manager(UseFocusManagerInput::default());
    let fm_vis_mgr = StoredValue::new(fm_vis.focus_manager);

    let fm_inert = use_focus_manager(UseFocusManagerInput::default());
    let fm_inert_mgr = StoredValue::new(fm_inert.focus_manager);

    let fm_outside = use_focus_manager(UseFocusManagerInput::default());
    let fm_outside_mgr = StoredValue::new(fm_outside.focus_manager);

    view! {
        <div id="test-page-hook-focus-manager">
            <h1>"Focus Manager Hook Test Page"</h1>

            <section>
                <h2>"Basic Navigation"</h2>
                <div id="test-fm-scope" {..fm.props.into_attrs()}>
                    <button id="test-fm-item-1">"Item 1"</button>
                    <button id="test-fm-item-2">"Item 2"</button>
                    <button id="test-fm-item-3">"Item 3"</button>
                </div>
                <FocusManagerControls focus_manager />
            </section>

            <section>
                <h2>"Wrap"</h2>
                <div id="test-fm-wrap-scope" {..fm_wrap.props.into_attrs()}>
                    <button id="test-fm-wrap-item-1">"Wrap Item 1"</button>
                    <button id="test-fm-wrap-item-2">"Wrap Item 2"</button>
                    <button id="test-fm-wrap-item-3">"Wrap Item 3"</button>
                </div>
                <FocusManagerWrapControls focus_manager=fm_wrap_mgr />
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
                <h2>"Accept Filter"</h2>
                <div id="test-fm-accept-scope" {..fm_accept.props.into_attrs()}>
                    <button id="test-fm-accept-item-1">"Accept Item 1"</button>
                    <button id="test-fm-accept-item-2">"Accept Item 2"</button>
                    <button id="test-fm-accept-item-3">"Accept Item 3"</button>
                </div>
                <FocusManagerAcceptControls focus_manager=fm_accept_mgr />
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

            <section>
                <h2>"Outside Scope"</h2>
                <button id="test-fm-outside-external">"External (outside scope)"</button>
                <div id="test-fm-outside-scope" {..fm_outside.props.into_attrs()}>
                    <button id="test-fm-outside-item-1">"Outside Item 1"</button>
                    <button id="test-fm-outside-item-2">"Outside Item 2"</button>
                    <button id="test-fm-outside-item-3">"Outside Item 3"</button>
                </div>
                <FocusManagerOutsideControls focus_manager=fm_outside_mgr />
            </section>
        </div>
    }
}

#[component]
fn FocusManagerControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let default_opts = || FocusManagerOptions {
        wrap: false,
        tabbable: false,
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
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let wrap_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: true,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let nowrap_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let nowrap_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: false,
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
            <button
                id="test-fm-nowrap-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=nowrap_next
            >
                "No-Wrap Next"
            </button>
            <button
                id="test-fm-nowrap-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=nowrap_prev
            >
                "No-Wrap Prev"
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
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let nontabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let nontabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: false,
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
        Arc::new(|el: &web_sys::Element| el.id() != "test-fm-accept-item-2");

    let reject_filter = StoredValue::new(reject_item_2);

    let accept_next = move |_| {
        let filter = reject_filter.with_value(Arc::clone);
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: false,
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
                tabbable: false,
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
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: true,
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
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: true,
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
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: true,
                tabbable: true,
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
        tabbable: false,
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
        tabbable: false,
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

#[component]
fn FocusManagerOutsideControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let on_focus_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let on_focus_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-outside-controls">
            <button
                id="test-fm-outside-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_next
            >
                "Outside Next"
            </button>
            <button
                id="test-fm-outside-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_prev
            >
                "Outside Prev"
            </button>
        </div>
    }
}
