use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::hooks::{PlacementX, PlacementY};
use leptonic::prelude::Size;
use leptonic::utils::locale::WritingDirection;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use std::marker::PhantomData;

#[component]
pub fn PageUsePopoverHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="popover" class="anchor">
                "use_popover"
                <AnchorLink href="#popover" description="Direct link to article header"/>
            </h1>

            <p>"A composition hook that provides behavior and accessibility implementation for a popover component. "
               "A popover is an overlay element positioned relative to a trigger."</p>

            <h2 id="hook-composition" class="anchor">
                "Hook Composition"
                <AnchorLink href="#hook-composition" description="Direct link to hook composition"/>
            </h2>

            <p><code>"use_popover"</code>" is a composition hook that combines several lower-level primitives:"</p>

            <ul>
                <li><code>"use_overlay_position"</code>" - Positions the popover relative to the trigger element"</li>
                <li><code>"use_prevent_scroll"</code>" - Prevents page scrolling when the popover is open (unless non-modal)"</li>
                <li>"Dismiss handling - Escape key and click outside detection"</li>
            </ul>

            <h2 id="basic-demo" class="anchor">
                "Basic Demo"
                <AnchorLink href="#basic-demo" description="Direct link to basic demo"/>
            </h2>

            <p>"Click the button to show a popover positioned below it:"</p>

            <BasicPopoverDemo />

            <h2 id="placement-demo" class="anchor">
                "Placement Demo"
                <AnchorLink href="#placement-demo" description="Direct link to placement demo"/>
            </h2>

            <p>"Popovers can be positioned in various locations relative to the trigger:"</p>

            <PlacementPopoverDemo />

            <h2 id="non-modal-demo" class="anchor">
                "Non-Modal Demo"
                <AnchorLink href="#non-modal-demo" description="Direct link to non-modal demo"/>
            </h2>

            <p>"A non-modal popover allows interaction with elements outside while open, "
               "and does not prevent page scrolling:"</p>

            <NonModalPopoverDemo />

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API"/>
            </h2>

            <h3 id="input" class="anchor">
                "UsePopoverInput"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h3>

            <Code>
                {indoc!(r"
                    pub struct UsePopoverInput<Trigger, Popover, M> {
                        /// Element ref for the trigger (what the popover positions relative to)
                        pub trigger_ref: Trigger,

                        /// Element ref for the popover itself
                        pub popover_ref: Popover,

                        /// Whether the popover is currently open
                        pub is_open: Signal<bool>,

                        /// Called when the popover should close
                        pub on_close: Callback<()>,

                        /// Horizontal placement (OuterLeft, Left, Center, Right, OuterRight)
                        pub placement_x: Signal<PlacementX>,

                        /// Vertical placement (Above, Top, Center, Bottom, Below)
                        pub placement_y: Signal<PlacementY>,

                        /// Writing direction for logical placement
                        pub writing_direction: Signal<WritingDirection>,

                        /// Non-modal popovers allow outside interaction
                        pub is_non_modal: bool,

                        /// Disable Escape key dismissal
                        pub is_keyboard_dismiss_disabled: bool,

                        /// Disable scroll prevention
                        pub is_scroll_prevention_disabled: bool,
                    }
                ")}
            </Code>

            <h3 id="return" class="anchor">
                "UsePopoverReturn"
                <AnchorLink href="#return" description="Direct link to return"/>
            </h3>

            <Code>
                {indoc!(r"
                    pub struct UsePopoverReturn {
                        /// Props for the popover element (positioning + keydown handler)
                        pub popover_props: UsePopoverProps,

                        /// Props for an optional backdrop element (click handler)
                        pub backdrop_props: UsePopoverBackdropProps,
                    }
                ")}
            </Code>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to when to use"/>
            </h2>

            <p><code>"use_popover"</code>" is a positioning and dismiss hook only. Choose the right hook based on your needs:"</p>

            <table style="width: 100%; border-collapse: collapse;">
                <thead>
                    <tr style="border-bottom: 2px solid var(--brand-color);">
                        <th style="text-align: left; padding: 0.5em;">"Use Case"</th>
                        <th style="text-align: left; padding: 0.5em;">"Hook(s)"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Simple info popover, tooltip, profile card"</td>
                        <td style="padding: 0.5em;"><code>"use_popover"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Menu with keyboard navigation"</td>
                        <td style="padding: 0.5em;"><code>"use_menu"</code>" + "<code>"use_menu_trigger"</code>" + "<code>"use_menu_item"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Dropdown with selection"</td>
                        <td style="padding: 0.5em;"><code>"use_select"</code>" or "<code>"use_menu"</code>" hooks"</td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Autocomplete/combobox"</td>
                        <td style="padding: 0.5em;"><code>"use_combobox"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Modal dialog"</td>
                        <td style="padding: 0.5em;"><code>"use_modal"</code>" + "<code>"use_modal_backdrop"</code></td>
                    </tr>
                </tbody>
            </table>

            <p style="margin-top: 1em;">
                <strong>"Key distinction:"</strong>
                " For menus with arrow key navigation and ARIA menu roles, use the "
                <code>"use_menu"</code>" family of hooks instead of "<code>"use_popover"</code>"."
            </p>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Automatic positioning relative to trigger element"</li>
                <li>"Escape key dismissal"</li>
                <li>"Click outside detection"</li>
                <li>"Optional backdrop element"</li>
                <li>"Scroll prevention (configurable)"</li>
                <li>"Non-modal mode for allowing outside interaction"</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "React Aria Deviations"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <h3>"Omitted Features"</h3>
            <ul>
                <li><code>"arrowRef"</code>"/"<code>"arrowProps"</code>" - Arrow element positioning not built-in"</li>
                <li><code>"groupRef"</code>" - Submenu-style popover groups not implemented"</li>
                <li><code>"shouldCloseOnInteractOutside"</code>" - Callback filter not implemented"</li>
                <li><code>"placement"</code>" return - Computed placement not returned"</li>
            </ul>

            <h3>"API Differences"</h3>
            <ul>
                <li><code>"underlayProps"</code>" renamed to "<code>"backdrop_props"</code>" for consistency with "<code>"use_modal_backdrop"</code></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_popover", link: "#popover" },
                Toc::Leaf { title: "Hook Composition", link: "#hook-composition" },
                Toc::Leaf { title: "Basic Demo", link: "#basic-demo" },
                Toc::Leaf { title: "Placement Demo", link: "#placement-demo" },
                Toc::Leaf { title: "Non-Modal Demo", link: "#non-modal-demo" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "React Aria Deviations", link: "#deviations" },
            ]
        }/>
    }
}

