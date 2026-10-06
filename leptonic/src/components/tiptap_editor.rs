use leptos::prelude::*;
use leptos_tiptap::{
    TiptapActiveKey, TiptapContent, TiptapEditorHandle, TiptapEditorResult, TiptapHeadingLevel,
    TiptapSelectionState, TiptapSetContentOptions, TiptapTextAlign, UseTiptapEditorInput,
    use_tiptap_editor,
};

use crate::{
    Out,
    atoms::toolbar::Toolbar,
    components::{
        button::{Button, ButtonSize},
        icon::Icon,
    },
    utils::{ValueBinding, aria::AriaPressed, classes::Classes, id::use_id, styles::Styles},
};

/// A button of the editor's formatting toolbar.
struct FormatButton {
    label: &'static str,
    icon: Option<icondata::Icon>,
    command: fn(&TiptapEditorHandle) -> TiptapEditorResult<()>,
    /// The format the button turns on, for its pressed state.
    active: TiptapActiveKey,
}

const FORMAT_BUTTONS: [FormatButton; 16] = [
    FormatButton {
        label: "H1",
        icon: None,
        command: |h| h.toggle_heading(TiptapHeadingLevel::H1),
        active: TiptapActiveKey::H1,
    },
    FormatButton {
        label: "H2",
        icon: None,
        command: |h| h.toggle_heading(TiptapHeadingLevel::H2),
        active: TiptapActiveKey::H2,
    },
    FormatButton {
        label: "H3",
        icon: None,
        command: |h| h.toggle_heading(TiptapHeadingLevel::H3),
        active: TiptapActiveKey::H3,
    },
    FormatButton {
        label: "H4",
        icon: None,
        command: |h| h.toggle_heading(TiptapHeadingLevel::H4),
        active: TiptapActiveKey::H4,
    },
    FormatButton {
        label: "H5",
        icon: None,
        command: |h| h.toggle_heading(TiptapHeadingLevel::H5),
        active: TiptapActiveKey::H5,
    },
    FormatButton {
        label: "H6",
        icon: None,
        command: |h| h.toggle_heading(TiptapHeadingLevel::H6),
        active: TiptapActiveKey::H6,
    },
    FormatButton {
        label: "Paragraph",
        icon: Some(icondata::BsParagraph),
        command: TiptapEditorHandle::set_paragraph,
        active: TiptapActiveKey::Paragraph,
    },
    FormatButton {
        label: "Bold",
        icon: Some(icondata::BsTypeBold),
        command: TiptapEditorHandle::toggle_bold,
        active: TiptapActiveKey::Bold,
    },
    FormatButton {
        label: "Italic",
        icon: Some(icondata::BsTypeItalic),
        command: TiptapEditorHandle::toggle_italic,
        active: TiptapActiveKey::Italic,
    },
    FormatButton {
        label: "Strike",
        icon: Some(icondata::BsTypeStrikethrough),
        command: TiptapEditorHandle::toggle_strike,
        active: TiptapActiveKey::Strike,
    },
    FormatButton {
        label: "Blockquote",
        icon: Some(icondata::BsBlockquoteLeft),
        command: TiptapEditorHandle::toggle_blockquote,
        active: TiptapActiveKey::Blockquote,
    },
    FormatButton {
        label: "Highlight",
        icon: Some(icondata::BsBrightnessAltHigh),
        command: |h| h.toggle_highlight(None),
        active: TiptapActiveKey::Highlight,
    },
    FormatButton {
        label: "Align left",
        icon: Some(icondata::BsTextLeft),
        command: |h| h.set_text_align(TiptapTextAlign::Left),
        active: TiptapActiveKey::AlignLeft,
    },
    FormatButton {
        label: "Align center",
        icon: Some(icondata::BsTextCenter),
        command: |h| h.set_text_align(TiptapTextAlign::Center),
        active: TiptapActiveKey::AlignCenter,
    },
    FormatButton {
        label: "Align right",
        icon: Some(icondata::BsTextRight),
        command: |h| h.set_text_align(TiptapTextAlign::Right),
        active: TiptapActiveKey::AlignRight,
    },
    FormatButton {
        label: "Justify",
        icon: Some(icondata::BsJustify),
        command: |h| h.set_text_align(TiptapTextAlign::Justify),
        active: TiptapActiveKey::AlignJustify,
    },
];

