use leptos::*;

use crate::{
    hooks::calendar::use_calendar,
    utils::time::{GuideMode, InMonth},
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
    #[prop(into)] on_change: Callback<time::OffsetDateTime>,
    #[prop(optional)] min: Option<time::OffsetDateTime>,
    #[prop(optional)] max: Option<time::OffsetDateTime>,
    #[prop(into, optional, default = GuideMode::CalendarFirst.into())] guide_mode: MaybeSignal<
        GuideMode,
    >,
) -> impl IntoView {
    let calendar = use_calendar(value, min, max);

    create_effect(move |_| on_change.call(calendar.selected.get()));

    let (show, set_show) = create_signal(match guide_mode.get() {
        GuideMode::CalendarFirst => Selection::Day,
        GuideMode::YearFirst => Selection::Year,
    });

    // TODO: Support internationalization
    let (short_weekday_names, _) = create_signal(create_week_day_names());

    view! {
        <leptonic-datetime>
        <leptonic-date-selector>
            <leptonic-calender-month>
                <div class={"actions"}>
                    {move || match show.get() {
                        Selection::Year => view! {
                            <div on:click=move |_| calendar.select_previous_years()
                                class="previous arrow-left">
                            </div>
                            <div on:click=move |_| set_show.update(|show| *show = Selection::Month)
                                class="current-date">
                                {calendar.years_range}
                            </div>
                            <div on:click=move |_| calendar.select_next_years()
                                class="next arrow-right">
                            </div>
                        },
                        Selection::Month => view! {
                            <div on:click=move |_| calendar.select_previous_year()
                                class="previous arrow-left">
                            </div>
                            <div on:click=move |_| set_show.update(|show| *show = Selection::Year)
                                class="current-date">
                                { move || calendar.staging.get().year() }
                            </div>
                            <div on:click=move |_| calendar.select_next_year()
                                class="next arrow-right">
                            </div>
                        },
                        Selection::Day => view! {
                            <div on:click=move |_| calendar.select_previous_month()
                                class="previous arrow-left">
                            </div>
                            <div on:click=move |_| set_show.update(|show| *show = Selection::Year)
                                class="current-date">
                                { move || calendar.staging_month_name.get() } " " { move || calendar.staging_year.get() }
                            </div>
                            <div on:click=move |_| calendar.select_next_month()
                                class="next arrow-right">
                            </div>
                        },
                    }}
                </div>

                <Show when=move || show.get() == Selection::Year fallback=|| ()>
                    <div class="years">
                        <For
                            each=move || calendar.years.get()
                            key=|year| year.number
                            children=move |year| {
                                view! {
                                    <div on:click=move |_| {
                                        if !year.disabled {
                                            calendar.select_year(year);
                                            set_show.update(|show| *show = Selection::Month);
                                        }
                                    }
                                        class="year"
                                        class:is-staging=year.is_staging
                                        class:is-now=year.is_now
                                        class:disabled=year.disabled
                                    >
                                        { year.number }
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
                                let month_clone = month.clone();
                                view! {
                                    <div on:click=move |_| {
                                        if !month.disabled {
                                            calendar.select_month(month_clone.clone());
                                            set_show.update(|show| *show = Selection::Day);
                                        }
                                    }
                                        class="month"
                                        class:is-staging=month.is_staging
                                        class:is-now=month.is_now
                                        class:disabled=month.disabled>
                                        {month.name}
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>

                <Show when=move || show.get() == Selection::Day fallback=|| ()>
                    <div class={"weekday-names"}>
                        // Not use For for this...?
                        <For
                            each=move || short_weekday_names.get()
                            key=|short_weekday_name| short_weekday_name.clone()
                            children=move |short_weekday_name| {
                                view! {
                                    <div class={"weekday-name"}>
                                        {short_weekday_name}
                                    </div>
                                }
                            }
                        />
                    </div>

                    <div class={"weeks"}>
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
                                                                calendar.select_day(day);
                                                            }
                                                        }
                                                        class="day"
                                                        class:is-staging=day.is_staging
                                                        class:is-now=day.is_now
                                                        class:not-in-month=day.in_month != InMonth::Current
                                                        class:disabled=day.disabled
                                                    >
                                                        <span class="text">
                                                            {day.index}
                                                        </span>
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

            </leptonic-calender-month>
        </leptonic-date-selector>
        </leptonic-datetime>
    }
}

pub fn create_week_day_names() -> Vec<String> {
    //let day_in_month = value.date().day(); // 1 based
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
