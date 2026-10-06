//! Renders the segments of a date or time field. leptonic has no segment atom yet (tracked in its roadmap), so the
//! date field, time field and date picker demos share this Leptos component built from `use_date_segment`.
use leptonic::hooks::*;
use leptos::{html, prelude::*};

/// The segments and segment callbacks of `use_date_field` or `use_time_field`.
#[derive(Clone, Copy)]
pub struct SegmentControls {
    pub segments: Signal<Vec<DateSegment>>,
    pub focused_segment: Signal<Option<usize>>,
    pub focus_segment: Callback<usize>,
    pub focus_next: Callback<()>,
    pub focus_previous: Callback<()>,
    pub set_segment: Callback<(DateSegmentType, i32)>,
    pub clear_segment: Callback<DateSegmentType>,
    pub increment: Callback<DateSegmentType>,
    pub decrement: Callback<DateSegmentType>,
    pub increment_page: Callback<DateSegmentType>,
    pub decrement_page: Callback<DateSegmentType>,
    pub increment_to_max: Callback<DateSegmentType>,
    pub decrement_to_min: Callback<DateSegmentType>,
    pub confirm_placeholder: Callback<()>,
}

impl From<&UseDateFieldReturn> for SegmentControls {
    fn from(field: &UseDateFieldReturn) -> Self {
        Self {
            segments: field.segments,
            focused_segment: field.focused_segment,
            focus_segment: field.focus_segment,
            focus_next: field.focus_next,
            focus_previous: field.focus_previous,
            set_segment: field.set_segment,
            clear_segment: field.clear_segment,
            increment: field.increment,
            decrement: field.decrement,
            increment_page: field.increment_page,
            decrement_page: field.decrement_page,
            increment_to_max: field.increment_to_max,
            decrement_to_min: field.decrement_to_min,
            confirm_placeholder: field.confirm_placeholder,
        }
    }
}

impl From<&UseTimeFieldReturn> for SegmentControls {
    fn from(field: &UseTimeFieldReturn) -> Self {
        Self {
            segments: field.segments,
            focused_segment: field.focused_segment,
            focus_segment: field.focus_segment,
            focus_next: field.focus_next,
            focus_previous: field.focus_previous,
            set_segment: field.set_segment,
            clear_segment: field.clear_segment,
            increment: field.increment,
            decrement: field.decrement,
            increment_page: field.increment_page,
            decrement_page: field.decrement_page,
            increment_to_max: field.increment_to_max,
            decrement_to_min: field.decrement_to_min,
            confirm_placeholder: field.confirm_placeholder,
        }
    }
}

/// Renders the segments of a date or time field: an element per editable segment, wired to `use_date_segment`, and
/// the literal separators between them.
#[component]
pub fn DateSegments(
    controls: SegmentControls,
    #[prop(into)] is_disabled: Signal<bool>,
    #[prop(into)] is_read_only: Signal<bool>,
    #[prop(into)] is_invalid: Signal<bool>,
) -> impl IntoView {
    let SegmentControls {
        segments,
        focused_segment,
        focus_segment,
        ..
    } = controls;

    // Editing changes the segments' values, never which segments there are. Create the elements once, so that they
    // (and the digits `use_date_segment` buffers while you type) survive edits.
    let initial = segments.get_untracked();
    let node_refs = initial
        .iter()
        .map(|_| NodeRef::<html::Span>::new())
        .collect::<Vec<_>>();
    let elements = StoredValue::new(node_refs.clone());

    // The first editable segment is the field's tab stop until another segment receives focus.
    if focused_segment.get_untracked().is_none()
        && let Some(first) = initial.iter().position(|segment| segment.is_editable)
    {
        focus_segment.run(first);
    }

    // Arrow keys and auto-advance only move `focused_segment`. Move the browser focus along while the field has it.
    Effect::new(move |_| {
        let Some(index) = focused_segment.get() else {
            return;
        };
        elements.with_value(|elements| {
            let has_focus = elements
                .iter()
                .filter_map(NodeRef::get_untracked)
                .any(|element| element.matches(":focus").unwrap_or(false));
            if has_focus && let Some(element) = elements.get(index).and_then(NodeRef::get_untracked)
            {
                let _ = element.focus();
            }
        });
    });

    initial
        .into_iter()
        .zip(node_refs)
        .enumerate()
        .map(|(index, (segment, node_ref))| {
            // `use_date_segment` keeps the segment it was created with; read the current text from the field.
            let text = move || segments.with(|segments| segments.get(index).map(|s| s.text.clone()).unwrap_or_default());
            let is_placeholder = move || segments.with(|segments| segments.get(index).is_some_and(|s| s.is_placeholder));

            if !segment.is_editable {
                return view! {
                    <span node_ref=node_ref aria-hidden="true" class="demo-date-segment-literal">{text}</span>
                }
                .into_any();
            }

            let ty = segment.segment_type;
            let UseDateSegmentReturn { segment_props, .. } = use_date_segment(UseDateSegmentInput {
                segment,
                is_focused: Signal::derive(move || focused_segment.get() == Some(index)),
                is_disabled,
                is_read_only,
                on_change: Some(Callback::new(move |value| controls.set_segment.run((ty, value)))),
                on_increment: Some(Callback::new(move |()| controls.increment.run(ty))),
                on_decrement: Some(Callback::new(move |()| controls.decrement.run(ty))),
                on_focus_next: Some(controls.focus_next),
                on_focus_previous: Some(controls.focus_previous),
                on_clear: Some(Callback::new(move |()| controls.clear_segment.run(ty))),
                on_increment_page: Some(Callback::new(move |()| controls.increment_page.run(ty))),
                on_decrement_page: Some(Callback::new(move |()| controls.decrement_page.run(ty))),
                on_increment_to_max: Some(Callback::new(move |()| controls.increment_to_max.run(ty))),
                on_decrement_to_min: Some(Callback::new(move |()| controls.decrement_to_min.run(ty))),
                on_blur: Some(controls.confirm_placeholder),
                is_invalid,
            });

            view! {
                <span
                    {..segment_props.into_attrs()}
                    node_ref=node_ref
                    class="demo-date-segment"
                    data-placeholder=move || is_placeholder().then_some("true")
                    // Clicking a segment makes it the tab stop.
                    on:focus=move |_| focus_segment.run(index)
                >
                    {text}
                </span>
            }
            .into_any()
        })
        .collect_view()
}
