use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;
use tracing::info;
use uuid::Uuid;

use leptonic::prelude::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Leptos component demo"/>
        <ThemeProvider>
            <ToastRoot>
                <ModalRoot>
                    <Box>
                        <Router>
                            <Routes>
                                <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                            </Routes>
                        </Router>
                    </Box>
                </ModalRoot>
            </ToastRoot>
        </ThemeProvider>
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal_ls(cx, "count", 0u64);

    let increase_counter_by_one = move |_| set_count.update(|count| *count += 1);

    let (show_modal, set_show_modal) = create_signal(cx, false);
    let (show_modal2, set_show_modal2) = create_signal(cx, false);

    let (test_bool, set_test_bool) = create_signal(cx, false);
    let (center_alert, set_center_alert) = create_signal(cx, false);

    let (transition_collapse, set_transition_collapse) = create_signal(cx, false);
    let (transition_fade, set_transition_fade) = create_signal(cx, false);
    let (transition_grow, set_transition_grow) = create_signal(cx, false);
    let (transition_slide, set_transition_slide) = create_signal(cx, false);
    let (transition_zoom, set_transition_zoom) = create_signal(cx, false);

    let toasts = use_context::<Toasts>(cx).unwrap();

    view! { cx,
        <h1>"Leptos component demo!"</h1>
        <DarkThemeToggle />

        <Button on_click=increase_counter_by_one>"Click Me: " {count}</Button>

        <Button on_click=move |_| set_show_modal.set(true)>"Show Modal"</Button>

        <Modal display_if=show_modal>
            <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
            <ModalBody>"This ia a test modal."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal.set(false) color=ButtonColor::Danger>"Accept"</Button>
                    <Button on_click=move |_| {
                        //set_show_modal.set(false);
                        set_show_modal2.set(true);
                    } color=ButtonColor::Info>"Next"</Button>
                    <Button on_click=move |_| set_show_modal.set(false)>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <Modal display_if=show_modal2>
            <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
            <ModalBody>"This overlays..."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal2.set(false)>"Back"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <Separator />

        <h2>"Buttons"</h2>
        <div>
            <Button on_click=move |_| {}>"Flat"</Button>
            <Button on_click=move |_| {}>"Outlined"</Button>
            <Button on_click=move |_| {}>"Filled"</Button>
        </div>

        <Separator />

        <h2>"Tabs"</h2>
        <Tabs>
            <Tab
                name="outer-1"
                label=view! {cx, "Toasts; Count is" {move || count.get()}}
                on_show=move || {info!("tab1 is now shown!")}
                on_hide=move || {info!("tab1 is now hidden!")}
            >
                <Checkbox checked=(test_bool, set_test_bool) />
                <Checkbox checked=(test_bool, set_test_bool) />
                <Toggle on=test_bool set_on=set_test_bool />
                <Button on_click=move |_| toasts.push(op_success_toast(cx))>"Create Toast"</Button>
                <If sig=test_bool>
                    "asd"
                </If>
            </Tab>
            <Tab name="outer-2" label="Tab2Label">
                <Tabs>
                    <Tab name="inner-1" label="Inner1">
                        "That is nested!"
                    </Tab>
                    <Tab name="inner-2" label="Inner2">
                        "That is nested as well!"
                    </Tab>
                </Tabs>
            </Tab>
            <Tab name="outer-3" label="Tab2Label">
                <ProgressBar progress=create_signal(cx, 34).0/>
            </Tab>
            <Tab name="outer-4" label="Tab4Label">
            </Tab>
        </Tabs>

        <Separator />

        <h2>"Collapsibles"</h2>
        <Collapsibles default_on_open=OnOpen::CloseOthers>
            <Collapsible
                header="Header1"
                body=view! {cx, "Body1"} />
            <Collapsible
                header="Header2"
                body=view! {cx, "Body2"} />
            <Collapsible
                header="Header3 - on_open::DoNothing"
                body=view! {cx, "Body3"}
                on_open=OnOpen::DoNothing />
        </Collapsibles>


        <Separator />

        <h2>"Transition - Collapse"</h2>
        <Toggle on=transition_collapse set_on=set_transition_collapse />
        <Collapse show=transition_collapse axis=CollapseAxis::X>
            <Skeleton height="5em">"Collapse"</Skeleton>
        </Collapse>

        <Toggle on=transition_collapse set_on=set_transition_collapse />
        <Collapse show=transition_collapse axis=CollapseAxis::Y>
            <Skeleton height="5em">"Collapse"</Skeleton>
        </Collapse>

        <Separator />

        <h2>"Transition - Fade"</h2>
        <Toggle on=transition_fade set_on=set_transition_fade />
        <Fade inn=Signal::derive(cx, move || transition_fade.get())>
            <Skeleton>"Fade"</Skeleton>
        </Fade>

        <Separator />

        <h2>"Transition - Grow"</h2>
        <Toggle on=transition_grow set_on=set_transition_grow />
        <Grow inn=Signal::derive(cx, move || transition_grow.get())>
            <Skeleton>"Grow"</Skeleton>
        </Grow>

        <Separator />

        <h2>"Transition - Slide"</h2>
        <Toggle on=transition_slide set_on=set_transition_slide />
        <Slide inn=Signal::derive(cx, move || transition_slide.get())>
            <Skeleton>"Slide"</Skeleton>
        </Slide>

        <Separator />

        <h2>"Transition - Zoom"</h2>
        <Toggle on=transition_zoom set_on=set_transition_zoom />
        <Zoom inn=Signal::derive(cx, move || transition_zoom.get())>
            <Skeleton>"Zoom"</Skeleton>
        </Zoom>

        <Separator />

        <h2>"Stack"</h2>
        <Stack spacing=6>
            <Skeleton>"Item 1"</Skeleton>
            <Skeleton>"Item 2"</Skeleton>
            <Skeleton>"Item 3"</Skeleton>
        </Stack>

        <Separator />

        <h2>"Grid"</h2>
        <Grid spacing=6>
            <Row>
                <Col md=3 sm=4 xs=6>
                    <Skeleton>"Item 1"</Skeleton>
                </Col>
                <Col md=3 sm=4 xs=6>
                    <Skeleton>"Item 2"</Skeleton>
                </Col>
                <Col md=3 sm=4 xs=6>
                    <Skeleton>"Item 3"</Skeleton>
                </Col>
                <Col md=3 sm=12 xs=6>
                    <Skeleton>"Item 4"</Skeleton>
                </Col>
            </Row>
            <Row>
                <Col md=8 sm=6 xs=12>
                    <Skeleton>"Item 5"</Skeleton>
                </Col>
                <Col md=4 sm=6 xs=12>
                    <Skeleton>"Item 6"</Skeleton>
                </Col>
            </Row>
        </Grid>

        <Separator />

        <h2>"Alerts"</h2>
        <Button on_click=move |_| set_center_alert.update(|it| *it = !*it)>"Center toggle"</Button>
        <Alert variant=AlertVariant::Success title="asd" centered=center_alert.into()>"Success alert"</Alert>
        <Alert variant=AlertVariant::Info title="asd" centered=true.into()>"Info alert"</Alert>
        <Alert variant=AlertVariant::Warn title="asd">"Warn alert"</Alert>
        <Alert variant=AlertVariant::Danger title="asd">"Danger alert"</Alert>

        <Separator />

        <Icon icon=BsIcon::BsFolderFill/>
        <Icon icon=BsIcon::BsFolder/>

        <h2>"Sliders"</h2>

        <Separator />

        <h2>"Separators"</h2>
        <Separator />
    }
}

fn op_success_toast(cx: Scope) -> Toast {
    Toast {
        id: Uuid::new_v4(),
        created_at: time::OffsetDateTime::now_utc(),
        variant: ToastVariant::Success,
        header: view! { cx, "Header" }.into_view(cx),
        body: view! { cx, "Body" }.into_view(cx),
        timeout: ToastTimeout::DefaultDelay,
    }
}
