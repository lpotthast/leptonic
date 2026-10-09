use leptonic::{
    CapturedElement, I18nProvider, Locale,
    atoms::{button::Button, dialog::DialogTrigger, overlay_arrow::OverlayArrow, popover::Popover},
    hooks::overlay::{Placement, PopoverModality, Rect},
    leptos_classes::Classes,
};
use leptos::prelude::*;

use crate::pages::{Section, prevent_focus_steal};

/// The trigger `#test-opo-<name>-trigger` (60x30) of a popover (the children), at a fixed place in
/// the viewport: `left`/`top` (px).
#[component]
fn Positioned(
    name: &'static str,
    #[prop(into)] left: Signal<f64>,
    top: f64,
    /// Classes of the trigger besides `test-opo-trigger`.
    #[prop(into, optional)]
    trigger_classes: Classes,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div
            style:position="fixed"
            style:left=move || format!("{}px", left.get())
            style:top=format!("{top}px")
        >
            <DialogTrigger>
                <Button
                    id=format!("test-opo-{name}-trigger")
                    classes=trigger_classes.clone().add("test-opo-trigger")
                >
                    {name}
                </Button>
                {children()}
            </DialogTrigger>
        </div>
    }
}

/// The options of `use_overlay_position` through the `Popover` atom (react-aria's
/// `useOverlayPosition.test.tsx` and `calculatePosition.test.ts`), one per section, each trigger at a
/// fixed place of the 800x600 viewport:
/// - `offset`: placed below with an offset of 0, 20 after `#test-opo-offset-toggle`.
/// - `resize`: placed below; `#test-opo-move` moves the trigger 100px right (no resize observer
///   sees it; a window `resize` event does).
/// - `max-height`: a 200px high content in a popover with `max_height` 50.
/// - `margin`: a trigger with a 20px margin; the popover below it.
/// - `cross-offset`: placed below with a cross offset of 30.
/// - `rtl`: in a right-to-left subtree, one popover at `Start` and one at `BottomStart`.
/// - `target-rect`: placed below a 0x0 target rectangle at (400, 100) instead of the trigger.
/// - `arrow`: a 20px wide trigger at the viewport's left edge, its 200px popover below with an
///   arrow (12px wide) kept 30px from the popover's edges.
/// - `boundary`: a boundary box (`#test-opo-boundary`, x 300..600) around the trigger at its left
///   edge; the 240px popover stays within it (12px padding).
#[component]
pub fn PageAtomOverlayPositionOptions() -> impl IntoView {
    let offset = RwSignal::new(0.0);
    let moved = RwSignal::new(false);
    let rtl: Locale = "ar-EG".parse().expect("a valid locale");
    let boundary = CapturedElement::new();

    view! {
        <h1>"Overlay position options"</h1>
        <style>
            ".test-opo-trigger { width: 60px; height: 30px; padding: 0; box-sizing: border-box; }
            .test-opo-narrow { width: 20px; }
            .test-opo-margin { margin: 20px; }"
        </style>
        <Section name="offset">
            <button
                id="test-opo-offset-toggle"
                on:mousedown=prevent_focus_steal
                on:click=move |_| offset.set(20.0)
            >
                "Offset 20"
            </button>
            <Positioned name="offset" left=300.0 top=200.0>
                <Popover
                    placement=Placement::Bottom
                    offset=offset
                    modality=PopoverModality::NonModal
                    classes="test-opo-offset-popover"
                >
                    <div style="width: 160px; height: 40px;">"Offset"</div>
                </Popover>
            </Positioned>
        </Section>
        <Section name="resize">
            <button
                id="test-opo-move"
                on:mousedown=prevent_focus_steal
                on:click=move |_| moved.set(true)
            >
                "Move"
            </button>
            <Positioned
                name="resize"
                left=Signal::derive(move || if moved.get() { 400.0 } else { 300.0 })
                top=200.0
            >
                <Popover
                    placement=Placement::Bottom
                    modality=PopoverModality::NonModal
                    classes="test-opo-resize-popover"
                >
                    <div style="width: 160px; height: 40px;">"Resize"</div>
                </Popover>
            </Positioned>
        </Section>
        <Section name="max-height">
            <Positioned name="max-height" left=300.0 top=100.0>
                <Popover
                    placement=Placement::Bottom
                    max_height=Some(50.0)
                    modality=PopoverModality::NonModal
                    classes="test-opo-max-height-popover"
                >
                    <div style="width: 160px; height: 200px;">"Tall"</div>
                </Popover>
            </Positioned>
        </Section>
        <Section name="margin">
            <Positioned name="margin" left=300.0 top=200.0 trigger_classes="test-opo-margin">
                <Popover
                    placement=Placement::Bottom
                    modality=PopoverModality::NonModal
                    classes="test-opo-margin-popover"
                >
                    <div style="width: 160px; height: 40px;">"Margin"</div>
                </Popover>
            </Positioned>
        </Section>
        <Section name="cross-offset">
            <Positioned name="cross-offset" left=300.0 top=200.0>
                <Popover
                    placement=Placement::Bottom
                    cross_offset=30.0
                    modality=PopoverModality::NonModal
                    classes="test-opo-cross-offset-popover"
                >
                    <div style="width: 160px; height: 40px;">"Cross offset"</div>
                </Popover>
            </Positioned>
        </Section>
        <Section name="rtl">
            <I18nProvider locale=rtl>
                <Positioned name="start" left=370.0 top=200.0>
                    <Popover
                        placement=Placement::Start
                        modality=PopoverModality::NonModal
                        classes="test-opo-start-popover"
                    >
                        <div style="width: 160px; height: 40px;">"Start"</div>
                    </Popover>
                </Positioned>
                <Positioned name="bottom-start" left=370.0 top=350.0>
                    <Popover
                        placement=Placement::BottomStart
                        modality=PopoverModality::NonModal
                        classes="test-opo-bottom-start-popover"
                    >
                        <div style="width: 160px; height: 40px;">"Bottom start"</div>
                    </Popover>
                </Positioned>
            </I18nProvider>
        </Section>
        <Section name="target-rect">
            <Positioned name="target-rect" left=100.0 top=400.0>
                <Popover
                    placement=Placement::Bottom
                    target_rect=Some(Rect {
                        top: 100.0,
                        left: 400.0,
                        width: 0.0,
                        height: 0.0,
                    })
                    modality=PopoverModality::NonModal
                    classes="test-opo-target-rect-popover"
                >
                    <div style="width: 160px; height: 40px;">"Target rect"</div>
                </Popover>
            </Positioned>
        </Section>
        <Section name="arrow">
            <div style="position: fixed; left: 0; top: 200px;">
                <DialogTrigger>
                    <Button
                        id="test-opo-arrow-trigger"
                        classes=["test-opo-trigger", "test-opo-narrow"]
                    >
                        "A"
                    </Button>
                    <Popover
                        placement=Placement::Bottom
                        arrow_boundary_offset=30.0
                        modality=PopoverModality::NonModal
                        classes="test-opo-arrow-popover"
                    >
                        <div style="width: 200px; height: 40px;">"Arrow"</div>
                        <OverlayArrow classes="test-opo-arrow">
                            <div style="width: 12px; height: 6px;"></div>
                        </OverlayArrow>
                    </Popover>
                </DialogTrigger>
            </div>
        </Section>
        <Section name="boundary">
            <div
                id="test-opo-boundary"
                style="position: fixed; left: 300px; top: 100px; width: 300px; height: 300px;"
                {..boundary.attr()}
            >
                <DialogTrigger>
                    <Button id="test-opo-boundary-trigger" classes="test-opo-trigger">
                        "Boundary"
                    </Button>
                    <Popover
                        placement=Placement::Bottom
                        boundary=boundary
                        modality=PopoverModality::NonModal
                        classes="test-opo-boundary-popover"
                    >
                        <div style="width: 240px; height: 40px;">"Boundary"</div>
                    </Popover>
                </DialogTrigger>
            </div>
        </Section>
    }
}
