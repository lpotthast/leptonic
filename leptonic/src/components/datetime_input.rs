use leptos::prelude::*;
use time::format_description::well_known::Rfc3339;
use web_sys::KeyboardEvent;

use crate::{
    components::date_selector::{DateSelector, DateSelectorProps},
    utils::time::{GuideMode, Type},
    Margin, Out,
};

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn DateTimeInput(
    #[prop(into, optional, default = Signal::from(Oco::Borrowed("")))] label: Signal<
        Oco<'static, str>,
    >,
    #[prop(into)] get: Signal<Option<time::OffsetDateTime>>,
    #[prop(into)] set: Out<Option<time::OffsetDateTime>>,
    #[prop(into, optional)] prepend: ViewFn,
    #[prop(into, optional)] id: Option<Oco<'static, str>>,
    #[prop(into, optional)] class: Option<Oco<'static, str>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] margin: Option<Margin>,

    #[prop(optional)] min: Option<time::OffsetDateTime>,
    #[prop(optional)] max: Option<time::OffsetDateTime>,
    #[prop(optional)] input_type: Type,
    #[prop(optional)] guide_mode: GuideMode,
    // #[prop(into)] on_open: Option<Callback<()>>,
    // #[prop(into)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let id = id.map(Oco::into_owned);

    let class = class.map_or_else(
        || Oco::from("leptonic-input datetime-selected "),
        |it| Oco::from(format!("leptonic-input datetime-selected {it}")),
    );

    let style = margin.map(|it| format!("--margin: {it}"));

    let (open, set_open) = signal(false);
    let (in_focus, set_in_focus) = signal(false);

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

    let date_selector = move || {
        DateSelector(DateSelectorProps {
            value: get.get().unwrap(),
            on_change: Out::new_callback(move |new_value| {
                tracing::info!("Received new value {:?}", new_value);
                // Skip propagating a change event when the received value does not deviate from the current value.
                if let Some(current) = get.get() {
                    if current == new_value {
                        return;
                    }
                }
                set.set(Some(new_value));
            }),
            min,
            max,
            guide_mode: guide_mode.into(),
        })
    };

    let time_selector = move || {
        view! { "TODO: Implement the time selector!" }
    };

    view! {
        <leptonic-input-field style=style>
            {prepend.run()}
            <input
                id=id
                class=class
                placeholder=label
                tabindex="0"
                type="text"
                prop:disabled=move || disabled.get()
                prop:value=move || {
                    get
                        .get()
                        .map(|it| {
                            it.format(&Rfc3339).expect("Formatting to Rfc3339 to be non-fallible.")
                        })
                        .unwrap_or_default()
                }
                on:click=move |_| set_open.update(|open| *open = !*open)
                on:focusin=move |_| set_in_focus.set(true)
                on:focusout=move |_| set_in_focus.set(false)
                on:keydown=on_key_down
            /> <div class="datetime-dropdown-menu-ref">
                <Show when=move || open.get() fallback=|| ()>
                    <div class="datetime-dropdown-menu">
                        {match input_type {
                            Type::Date => date_selector().into_any(),
                            Type::Time => time_selector().into_any(),
                            Type::DateTime => {
                                view! {
                                    {date_selector()}
                                    {time_selector()}
                                }
                                    .into_any()
                            }
                        }}
                    </div>
                </Show>
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
