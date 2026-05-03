use leptos::prelude::*;

use crate::{
    Out,
    hooks::{UseCalendarStateInput, use_calendar_state},
    utils::{
        classes::Classes,
        styles::Styles,
        time::{GuideMode, InMonth},
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Selection {
    Year,
    Month,
    Day,
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn DateSelector(
    value: time::OffsetDateTime,
    #[prop(into)] on_change: Out<time::OffsetDateTime>,
    #[prop(optional)] min: Option<time::OffsetDateTime>,
    #[prop(optional)] max: Option<time::OffsetDateTime>,
    // TODO (new): guide_mode should not be a signal!
    #[prop(into, optional, default = GuideMode::CalendarFirst.into())] guide_mode: Signal<
        GuideMode,
    >,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let calendar = use_calendar_state(UseCalendarStateInput {
        default_value: Some(value),
        min,
        max,
        on_change: Some(Callback::new(move |val: Option<time::OffsetDateTime>| {
            if let Some(v) = val {
                on_change.set(v);
            }
        })),
        ..Default::default()
    });

    let (show, set_show) = signal(match guide_mode.get() {
        GuideMode::CalendarFirst => Selection::Day,
        GuideMode::YearFirst => Selection::Year,
    });

    // TODO: Support internationalization
    let (short_weekday_names, _) = signal(create_week_day_names());

    view! {
        <div class=classes.add("leptonic-datetime") style=styles>
            <div class="leptonic-date-selector">
                <div class="leptonic-calender-month">
                    <div class="actions">
                        {move || match show.get() {
                            Selection::Year => {
                                view! {
                                    <div
                                        on:click=move |_| calendar.navigate_years_backward.run(())
                                        class="previous arrow-left"
                                    ></div>
                                    <div
                                        on:click=move |_| {
                                            set_show.update(|show| *show = Selection::Month);
                                        }
                                        class="current-date"
                                    >
                                        {calendar.years_range}
                                    </div>
                                    <div
                                        on:click=move |_| calendar.navigate_years_forward.run(())
                                        class="next arrow-right"
                                    ></div>
                                }
                                    .into_any()
                            }
                            Selection::Month => {
                                view! {
                                    <div
                                        on:click=move |_| calendar.focus_previous_section.run(true)
                                        class="previous arrow-left"
                                    ></div>
                                    <div
                                        on:click=move |_| {
                                            set_show.update(|show| *show = Selection::Year);
                                        }
                                        class="current-date"
                                    >
                                        {move || calendar.focused_date.get().year()}
                                    </div>
                                    <div
                                        on:click=move |_| calendar.focus_next_section.run(true)
                                        class="next arrow-right"
                                    ></div>
                                }
                                    .into_any()
                            }
                            Selection::Day => {
                                view! {
                                    <div
                                        on:click=move |_| calendar.focus_previous_page.run(())
                                        class="previous arrow-left"
                                    ></div>
                                    <div
                                        on:click=move |_| {
                                            set_show.update(|show| *show = Selection::Year);
                                        }
                                        class="current-date"
                                    >
                                        {move || calendar.focused_month_name.get()}
                                        " "
                                        {move || calendar.focused_year.get()}
                                    </div>
                                    <div
                                        on:click=move |_| calendar.focus_next_page.run(())
                                        class="next arrow-right"
                                    ></div>
                                }
                                    .into_any()
                            }
                        }}
                    </div>

                    <Show when=move || show.get() == Selection::Year fallback=|| ()>
                        <div class="years">
                            <For
                                each=move || calendar.years.get()
                                key=|year| year.number
                                children=move |year| {
                                    view! {
                                        <div
                                            on:click=move |_| {
                                                if !year.disabled {
                                                    calendar.focus_year.run(year.number);
                                                    set_show.update(|show| *show = Selection::Month);
                                                }
                                            }
                                            class="year"
                                            class:is-staging=year.is_focused
                                            class:is-now=year.is_now
                                            class:disabled=year.disabled
                                        >
                                            {year.number}
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </Show>

                    <Show when=move || show.get() == Selection::Month fallback=|| ()>
                        <div class="months">
                            <For
                                each=move || calendar.months.get()
                                key=|month| month.index
                                children=move |month| {
                                    let month_index = month.index;
                                    view! {
                                        <div
                                            on:click=move |_| {
                                                if !month.disabled {
                                                    calendar.focus_month.run(month_index);
                                                    set_show.update(|show| *show = Selection::Day);
                                                }
                                            }
                                            class="month"
                                            class:is-staging=month.is_focused
                                            class:is-now=month.is_now
                                            class:disabled=month.disabled
                                        >
                                            {month.name}
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </Show>

                    <Show when=move || show.get() == Selection::Day fallback=|| ()>
                        <div class="weekday-names">
                            // Not use For for this...?
                            <For
                                each=move || short_weekday_names.get()
                                key=|short_weekday_name| short_weekday_name.clone()
                                children=move |short_weekday_name| {
                                    view! { <div class="weekday-name">{short_weekday_name}</div> }
                                }
                            />
                        </div>

                        <div class="weeks">
                            <For
                                each=move || calendar.weeks.get()
                                key=|week| week.id
                                children=move |week| {
                                    view! {
                                        <div class="week">
                                            <For
                                                each=move || week.days.clone()
                                                key=|day| day.id
                                                children=move |day| {
                                                    view! {
                                                        <div
                                                            on:click=move |_| {
                                                                if !day.disabled {
                                                                    calendar.select_date.run(day.date_time);
                                                                }
                                                            }
                                                            class="day"
                                                            class:is-staging=day.is_focused || day.is_selected
                                                            class:is-now=day.is_now
                                                            class:not-in-month=day.in_month != InMonth::Current
                                                            class:disabled=day.disabled
                                                        >
                                                            <span class="text">{day.index}</span>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </Show>

                </div>
            </div>
        </div>
    }
}

pub fn create_week_day_names() -> Vec<String> {
    //let day_in_month = value.date().day(); // 0 based
    //let days_from_monday = value.date().weekday().num_days_from_monday(); // 0 based
    //let monday = value.date().with_day(day_in_month - days_from_monday).format("%a").to_string();

    vec![
        "Mon".to_owned(),
        "Tue".to_owned(),
        "Wed".to_owned(),
        "Thr".to_owned(),
        "Fri".to_owned(),
        "Sat".to_owned(),
        "Sun".to_owned(),
    ]
}
