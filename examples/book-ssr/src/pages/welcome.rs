use leptonic::{atoms::prelude::Link as AtomLink, jiff::civil::Date};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::{
    app::{BookToast, BookToasts, MainLandmark, SITE_DESCRIPTION},
    kit::{Code, Icon, Language, Link},
    routes,
};

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
        <Title text="Leptonic \u{2013} accessible UI building blocks for Leptos"/>
        <Meta name="description" content=SITE_DESCRIPTION/>

        <MainLandmark class="book-welcome">
            <div class="book-welcome-intro">
                <h1 class="book-welcome-title">"Leptonic"</h1>
                <p class="book-welcome-tagline">
                    "Accessible UI building blocks for Leptos: hooks, atoms and themed components."
                </p>
                <div class="book-welcome-actions">
                    <AtomLink href=routes::doc::Installation.materialize() classes="book-button">
                        "Get started"
                    </AtomLink>
                    <AtomLink
                        href=routes::doc::Overview.materialize()
                        classes="book-button"
                        attr:data-variant="secondary"
                    >
                        "Read the overview"
                    </AtomLink>
                </div>
                <Code language=Language::Shell classes="book-welcome-install">"cargo add leptonic --features full"</Code>
            </div>

            <Showcase/>

            <ul class="book-welcome-features">
                <Feature icon=icondata::BsUniversalAccess title="Accessible" href=routes::doc::Accessibility.materialize()>
                    "Keyboard, pointer, touch and screen reader interaction, ported from react-aria\u{2019}s "
                    "battle-tested hooks."
                </Feature>
                <Feature icon=icondata::BsLayers title="Three layers" href=routes::doc::Architecture.materialize()>
                    "Hooks for full control, unstyled atoms for your own design system, or themed components that "
                    "work out of the box."
                </Feature>
                <Feature icon=icondata::BsBraces title="Rust all the way" href=routes::doc::Ssr.materialize()>
                    "Typed APIs, server-side rendering with hydration, and internationalization without a JavaScript "
                    "runtime."
                </Feature>
            </ul>
        </MainLandmark>
    }
}

/// One of the welcome page's selling points: an icon, a linked title and a sentence.
#[component]
fn Feature(
    icon: icondata::Icon,
    title: &'static str,
    href: String,
    children: Children,
) -> impl IntoView {
    view! {
        <li class="book-welcome-feature">
            <Icon icon=icon classes="book-welcome-feature-icon"/>
            <h2><Link href>{title}</Link></h2>
            <p>{children()}</p>
        </li>
    }
}

/// A few of leptonic's atoms in a small form, live, styled by the book (`_pages.scss`): the save button shows a toast.
#[component]
fn Showcase() -> impl IntoView {
    use leptonic::atoms::{
        button::Button,
        calendar::{
            Calendar, CalendarCell, CalendarCellButton, CalendarGrid, CalendarGridBody,
            CalendarGridHeader, CalendarHeaderCell, CalendarHeaderRow, CalendarHeading,
            CalendarNextButton, CalendarPreviousButton, CalendarWeek,
        },
        datepicker::{DateInput, DatePicker, DatePickerButton, DatePickerGroup, DateSegment},
        dialog::Dialog,
        field::Label,
        input::Input,
        popover::Popover,
        slider::{Slider, SliderFill, SliderOutput, SliderThumb, SliderTrack},
        switch::Switch,
        text_field::TextField,
    };

    let toasts = expect_context::<BookToasts>();
    let project = RwSignal::new("Rocket".to_owned());
    let launch = RwSignal::new(None::<Date>);
    let notify = RwSignal::new(true);
    let volume = RwSignal::new(vec![60_u8]);

    let save = move |_| {
        let name = project.get_untracked();
        let body = match launch.get_untracked() {
            Some(date) => format!(
                "\u{201c}{name}\u{201d} launches on {}.",
                date.strftime("%B %-d, %Y")
            ),
            None => format!("\u{201c}{name}\u{201d} has no launch date yet."),
        };
        toasts.show(BookToast {
            title: "Saved".to_owned(),
            description: body,
        });
    };
    let reset = move |_| {
        project.set("Rocket".to_owned());
        launch.set(None);
        notify.set(true);
        volume.set(vec![60]);
    };

    view! {
        <section class="book-welcome-showcase" aria-labelledby="book-welcome-showcase-title">
            <h2 id="book-welcome-showcase-title" class="book-welcome-showcase-title">
                "Try them: leptonic\u{2019}s atoms, styled by this book"
            </h2>
            <div class="book-welcome-showcase-form">
                <TextField value=project set_value=project classes="book-welcome-field">
                    <Label classes="book-welcome-label">"Project"</Label>
                    <Input classes="book-welcome-input"/>
                </TextField>
                <DatePicker<Date> value=launch set_value=launch classes="book-welcome-field">
                    <Label classes="book-welcome-label">"Launch date"</Label>
                    <DatePickerGroup classes="book-welcome-date">
                        <DateInput
                            classes="book-welcome-date-segments"
                            children=|segment| view! { <DateSegment segment classes="book-welcome-date-segment"/> }
                        />
                        <DatePickerButton classes="book-welcome-date-button">
                            <Icon icon=icondata::BsCalendar3/>
                        </DatePickerButton>
                    </DatePickerGroup>
                    <Popover classes="book-welcome-popover">
                        <Dialog>
                            <Calendar classes="book-welcome-calendar">
                                <header class="book-welcome-calendar-header">
                                    <CalendarPreviousButton classes="book-welcome-calendar-nav">
                                        <Icon icon=icondata::BsChevronLeft/>
                                    </CalendarPreviousButton>
                                    <CalendarHeading classes="book-welcome-calendar-title"/>
                                    <CalendarNextButton classes="book-welcome-calendar-nav">
                                        <Icon icon=icondata::BsChevronRight/>
                                    </CalendarNextButton>
                                </header>
                                <CalendarGrid classes="book-welcome-calendar-grid">
                                    <CalendarGridHeader>
                                        <CalendarHeaderRow children=|day| view! { <CalendarHeaderCell>{day}</CalendarHeaderCell> }/>
                                    </CalendarGridHeader>
                                    <CalendarGridBody children=|week| view! {
                                        <CalendarWeek week children=|date| view! {
                                            <CalendarCell date>
                                                <CalendarCellButton classes="book-welcome-calendar-day"/>
                                            </CalendarCell>
                                        }/>
                                    }/>
                                </CalendarGrid>
                            </Calendar>
                        </Dialog>
                    </Popover>
                </DatePicker<Date>>
                <Slider values=volume set_values=volume min_value=0 max_value=100 classes="book-welcome-slider">
                    <div class="book-welcome-slider-header">
                        <Label classes="book-welcome-label">"Volume"</Label>
                        <SliderOutput classes="book-welcome-slider-output"/>
                    </div>
                    <SliderTrack classes="book-welcome-slider-track">
                        <SliderFill classes="book-welcome-slider-fill"/>
                        <SliderThumb classes="book-welcome-slider-thumb"/>
                    </SliderTrack>
                </Slider>
                <Switch is_selected=notify set_selected=notify classes="book-welcome-switch">
                    <span class="book-welcome-switch-track" aria-hidden="true">
                        <span class="book-welcome-switch-thumb"></span>
                    </span>
                    "Email me before the launch"
                </Switch>
            </div>
            <div class="book-welcome-showcase-actions">
                <Button on_press=save classes="book-welcome-button">"Save"</Button>
                <Button on_press=reset classes="book-welcome-button" attr:data-variant="secondary">"Reset"</Button>
            </div>
        </section>
    }
}
