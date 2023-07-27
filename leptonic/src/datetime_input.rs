use std::borrow::Cow;

use leptos::*;
use time::format_description::well_known::Rfc3339;
use web_sys::KeyboardEvent;

use crate::{
    date_selector::DateSelectorProps,
    datetime::{GuideMode, Type},
    prelude::DateSelector,
    Margin, OptionalMaybeSignal,
};

#[component]
pub fn DateTimeInput<S>(
    cx: Scope,
    #[prop(optional, into)] label: OptionalMaybeSignal<String>,
    #[prop(into)] get: MaybeSignal<Option<time::OffsetDateTime>>,
    set: S,
    #[prop(optional, into)] prepend: OptionalMaybeSignal<View>,
    #[prop(into, optional)] id: Option<Cow<'static, str>>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(optional)] margin: Option<Margin>,

    #[prop(optional)] min: Option<time::OffsetDateTime>,
    #[prop(optional)] max: Option<time::OffsetDateTime>,
    #[prop(optional)] input_type: Type,
    #[prop(optional)] guide_mode: GuideMode,
    // #[prop(into)] on_open: Option<Callback<()>>,
    // #[prop(into)] on_close: Option<Callback<()>>,
) -> impl IntoView
where
    S: Fn(Option<time::OffsetDateTime>) + 'static,
{
    let id = id.map(|it| it.into_owned());

    let class = class
        .map(|it| Cow::Owned(format!("leptonic-input datetime-selected {it}")))
        .unwrap_or(Cow::Borrowed("leptonic-input datetime-selected "));

    let style = margin.map(|it| format!("--margin: {it}"));

    let (open, set_open) = create_signal(cx, false);
    let (in_focus, set_in_focus) = create_signal(cx, false);

    let on_key_down = move |event: KeyboardEvent| {
        let in_focus = in_focus.get();
        let open = open.get();
        if in_focus {
            if !open
                && (event.key().as_str() == "ArrowDown"
                    || event.key().as_str() == "Enter"
                    || event.key().as_str() == " ")
            {
                event.prevent_default();
                set_open.set(true);
            } else if open && (event.key().as_str() == "Escape" || event.key().as_str() == "Tab") {
                set_open.set(false);
            } else if event.key().as_str() == "Tab" {
                // Do nothing.
            } else {
                event.prevent_default();
                event.stop_propagation();
            }
        }
    };

    let on_change = move |new_value| {
        tracing::info!("Received new value {:?}", new_value);
        // Skip propagating a change event when the received value does not deviate from the current value.
        if let Some(current) = get.get() {
            if current == new_value {
                return;
            }
        }
        set(Some(new_value));
    };

    let date_selector = move || {
        DateSelector(
            cx,
            DateSelectorProps {
                value: get.get().unwrap(),
                on_change,
                min,
                max,
                guide_mode: guide_mode.into(),
            },
        )
    };

    let time_selector = move || {
        view! {cx,
            "TODO: Implement the time selector!"
            // <CrudOffsetDatetimeTimeSelector
            //     value={this.value.clone().unwrap_or_else(|| time::OffsetDateTime::now_utc())}
            // />
        }
    };

    view! { cx,
        <leptonic-input-field style=style>
            {match prepend.0 {
                Some(view) => view! { cx,
                    <div>
                        { view.get() }
                    </div>
                }.into_view(cx),
                None => ().into_view(cx),
            }}
            <input
                id=id
                class=class
                placeholder=move || match &label.0 {
                    Some(label) => Cow::Owned(label.get()),
                    None => Cow::Borrowed(""),
                }
                tabindex="0"
                type="text"
                prop:disabled=move || disabled.0.as_ref().map(|it| it.get()).unwrap_or(false)
                prop:value=move || get.get().map(|it| it.format(&Rfc3339).expect("Formatting to Rfc3339 to be non-fallible.")).unwrap_or_default()
                on:click=move |_| set_open.update(|open| *open = !*open)
                on:focusin=move |_| set_in_focus.set(true)
                on:focusout=move |_| set_in_focus.set(false)
                on:keydown=on_key_down
            />
            <div class="datetime-dropdown-menu-ref">
                { match open.get() {
                    true => view! {cx,
                        <div class="datetime-dropdown-menu">
                            {
                                match input_type {
                                    Type::Date => date_selector().into_view(cx),
                                    Type::Time => time_selector().into_view(cx),
                                    Type::DateTime => view! {cx,
                                        <>
                                            {date_selector()}
                                            {time_selector()}
                                        </>
                                    }.into_view(cx),
                                }
                            }
                        </div>
                    }.into_view(cx),
                    false => ().into_view(cx)
                } }
            </div>
        </leptonic-input-field>
    }
}

// TODO: Consider this old piece of YEW code...
//             Msg::CloseMenu => {
//                 if self.open {
//                     // Opening the menu puts the focus on the search input.
//                     // On close, the focus on the select itself should be restored, as the user might still want to interact with it
//                     // or want to tab further down the focusable elements on the site.
//                     // If the menu is closed by pressing escape,
//                     // the search input is still focused and the focus can be restored safely.
//                     // If the menu is closed with a click outside the select menu and onto a focusable element, restoring focus
//                     // to the select would probably be against the users intention / will.
//
//                     // TODO: Implement this via js calls...
//                     //if (!!this.input && !!this.input.nativeElement
//                     //&& !!this.document.activeElement
//                     //&& (this.document.activeElement === this.document.body
//                     //    || this.document.activeElement === this.document.body.parentElement)) {
//                     //this.input.nativeElement.focus();
//                     //}
//
//                     self.open = false;
//                     if let Some(onclose) = &ctx.props().onclose {
//                         onclose.emit(());
//                     }
//                     true
//                 } else {
//                     false
//                 }
//             }