/// A rich text editor (tiptap) with a formatting toolbar, editing HTML.
#[component]
pub fn TiptapEditor(
    /// The initial content (HTML).
    #[prop(into, optional)]
    default_value: String,
    /// The content as HTML (controlled): a value or any signal. Content set from outside replaces
    /// the editor's.
    #[prop(into, optional)]
    value: Option<Signal<String>>,
    /// Receives the content after every edit: an `RwSignal`, `WriteSignal`, closure, `Callback`,
    /// ...
    #[prop(into, optional)]
    set_value: Option<Out<String>>,
    /// Called with the content (HTML) after every edit.
    #[prop(into, optional)]
    on_change: Option<Callback<String>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Names the editor (a group of its toolbar and text). Default: "Rich text editor".
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let (binding, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let initial = binding.map_or(default_value, |binding| binding.value.get_untracked());
    // The content the editor reported last (or was given): only other content is set from outside.
    let current = StoredValue::new(initial.clone());
    let (selection, set_selection) = signal(TiptapSelectionState::default());

    let handle = TiptapEditorHandle::new();
    let editor = use_tiptap_editor(UseTiptapEditorInput {
        handle: Some(handle),
        on_change: Some(Callback::new(move |()| match handle.get_html() {
            Ok(html) => {
                current.set_value(html.clone());
                if let Some(binding) = binding {
                    binding.set(html.clone());
                }
                if let Some(on_change) = on_change {
                    on_change.run(html);
                }
            }
            Err(err) => tracing::warn!("TiptapEditor: reading the content failed: {err}"),
        })),
        on_selection_change: Some(Callback::new(move |state| set_selection.set(state))),
        disabled: is_disabled,
        ..UseTiptapEditorInput::new(use_id("tiptap"), TiptapContent::html(initial))
    });
    let is_ready = editor.is_ready;

    // Controlled: content from outside replaces the editor's (without reporting it back).
    if let Some(binding) = binding {
        Effect::new(move |_| {
            let html = binding.value.get();
            if !is_ready.get() || current.with_value(|current| *current == html) {
                return;
            }
            current.set_value(html.clone());
            if let Err(err) = handle.set_content_with_options(
                TiptapContent::html(html),
                TiptapSetContentOptions::default(),
            ) {
                tracing::warn!("TiptapEditor: setting the content failed: {err}");
            }
        });
    }

    let toolbar = move || {
        (!is_disabled.get()).then(|| {
            let buttons = FORMAT_BUTTONS
                .iter()
                .map(|button| {
                    let FormatButton {
                        label,
                        icon,
                        command,
                        active,
                    } = *button;
                    let pressed = Signal::derive(move || {
                        Some(AriaPressed::from(selection.with(|s| s.is_active(active))))
                    });
                    view! {
                        <Button
                            classes="leptonic-tiptap-btn"
                            size=ButtonSize::Small
                            is_disabled=Signal::derive(move || !is_ready.get())
                            aria_pressed=pressed
                            on_press=move |_| {
                                if let Err(err) = command(&handle) {
                                    tracing::warn!("TiptapEditor: \"{label}\" failed: {err}");
                                }
                            }
                        >
                            {icon.map(|icon| view! { <Icon icon=icon /> })}
                            {label}
                        </Button>
                    }
                })
                .collect_view();
            view! {
                <Toolbar classes="leptonic-tiptap-menu" aria_label="Formatting">
                    {buttons}
                </Toolbar>
            }
        })
    };
    let aria_label = move || {
        aria_label
            .get()
            .unwrap_or_else(|| "Rich text editor".to_owned())
    };

    view! {
        <div class=classes.add("leptonic-tiptap-editor") style=styles role="group" aria-label=aria_label>
            {toolbar}
            <div {..editor.props.into_attrs()} class="leptonic-tiptap-instance" />
        </div>
    }
}
