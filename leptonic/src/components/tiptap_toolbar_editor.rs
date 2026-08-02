use leptos::prelude::*;
use leptos_tiptap::{
    TiptapActiveKey, TiptapContent, TiptapEditor as UpstreamTiptapEditor, TiptapEditorHandle,
    TiptapEditorReport, TiptapEditorResult, TiptapHeadingLevel, TiptapSelectionState,
    TiptapTextAlign,
};

use crate::components::{
    button::{Button, ButtonSize},
    icon::Icon,
};

fn report_command_result(
    result: TiptapEditorResult<()>,
    report_error: Callback<TiptapEditorReport>,
) {
    if let Err(report) = result {
        report_error.run(report);
    }
}

/// A [`leptos_tiptap::TiptapEditor`] with Leptonic's opinionated formatting toolbar.
///
/// The editor activates every extension compiled by Leptonic's `tiptap` feature. Use the upstream
/// component or hook directly when you need custom extension selection, placeholder text, host
/// classes or styles, or custom host composition.
#[component]
#[allow(clippy::too_many_lines)]
pub fn TiptapToolbarEditor(
    /// A stable ID which must be globally unique across all mounted editor instances.
    #[prop(into)]
    id: String,
    /// The handle used to observe readiness, run commands, and read editor content.
    handle: TiptapEditorHandle,
    /// Content used once when the editor is initialized.
    #[prop(into)]
    initial_content: TiptapContent,
    /// Reactively controls whether the editor is editable. The toolbar is hidden while disabled.
    #[prop(into, optional)]
    disabled: Signal<bool>,
    /// Called once the editor is ready and available through `handle`.
    #[prop(into, optional)]
    on_ready: Option<Callback<()>>,
    /// Called when the document changes. Pull the desired representation through `handle`.
    #[prop(into, optional)]
    on_change: Option<Callback<()>>,
    /// Receives asynchronous bridge errors and toolbar command failures.
    #[prop(into, optional)]
    on_error: Option<Callback<TiptapEditorReport>>,
) -> impl IntoView {
    let (selection, set_selection) = signal(TiptapSelectionState::default());
    let toolbar_disabled = Signal::derive(move || !handle.is_ready());
    let report_error = Callback::new(move |report: TiptapEditorReport| {
        if let Some(on_error) = on_error.as_ref() {
            on_error.run(report);
        } else {
            tracing::error!(error = %report, "Tiptap editor operation failed");
        }
    });
    let on_ready = Callback::new(move |()| {
        if let Some(on_ready) = on_ready.as_ref() {
            on_ready.run(());
        }
    });
    let on_change = Callback::new(move |()| {
        if let Some(on_change) = on_change.as_ref() {
            on_change.run(());
        }
    });

    let button_class = move |key| {
        move || {
            format!(
                "leptonic-tiptap-btn {}",
                if selection.with(|state| state.is_active(key)) {
                    "active"
                } else {
                    ""
                }
            )
        }
    };

    view! {
        <leptonic-tiptap-editor>
            {move || (!disabled.get()).then(|| view! {
                <leptonic-tiptap-menu>
                    <Button
                        attr:class=button_class(TiptapActiveKey::H1)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_heading(TiptapHeadingLevel::H1), report_error)
                    >
                        "H1"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::H2)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_heading(TiptapHeadingLevel::H2), report_error)
                    >
                        "H2"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::H3)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_heading(TiptapHeadingLevel::H3), report_error)
                    >
                        "H3"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::H4)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_heading(TiptapHeadingLevel::H4), report_error)
                    >
                        "H4"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::H5)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_heading(TiptapHeadingLevel::H5), report_error)
                    >
                        "H5"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::H6)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_heading(TiptapHeadingLevel::H6), report_error)
                    >
                        "H6"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::Paragraph)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.set_paragraph(), report_error)
                    >
                        <Icon icon=icondata::BsParagraph/>
                        "Paragraph"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::Bold)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_bold(), report_error)
                    >
                        <Icon icon=icondata::BsTypeBold/>
                        "Bold"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::Italic)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_italic(), report_error)
                    >
                        <Icon icon=icondata::BsTypeItalic/>
                        "Italic"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::Strike)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_strike(), report_error)
                    >
                        <Icon icon=icondata::BsTypeStrikethrough/>
                        "Strike"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::Blockquote)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_blockquote(), report_error)
                    >
                        <Icon icon=icondata::BsBlockquoteLeft/>
                        "Blockquote"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::Highlight)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.toggle_highlight(None), report_error)
                    >
                        <Icon icon=icondata::BsBrightnessAltHigh/>
                        "Highlight"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::AlignLeft)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.set_text_align(TiptapTextAlign::Left), report_error)
                    >
                        <Icon icon=icondata::BsTextLeft/>
                        "left"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::AlignCenter)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.set_text_align(TiptapTextAlign::Center), report_error)
                    >
                        <Icon icon=icondata::BsTextCenter/>
                        "center"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::AlignRight)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.set_text_align(TiptapTextAlign::Right), report_error)
                    >
                        <Icon icon=icondata::BsTextRight/>
                        "right"
                    </Button>
                    <Button
                        attr:class=button_class(TiptapActiveKey::AlignJustify)
                        size=ButtonSize::Small
                        disabled=toolbar_disabled
                        on_press=move |_| report_command_result(handle.set_text_align(TiptapTextAlign::Justify), report_error)
                    >
                        <Icon icon=icondata::BsJustify/>
                        "justify"
                    </Button>
                </leptonic-tiptap-menu>
            })}
            <UpstreamTiptapEditor
                id=id
                handle=handle
                initial_content=initial_content
                disabled=disabled
                on_ready=on_ready
                on_change=on_change
                on_error=report_error
                on_selection_change=move |state| set_selection.set(state)
            />
        </leptonic-tiptap-editor>
    }
}