/// Basic popover demo
#[component]
fn BasicPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let trigger_el: NodeRef<html::Button> = NodeRef::new();
    let popover_el: NodeRef<html::Div> = NodeRef::new();

    let UsePopoverReturn {
        popover_props,
        backdrop_props,
    } = use_popover(UsePopoverInput {
        trigger_ref: trigger_el,
        popover_ref: popover_el,
        is_open: is_open.into(),
        on_close: Callback::new(move |_| set_is_open.set(false)),
        placement_x: Signal::derive(|| PlacementX::Center),
        placement_y: Signal::derive(|| PlacementY::Below),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        is_non_modal: false,
        is_keyboard_dismiss_disabled: false,
        is_scroll_prevention_disabled: false,
        phantom_data: PhantomData,
    });

    let popover_props = StoredValue::new(popover_props.into_attrs());
    let backdrop_props = StoredValue::new(backdrop_props.into_attrs());

    view! {
        <div style="display: flex; justify-content: center; padding: 2em;">
            <button
                node_ref=trigger_el
                on:click=move |_| set_is_open.set(!is_open.get())
                style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: var(--brand-color); color: white; border: none; font-size: 1em;"
            >
                {move || if is_open.get() { "Close Popover" } else { "Open Popover" }}
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                // Optional backdrop - click to close
                <div
                    {..backdrop_props.get_value()}
                    style="position: fixed; inset: 0; z-index: 999;"
                />
                // Popover content
                <div
                    {..popover_props.get_value()}
                    node_ref=popover_el
                    style="
                        background: white;
                        border: 1px solid #ccc;
                        border-radius: 8px;
                        padding: 1em;
                        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                        z-index: 1000;
                        max-width: 300px;
                    "
                >
                    <h4 style="margin: 0 0 0.5em 0; color: #333;">"Popover Title"</h4>
                    <p style="margin: 0; color: #666;">
                        "This is popover content. Press Escape or click outside to close."
                    </p>
                </div>
            </Show>
        </Portal>
    }
}

