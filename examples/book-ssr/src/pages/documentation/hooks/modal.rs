use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::focus_scope::FocusScope;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;

#[component]
pub fn PageUseModalHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="modal" class="anchor">
                "Modal & Dialog Hooks"
                <AnchorLink href="#modal" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible modal dialogs with proper focus management, dismiss handling, and ARIA attributes."</p>

            <h2 id="hook-composition" class="anchor">
                "Hook Composition"
                <AnchorLink href="#hook-composition" description="Direct link to hook composition"/>
            </h2>

            <p>"The modal system is built from composable layers, each handling a specific concern:"</p>

            <ol>
                <li>
                    <strong>"State Layer"</strong>
                    " - "
                    <code>"use_modal_state"</code>
                    " or "
                    <code>"use_dialog_state"</code>
                    ": Manages open/close state and optional confirmation tracking"
                </li>
                <li>
                    <strong>"Behavior Layer"</strong>
                    " - "
                    <code>"use_modal"</code>
                    ": ARIA attributes (role, aria-modal), Escape key handling"
                </li>
                <li>
                    <strong>"Backdrop Layer"</strong>
                    " - "
                    <code>"use_modal_backdrop"</code>
                    ": Scroll prevention, backdrop click handling"
                </li>
                <li>
                    <strong>"Dialog Layer"</strong>
                    " - "
                    <code>"use_dialog"</code>
                    " (optional): ARIA labeling (aria-labelledby/describedby), Dialog vs AlertDialog role, focus on mount"
                </li>
                <li>
                    <strong>"Focus Layer"</strong>
                    " - "
                    <code>"FocusScope"</code>
                    " atom: Focus trapping and restoration"
                </li>
            </ol>

            <p>"For complete modal functionality, combine the layers you need. A basic modal uses "
               <code>"use_modal_state"</code>", "
               <code>"use_modal"</code>", "
               <code>"use_modal_backdrop"</code>", and "
               <code>"FocusScope"</code>". Add "
               <code>"use_dialog"</code>" when you need title/description ARIA associations or focus-on-mount behavior. "
               "Use "<code>"use_dialog"</code>" for ARIA semantics and focus management. "
               "Use "<code>"use_modal"</code>" for dismiss behavior and aria-modal. Compose both for full accessible dialogs."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"A complete modal with scroll prevention, backdrop dismiss, Escape key handling, and focus trapping:"</p>

            <BasicModalDemo />

            <h2 id="alert-dialog-demo" class="anchor">
                "Alert Dialog Demo"
                <AnchorLink href="#alert-dialog-demo" description="Direct link to alert dialog demo"/>
            </h2>

            <p>"An alert dialog is used for important messages that require user acknowledgment. "
               "It uses "<code>"role=\"alertdialog\""</code>" and typically cannot be dismissed with Escape:"</p>

            <AlertDialogDemo />

            <h2 id="non-dismissable-demo" class="anchor">
                "Non-Dismissable Modal Demo"
                <AnchorLink href="#non-dismissable-demo" description="Direct link to non-dismissable demo"/>
            </h2>

            <p>"A modal that cannot be dismissed by clicking outside or pressing Escape. "
               "Users must complete an action (like filling a form) to close it:"</p>

            <NonDismissableModalDemo />

            <h2 id="confirmation-demo" class="anchor">
                "Confirmation Dialog Demo"
                <AnchorLink href="#confirmation-demo" description="Direct link to confirmation demo"/>
            </h2>

            <p>"A confirmation dialog using "<code>"use_dialog_state"</code>" to track whether the user confirmed or cancelled. "
               "Watch the status text below to see the result:"</p>

            <ConfirmationDialogDemo />

            <h2 id="use_modal_state" class="anchor">
                "use_modal_state"
                <AnchorLink href="#use_modal_state" description="Direct link to use_modal_state"/>
            </h2>

            <Code>
                {indoc!(r"
                    let UseModalStateReturn { is_open, open, close, toggle } = use_modal_state(false);

                    // open.run(()) - opens the modal
                    // close.run(()) - closes the modal
                    // toggle.run(()) - toggles the modal
                ")}
            </Code>

            <h2 id="use_modal" class="anchor">
                "use_modal"
                <AnchorLink href="#use_modal" description="Direct link to use_modal"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseModalReturn { modal_props, id } = use_modal(UseModalInput {
                        is_open: is_open.into(),
                        on_close: Some(Callback::new(move |_| close.run(()))),
                        is_dismissable: true,
                        should_close_on_interact_outside: true,
                        is_keyboard_dismiss_disabled: false,
                    });

                    view! {
                        <Show when=move || is_open.get()>
                            <div {..modal_props}>
                                "Modal content"
                            </div>
                        </Show>
                    }
                "#)}
            </Code>

            <p>"Handles:"</p>
            <ul>
                <li>"Escape key to dismiss"</li>
                <li>"aria-modal attribute"</li>
                <li>"Close on outside interaction (optional)"</li>
            </ul>

            <h2 id="use_modal_backdrop" class="anchor">
                "use_modal_backdrop"
                <AnchorLink href="#use_modal_backdrop" description="Direct link to use_modal_backdrop"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseModalBackdropReturn { backdrop_props, content_props, .. } =
                        use_modal_backdrop(UseModalBackdropInput {
                            is_open,
                            on_close: Some(close),
                            should_close_on_interact_outside: true,
                            prevent_scroll: true,
                        });

                    view! {
                        <Show when=move || is_open.get()>
                            // Backdrop - spread backdrop_props for click handling
                            <div {..backdrop_props} class="backdrop">
                                // Modal content - spread content_props to stop propagation
                                <div {..content_props} class="modal">
                                    "Modal content"
                                </div>
                            </div>
                        </Show>
                    }
                "#)}
            </Code>

            <p>"Handles:"</p>
            <ul>
                <li>"Click outside detection (backdrop click)"</li>
                <li>"Scroll prevention on body"</li>
                <li>"Backdrop styling props"</li>
            </ul>

            <h2 id="use_dialog" class="anchor">
                "use_dialog"
                <AnchorLink href="#use_dialog" description="Direct link to use_dialog"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseDialogReturn { dialog_props, title_props, description_props, dialog_id } =
                        use_dialog(UseDialogInput {
                            title: Some("Title".to_string()),
                            description: Some("Description".to_string()),
                            aria_label: None,
                            role: DialogRole::Dialog, // or DialogRole::AlertDialog
                        });

                    // dialog_props includes an ElementCaptureAttr that automatically captures
                    // the DOM element for focus-on-mount — no manual NodeRef wiring needed.
                    view! {
                        <div {..dialog_props}>
                            <h2 id=title_props.id>"Title"</h2>
                            <p id=description_props.id>"Description"</p>
                        </div>
                    }
                "#)}
            </Code>

            <p>"Provides:"</p>
            <ul>
                <li>"role=\"dialog\" or role=\"alertdialog\""</li>
                <li>"aria-labelledby (links to title)"</li>
                <li>"aria-describedby (links to description)"</li>
                <li>"Focus on mount (focuses the dialog unless a child already has focus)"</li>
                <li>"iOS Safari VoiceOver workaround (blur/refocus after 500ms)"</li>
            </ul>

            <p>"For dismiss behavior (Escape key, aria-modal), compose with "<code>"use_modal"</code>"."</p>

            <h2 id="use_dialog_state" class="anchor">
                "use_dialog_state"
                <AnchorLink href="#use_dialog_state" description="Direct link to use_dialog_state"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseDialogStateReturn { is_open, open, close, confirm, is_confirmed } =
                        use_dialog_state(false);

                    // When user clicks "Confirm", call confirm.run(())
                    // When user clicks "Cancel", call close.run(())
                    // Check is_confirmed.get() to determine the result after close
                "#)}
            </Code>

            <h2 id="use_dismiss" class="anchor">
                "use_dismiss"
                <AnchorLink href="#use_dismiss" description="Direct link to use_dismiss"/>
            </h2>

            <p>"Handles dismissal behavior:"</p>
            <ul>
                <li>"Escape key handling"</li>
                <li>"Blur dismiss (for non-modal overlays)"</li>
                <li>"Accessible dismiss button creation"</li>
            </ul>

            <h2 id="dialog-roles" class="anchor">
                "Dialog Roles"
                <AnchorLink href="#dialog-roles" description="Direct link to dialog roles"/>
            </h2>

            <ul>
                <li><code>"DialogRole::Dialog"</code> " - Standard dialog for user interaction"</li>
                <li><code>"DialogRole::AlertDialog"</code> " - Important message requiring user response"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Escape key dismissal (via use_modal)"</li>
                <li>"Click outside to close (via use_modal_backdrop)"</li>
                <li>"Proper ARIA roles and associations (via use_dialog)"</li>
                <li>"Focus on mount with iOS Safari VoiceOver workaround (via use_dialog)"</li>
                <li>"Focus trapping (when combined with FocusScope)"</li>
                <li>"Scroll prevention on body (via use_modal_backdrop)"</li>
                <li>"Dismissable and non-dismissable modes"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Modal & Dialog Hooks", link: "#modal" },
                Toc::Leaf { title: "Hook Composition", link: "#hook-composition" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Alert Dialog Demo", link: "#alert-dialog-demo" },
                Toc::Leaf { title: "Non-Dismissable Demo", link: "#non-dismissable-demo" },
                Toc::Leaf { title: "Confirmation Demo", link: "#confirmation-demo" },
                Toc::Leaf { title: "use_modal_state", link: "#use_modal_state" },
                Toc::Leaf { title: "use_modal", link: "#use_modal" },
                Toc::Leaf { title: "use_modal_backdrop", link: "#use_modal_backdrop" },
                Toc::Leaf { title: "use_dialog", link: "#use_dialog" },
                Toc::Leaf { title: "use_dialog_state", link: "#use_dialog_state" },
                Toc::Leaf { title: "use_dismiss", link: "#use_dismiss" },
                Toc::Leaf { title: "Dialog Roles", link: "#dialog-roles" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

/// Basic modal demo with scroll prevention via use_modal_backdrop
#[component]
fn BasicModalDemo() -> impl IntoView {
    // State layer
    let UseModalStateReturn {
        is_open,
        open,
        close,
        toggle: _,
    } = use_modal_state(false);

    // Behavior layer - ARIA attributes and Escape key handling
    let UseModalReturn { modal_props, id: _ } = use_modal(UseModalInput {
        is_open,
        on_close: Some(close),
        is_dismissable: true,
        should_close_on_interact_outside: true,
        is_keyboard_dismiss_disabled: false,
    });

    // Backdrop layer - scroll prevention and backdrop click handling
    let UseModalBackdropReturn {
        backdrop_props,
        content_props,
        backdrop_id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: Some(close),
        should_close_on_interact_outside: true,
        prevent_scroll: true,
    });

    // Store in StoredValue so it can be copied into nested closures
    let modal_props = StoredValue::new(modal_props);
    let content_props = StoredValue::new(content_props);
    let backdrop_props = StoredValue::new(backdrop_props);

    view! {
        <button
            on:click=move |_| open.run(())
            style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: var(--brand-color); color: white; border: none; font-size: 1em;"
        >
            "Open Modal"
        </button>

        <Show when=move || is_open.get()>
            // Backdrop with backdrop_props for click handling
            <div
                {..backdrop_props.get_value()}
                style="position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 1000; display: flex; align-items: center; justify-content: center;"
            >
                // FocusScope traps focus within the modal and restores it on close
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    // Modal - spread both modal_props and content_props
                    <div
                        {..modal_props.get_value()}
                        {..content_props.get_value()}
                        aria-labelledby="basic-modal-title"
                        aria-describedby="basic-modal-description"
                        style="background: white; padding: 2em; border-radius: 12px; max-width: 400px; width: 90%; box-shadow: 0 4px 20px rgba(0,0,0,0.3);"
                    >
                        <h2 id="basic-modal-title" style="margin: 0 0 0.5em 0; color: #333;">
                            "Basic Modal"
                        </h2>
                        <p id="basic-modal-description" style="margin: 0 0 1.5em 0; color: #666;">
                            "This modal has scroll prevention enabled. Try scrolling the page - it won't work! "
                            "Press Escape or click outside to close."
                        </p>
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                            <button
                                on:click=move |_| close.run(())
                                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=move |_| close.run(())
                                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; background: var(--brand-color); color: white; border: none;"
                            >
                                "Confirm"
                            </button>
                        </Stack>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}

/// Alert dialog demo with AlertDialog role.
/// Composes use_modal (for dismiss behavior) + use_dialog (for ARIA semantics + focus).
#[component]
fn AlertDialogDemo() -> impl IntoView {
    let UseModalStateReturn {
        is_open,
        open,
        close,
        toggle: _,
    } = use_modal_state(false);

    // Behavior layer - non-dismissable for alert dialogs
    let UseModalReturn { modal_props, id: _ } = use_modal(UseModalInput {
        is_open,
        on_close: Some(close),
        is_dismissable: false, // AlertDialogs typically aren't dismissable with Escape
        should_close_on_interact_outside: false,
        is_keyboard_dismiss_disabled: true,
    });

    // Dialog layer - ARIA labeling, alertdialog role, focus on mount
    let UseDialogReturn {
        dialog_props,
        title_props,
        description_props,
        dialog_id: _,
    } = use_dialog(UseDialogInput {
        title: Some("Delete Item".to_string()),
        description: Some("This action cannot be undone. Are you sure?".to_string()),
        aria_label: None,
        role: DialogRole::AlertDialog,
    });

    // Backdrop layer
    let UseModalBackdropReturn {
        backdrop_props,
        content_props,
        backdrop_id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: Some(close),
        should_close_on_interact_outside: false, // Don't close on backdrop click
        prevent_scroll: true,
    });

    let modal_props = StoredValue::new(modal_props);
    let dialog_props = StoredValue::new(dialog_props);
    let content_props = StoredValue::new(content_props);
    let backdrop_props = StoredValue::new(backdrop_props);
    let title_props = StoredValue::new(title_props);
    let description_props = StoredValue::new(description_props);

    view! {
        <button
            on:click=move |_| open.run(())
            style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: #dc3545; color: white; border: none; font-size: 1em;"
        >
            "Delete Item (Alert Dialog)"
        </button>

        <Show when=move || is_open.get()>
            <div
                {..backdrop_props.get_value()}
                style="position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 1000; display: flex; align-items: center; justify-content: center;"
            >
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_props.get_value()}
                        {..dialog_props.get_value()}
                        {..content_props.get_value()}
                        style="background: white; padding: 2em; border-radius: 12px; max-width: 400px; width: 90%; box-shadow: 0 4px 20px rgba(0,0,0,0.3);"
                    >
                        <h2 id={title_props.get_value().id} style="margin: 0 0 0.5em 0; color: #dc3545;">
                            "Delete Item"
                        </h2>
                        <p id={description_props.get_value().id} style="margin: 0 0 1.5em 0; color: #666;">
                            "This action cannot be undone. Are you sure you want to delete this item?"
                        </p>
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                            <button
                                on:click=move |_| close.run(())
                                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=move |_| close.run(())
                                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; background: #dc3545; color: white; border: none;"
                            >
                                "Delete"
                            </button>
                        </Stack>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}

/// Non-dismissable modal demo
#[component]
fn NonDismissableModalDemo() -> impl IntoView {
    let UseModalStateReturn {
        is_open,
        open,
        close,
        toggle: _,
    } = use_modal_state(false);

    // Behavior layer - non-dismissable configuration
    let UseModalReturn { modal_props, id: _ } = use_modal(UseModalInput {
        is_open,
        on_close: Some(close),
        is_dismissable: false,                   // Can't dismiss with Escape
        should_close_on_interact_outside: false, // Can't dismiss by clicking outside
        is_keyboard_dismiss_disabled: true,      // Extra flag to disable keyboard dismiss
    });

    // Backdrop layer - also disable backdrop click
    let UseModalBackdropReturn {
        backdrop_props,
        content_props,
        backdrop_id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: Some(close),
        should_close_on_interact_outside: false, // Don't close on backdrop click
        prevent_scroll: true,
    });

    let modal_props = StoredValue::new(modal_props);
    let content_props = StoredValue::new(content_props);
    let backdrop_props = StoredValue::new(backdrop_props);

    view! {
        <button
            on:click=move |_| open.run(())
            style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: #6c757d; color: white; border: none; font-size: 1em;"
        >
            "Open Non-Dismissable Modal"
        </button>

        <Show when=move || is_open.get()>
            <div
                {..backdrop_props.get_value()}
                style="position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 1000; display: flex; align-items: center; justify-content: center;"
            >
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_props.get_value()}
                        {..content_props.get_value()}
                        aria-labelledby="non-dismissable-title"
                        aria-describedby="non-dismissable-description"
                        style="background: white; padding: 2em; border-radius: 12px; max-width: 400px; width: 90%; box-shadow: 0 4px 20px rgba(0,0,0,0.3);"
                    >
                        <h2 id="non-dismissable-title" style="margin: 0 0 0.5em 0; color: #333;">
                            "Non-Dismissable Modal"
                        </h2>
                        <p id="non-dismissable-description" style="margin: 0 0 1.5em 0; color: #666;">
                            "This modal cannot be closed by pressing Escape or clicking outside. "
                            "You must click the button below to close it."
                        </p>
                        <button
                            on:click=move |_| close.run(())
                            style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; background: var(--brand-color); color: white; border: none;"
                        >
                            "I Understand"
                        </button>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}

/// Confirmation dialog demo using use_dialog_state.
/// Composes use_modal (for dismiss behavior) + use_dialog (for ARIA semantics + focus).
#[component]
fn ConfirmationDialogDemo() -> impl IntoView {
    // Use dialog state with confirmation tracking
    let UseDialogStateReturn {
        is_open,
        open,
        close,
        confirm,
        is_confirmed,
    } = use_dialog_state(false);

    // Track the last action result
    let (last_result, set_last_result) = signal::<Option<bool>>(None);

    // Update last_result when dialog closes
    Effect::new(move || {
        if !is_open.get() && last_result.get().is_none() {
            // Dialog just closed, check confirmation status
            set_last_result.set(Some(is_confirmed.get()));
        }
    });

    // Behavior layer - dismissable confirmation dialog
    let UseModalReturn { modal_props, id: _ } = use_modal(UseModalInput {
        is_open,
        on_close: Some(close),
        is_dismissable: true,
        should_close_on_interact_outside: true,
        is_keyboard_dismiss_disabled: false,
    });

    // Dialog layer - ARIA labeling and focus on mount
    let UseDialogReturn {
        dialog_props,
        title_props,
        description_props,
        dialog_id: _,
    } = use_dialog(UseDialogInput {
        title: Some("Confirm Action".to_string()),
        description: Some("Do you want to proceed with this action?".to_string()),
        aria_label: None,
        role: DialogRole::Dialog,
    });

    let UseModalBackdropReturn {
        backdrop_props,
        content_props,
        backdrop_id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: Some(close),
        should_close_on_interact_outside: true,
        prevent_scroll: true,
    });

    let modal_props = StoredValue::new(modal_props);
    let dialog_props = StoredValue::new(dialog_props);
    let content_props = StoredValue::new(content_props);
    let backdrop_props = StoredValue::new(backdrop_props);
    let title_props = StoredValue::new(title_props);
    let description_props = StoredValue::new(description_props);

    view! {
        <div style="display: flex; gap: 1em; align-items: center;">
            <button
                on:click=move |_| {
                    set_last_result.set(None);
                    open.run(());
                }
                style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: var(--brand-color); color: white; border: none; font-size: 1em;"
            >
                "Open Confirmation Dialog"
            </button>

            <span style="color: #666;">
                {move || match last_result.get() {
                    None => "No action taken yet".to_string(),
                    Some(true) => "Confirmed!".to_string(),
                    Some(false) => "Cancelled".to_string(),
                }}
            </span>
        </div>

        <Show when=move || is_open.get()>
            <div
                {..backdrop_props.get_value()}
                style="position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 1000; display: flex; align-items: center; justify-content: center;"
            >
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_props.get_value()}
                        {..dialog_props.get_value()}
                        {..content_props.get_value()}
                        style="background: white; padding: 2em; border-radius: 12px; max-width: 400px; width: 90%; box-shadow: 0 4px 20px rgba(0,0,0,0.3);"
                    >
                        <h2 id={title_props.get_value().id} style="margin: 0 0 0.5em 0; color: #333;">
                            "Confirm Action"
                        </h2>
                        <p id={description_props.get_value().id} style="margin: 0 0 1.5em 0; color: #666;">
                            "Do you want to proceed with this action? "
                            "Click Confirm to accept or Cancel to decline."
                        </p>
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                            <button
                                on:click=move |_| {
                                    set_last_result.set(Some(false));
                                    close.run(());
                                }
                                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=move |_| {
                                    set_last_result.set(Some(true));
                                    confirm.run(());
                                }
                                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; background: var(--brand-color); color: white; border: none;"
                            >
                                "Confirm"
                            </button>
                        </Stack>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}
