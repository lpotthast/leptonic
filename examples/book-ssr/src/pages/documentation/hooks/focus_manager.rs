use indoc::indoc;
use leptonic::{atoms::prelude::FocusScope, components::prelude::*, hooks::*, prelude::Size};
use leptos::{prelude::*, web_sys};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[component]
pub fn PageUseFocusManager() -> impl IntoView {
    let (wrap, set_wrap) = signal(true);
    let (tabbable_only, set_tabbable_only) = signal(false);

    // Track the last focused element within the scope.
    // This is needed because when control buttons are clicked, they become
    // document.activeElement, which is outside the scope.
    let last_focused: StoredValue<Option<SendWrapper<web_sys::Element>>> = StoredValue::new(None);

    let UseFocusManagerReturn {
        focus_manager,
        props,
    } = use_focus_manager(UseFocusManagerInput::default());

    // Helper to build options from current signal state.
    let build_opts = {
        let last = last_focused;
        move || {
            let from = last.with_value(|el| el.as_ref().map(|sw| sw.clone().take()));
            FocusManagerOptions {
                from,
                wrap: wrap.get_untracked(),
                tabbable: tabbable_only.get_untracked(),
                ..Default::default()
            }
        }
    };

    // Helper to store a focus result.
    let store_result = move |result: Option<web_sys::Element>| {
        if let Some(el) = result {
            last_focused.set_value(Some(SendWrapper::new(el)));
        }
    };

    view! {
        <Article>
            <h1 id="use_focus_manager" class="anchor">
                "use_focus_manager"
                <AnchorLink href="#use_focus_manager" description="Direct link to article header"/>
            </h1>

            <p>"Programmatically navigate focus within a container. Provides methods to move focus to next, previous, first, or last focusable element."</p>

            <h2 id="basic-usage" class="anchor">
                "Basic Usage"
                <AnchorLink href="#basic-usage" description="Direct link to basic usage"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseFocusManagerReturn { focus_manager, props } =
                        use_focus_manager(UseFocusManagerInput::default());

                    // Move to the next focusable element:
                    focus_manager.focus_next(FocusManagerOptions::default());

                    view! {
                        <div {..props.into_attrs()}>
                            <button>"First"</button>
                            <button>"Second"</button>
                            <button>"Third"</button>
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Use the control buttons to move focus within the scope container:"</p>

            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5) attr:style="margin-bottom: 1em;">
                <button
                    style=demo_button()
                    on:click={
                        let fm = focus_manager.clone();
                        let opts = build_opts;
                        move |_| store_result(fm.focus_first(opts()))
                    }
                >
                    "Focus First"
                </button>
                <button
                    style=demo_button()
                    on:click={
                        let fm = focus_manager.clone();
                        let opts = build_opts;
                        move |_| store_result(fm.focus_previous(opts()))
                    }
                >
                    "Focus Previous"
                </button>
                <button
                    style=demo_button()
                    on:click={
                        let fm = focus_manager.clone();
                        let opts = build_opts;
                        move |_| store_result(fm.focus_next(opts()))
                    }
                >
                    "Focus Next"
                </button>
                <button
                    style=demo_button()
                    on:click={
                        let fm = focus_manager.clone();
                        let opts = build_opts;
                        move |_| store_result(fm.focus_last(opts()))
                    }
                >
                    "Focus Last"
                </button>
            </Stack>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(0.5) attr:style="margin-bottom: 1em;">
                <FormControl attr:style=form_control_row()>
                    <Checkbox checked=wrap set_checked=set_wrap />
                    <Label>"Wrap around"</Label>
                </FormControl>
                <FormControl attr:style=form_control_row()>
                    <Checkbox checked=tabbable_only set_checked=set_tabbable_only />
                    <Label>"Tabbable only (tabindex >= 0)"</Label>
                </FormControl>
            </Stack>

            <style>
                {format!(
                    ".focus-scope-demo button:focus, .focus-scope-demo input:focus {{ {} }}",
                    FOCUS_OUTLINE_CSS
                )}
            </style>

            <div
                {..props.into_attrs()}
                class="focus-scope-demo"
                style=demo_container_active()
                on:focusin=move |ev| {
                    if let Some(target) = ev.target() {
                        if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                            last_focused.set_value(Some(SendWrapper::new(el.clone())));
                        }
                    }
                }
            >
                <p style=demo_container_title()>"Focus Scope Container"</p>
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                    <button style=demo_button_solid()>
                        "Button 1"
                    </button>
                    <button style=demo_button_solid()>
                        "Button 2"
                    </button>
                    <input
                        type="text"
                        placeholder="Input field"
                        style=demo_input_solid()
                    />
                    <button style=demo_button_solid()>
                        "Button 3"
                    </button>
                </Stack>
            </div>

            <h2 id="tabbable" class="anchor">
                "Tabbable Option"
                <AnchorLink href="#tabbable" description="Direct link to tabbable option"/>
            </h2>

            <p>"By default, " <code>"focus_next"</code> "/" <code>"focus_previous"</code> " navigate to all focusable elements including those with " <code>"tabindex=\"-1\""</code> ". Set " <code>"tabbable: true"</code> " to restrict navigation to elements with " <code>"tabindex >= 0"</code> " (those reachable via Tab):"</p>

            <Code>
                {indoc!(r"
                    focus_manager.focus_next(FocusManagerOptions {
                        tabbable: true, // Skip elements with tabindex=-1
                        ..Default::default()
                    });
                ")}
            </Code>

            <p>"Toggle the \"Tabbable only\" checkbox in the demo above to see the difference."</p>

            <h2 id="accept" class="anchor">
                "Custom Filter (accept)"
                <AnchorLink href="#accept" description="Direct link to custom filter"/>
            </h2>

            <p>"The " <code>"accept"</code> " option takes a filter function to skip specific elements during navigation. This is useful when you need to exclude certain elements programmatically:"</p>

            <Code>
                {indoc!(r#"
                    use std::sync::Arc;

                    focus_manager.focus_next(FocusManagerOptions {
                        accept: Some(Arc::new(|el: &web_sys::Element| {
                            // Skip disabled-looking elements
                            !el.class_list().contains("skip-focus")
                        })),
                        ..Default::default()
                    });
                "#)}
            </Code>

            <h2 id="focus-trapping" class="anchor">
                "Focus Trapping with FocusScope"
                <AnchorLink href="#focus-trapping" description="Direct link to focus trapping"/>
            </h2>

            <p>"The " <code>"use_focus_manager"</code> " hook provides " <em>"programmatic"</em> " focus control only. It does not trap focus or intercept Tab key presses. For focus trapping (preventing Tab from leaving the container), use the " <code>"FocusScope"</code> " component which combines focus management with keyboard event handling."</p>

            <Code>
                {indoc!(r#"
                    <FocusScope contain=true auto_focus=true restore_focus=true>
                        <button>"First"</button>
                        <input type="text" placeholder="Middle" />
                        <button>"Last"</button>
                    </FocusScope>
                "#)}
            </Code>

            <p>"Try tabbing through the container below. Focus will wrap from the last element back to the first, and vice versa with Shift+Tab:"</p>

            <style>
                ".focus-trap-demo button:focus, .focus-trap-demo input:focus {
                    outline: 3px solid #4a9eff;
                    outline-offset: 2px;
                }"
            </style>

            <FocusScope contain=true>
                <div
                    class="focus-trap-demo"
                    style="
                        border: 3px solid #4a9eff;
                        padding: 1.5em;
                        border-radius: 8px;
                        background: rgba(74, 158, 255, 0.1);
                    "
                >
                    <p style=demo_container_title()>"Focus Trap Container (Tab cycles within)"</p>
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                        <button style=demo_button_solid()>
                            "Trapped 1"
                        </button>
                        <button style=demo_button_solid()>
                            "Trapped 2"
                        </button>
                        <input
                            type="text"
                            placeholder="Trapped input"
                            style=demo_input_solid()
                        />
                        <button style=demo_button_solid()>
                            "Trapped 3"
                        </button>
                    </Stack>
                </div>
            </FocusScope>

            <h3>"FocusScope Props"</h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Prop"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"contain"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, Tab/Shift+Tab navigation wraps within the scope."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"auto_focus"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, focuses the first focusable element on mount."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"restore_focus"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, restores focus to the previously focused element when unmounted."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusManagerInput"</code> " is an empty struct — all configuration is passed per-call via " <code>"FocusManagerOptions"</code> ":"</p>

            <h3>"FocusManagerOptions"</h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"from"</code></TableCell>
                            <TableCell><code>"Option<web_sys::Element>"</code></TableCell>
                            <TableCell><code>"document.activeElement"</code></TableCell>
                            <TableCell>"Element to start navigation from."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"wrap"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Whether to wrap around when reaching the end/beginning."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"tabbable"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Only consider elements with tabindex >= 0."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"accept"</code></TableCell>
                            <TableCell><code>"Option<Arc<dyn Fn(&Element) -> bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Custom filter function to skip specific elements."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusManagerReturn"</code> " fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"focus_manager"</code></TableCell>
                            <TableCell><code>"FocusManager"</code></TableCell>
                            <TableCell>"Provides " <code>"focus_next"</code> ", " <code>"focus_previous"</code> ", " <code>"focus_first"</code> ", and " <code>"focus_last"</code> " methods."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseFocusManagerProps"</code></TableCell>
                            <TableCell>"Spread onto the container element via " <code>"props.into_attrs()"</code> " to define the scope boundary."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Navigate to next/previous/first/last focusable element."</li>
                <li>"Respects tabindex order (positive tabindex elements come first)."</li>
                <li>"Filters out hidden and disabled elements."</li>
                <li>"Optional wrap-around, tabbable-only, and custom filter support."</li>
                <li>"Automatic element capture via prop spreading."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_manager", link: "#use_focus_manager" },
                Toc::Leaf { title: "Basic Usage", link: "#basic-usage" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "Tabbable Option", link: "#tabbable" },
                Toc::Leaf { title: "Custom Filter (accept)", link: "#accept" },
                Toc::Leaf { title: "Focus Trapping", link: "#focus-trapping" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