/// Placement demo showing different positions
#[component]
fn PlacementPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (placement_x, set_placement_x) = signal(PlacementX::Center);
    let (placement_y, set_placement_y) = signal(PlacementY::Below);

    let trigger_el: NodeRef<html::Button> = NodeRef::new();
    let popover_el: NodeRef<html::Div> = NodeRef::new();

    let UsePopoverReturn {
        popover_props,
        backdrop_props,
    } = use_popover(UsePopoverInput {
        trigger_ref: trigger_el,
        popover_ref: popover_el,
        is_open: is_open.into(),
        on_close: Callback::new(move |_| set_is_open.set(false)),
        placement_x: placement_x.into(),
        placement_y: placement_y.into(),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        is_non_modal: false,
        is_keyboard_dismiss_disabled: false,
        is_scroll_prevention_disabled: false,
        phantom_data: PhantomData,
    });

    let popover_props = StoredValue::new(popover_props.into_attrs());
    let backdrop_props = StoredValue::new(backdrop_props.into_attrs());

    view! {
        <Grid gap=Size::Em(0.5) attr:style="margin-bottom: 1em;">
            <Row>
                <Col xs=6 attr:style="
                    border: 0.1em solid lightgrey;
                    border-radius: 0.25em;
                    padding: 0.5em;
                ">
                    <strong>"Horizontal"</strong>
                    <RadioGroup attr:style="display: flex; flex-direction: column; gap: 0.2em; width: 100%; margin-top: 0.5em;">
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::OuterLeft)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::OuterLeft)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"OuterLeft"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::Left)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::Left)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Left"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::Center)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::Center)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Center"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::Right)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::Right)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Right"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::OuterRight)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::OuterRight)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"OuterRight"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
                <Col xs=6 attr:style="
                    border: 0.1em solid lightgrey;
                    border-radius: 0.25em;
                    padding: 0.5em;
                ">
                    <strong>"Vertical"</strong>
                    <RadioGroup attr:style="display: flex; flex-direction: column; gap: 0.2em; width: 100%; margin-top: 0.5em;">
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Above)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Above)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Above"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Top)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Top)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Top"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Center)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Center)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Center"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Bottom)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Bottom)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Bottom"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Below)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Below)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Below"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
            </Row>
        </Grid>

        <div style="display: flex; justify-content: center; padding: 4em;">
            <button
                node_ref=trigger_el
                on:click=move |_| set_is_open.set(!is_open.get())
                style="padding: 1em 2em; border-radius: 8px; cursor: pointer; background: var(--brand-color); color: white; border: none; font-size: 1em;"
            >
                {move || if is_open.get() { "Close" } else { "Open Popover" }}
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..backdrop_props.get_value()}
                    style="position: fixed; inset: 0; z-index: 999;"
                />
                <div
                    {..popover_props.get_value()}
                    node_ref=popover_el
                    style="
                        background: white;
                        border: 1px solid #ccc;
                        border-radius: 8px;
                        padding: 1em;
                        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                        z-index: 1000;
                    "
                >
                    <p style="margin: 0; color: #666;">
                        {move || format!("Placement: {:?} / {:?}", placement_x.get(), placement_y.get())}
                    </p>
                </div>
            </Show>
        </Portal>
    }
}

/// Non-modal popover demo
#[component]
fn NonModalPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (counter, set_counter) = signal(0);

    let trigger_el: NodeRef<html::Button> = NodeRef::new();
    let popover_el: NodeRef<html::Div> = NodeRef::new();

    let UsePopoverReturn {
        popover_props,
        backdrop_props: _,
    } = use_popover(UsePopoverInput {
        trigger_ref: trigger_el,
        popover_ref: popover_el,
        is_open: is_open.into(),
        on_close: Callback::new(move |_| set_is_open.set(false)),
        placement_x: Signal::derive(|| PlacementX::OuterRight),
        placement_y: Signal::derive(|| PlacementY::Top),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        is_non_modal: true, // Allow interaction outside
        is_keyboard_dismiss_disabled: false,
        is_scroll_prevention_disabled: true, // Don't prevent scroll
        phantom_data: PhantomData,
    });

    let popover_props = StoredValue::new(popover_props.into_attrs());

    view! {
        <div style="display: flex; gap: 1em; align-items: center; justify-content: center; padding: 2em;">
            <button
                node_ref=trigger_el
                on:click=move |_| set_is_open.set(!is_open.get())
                style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: var(--brand-color); color: white; border: none; font-size: 1em;"
            >
                {move || if is_open.get() { "Close" } else { "Open Non-Modal" }}
            </button>

            <button
                on:click=move |_| set_counter.update(|c| *c += 1)
                style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; border: 1px solid #ccc; font-size: 1em;"
            >
                {move || format!("Counter: {}", counter.get())}
            </button>
        </div>

        <p style="text-align: center; color: #666;">
            "Notice: You can still click the counter button while the popover is open!"
        </p>

        <Portal>
            <Show when=move || is_open.get()>
                // No backdrop for non-modal popover
                <div
                    {..popover_props.get_value()}
                    node_ref=popover_el
                    style="
                        background: white;
                        border: 1px solid #ccc;
                        border-radius: 8px;
                        padding: 1em;
                        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                        z-index: 1000;
                        max-width: 200px;
                    "
                >
                    <p style="margin: 0; color: #666;">
                        "This is a non-modal popover. You can interact with elements outside!"
                    </p>
                </div>
            </Show>
        </Portal>
    }
}
