use leptonic::{
    I18nProvider, Locale,
    atoms::{
        button::Button,
        dialog::Dialog,
        field::Label,
        input::Input,
        menu::{Menu, MenuItem, MenuItems, MenuSection, MenuTrigger, SubmenuTrigger},
        popover::Popover,
        text_field::TextField,
    },
    hooks::{
        collections::{Key, use_collection},
        menu::{MenuTriggerType, SubmenuKind},
    },
};
use leptos::prelude::*;

/// A menu tree (react-aria-components' `Menu.test.tsx` submenu setups): "File"
/// (`#test-submenu-trigger`) opens Open, Rename, Share… (submenu: Email… (submenu: Work,
/// Personal), SMS, X) and Delete; "Right click here" (`#test-context-trigger`) opens a context menu
/// (Cut, Paste). Every action is appended to `#test-submenu-actions`.
#[component]
pub fn PageAtomSubmenu() -> impl IntoView {
    let actions = RwSignal::new(Vec::<String>::new());
    let name = RwSignal::new("report.pdf".to_owned());
    let on_action = move |key: Key| actions.update(|a| a.push(key.to_string()));
    let root_items = use_collection(|b| {
        for (key, text) in [
            ("open", "Open"),
            ("rename", "Rename…"),
            ("share", "Share…"),
            ("signup", "Sign up…"),
            ("properties", "Properties…"),
            ("delete", "Delete…"),
        ] {
            b.item(key, text);
        }
    });
    let share_items = use_collection(|b| {
        for (key, text) in [("email", "Email…"), ("sms", "SMS"), ("x", "X")] {
            b.item(key, text);
        }
    });
    let email_items = use_collection(|b| {
        for (key, text) in [("work", "Work"), ("personal", "Personal")] {
            b.item(key, text);
        }
    });

    let rtl: Locale = "ar-EG".parse().expect("a valid locale");
    let rtl_items = use_collection(|b| {
        for (key, text) in [("rtl-open", "Open (RTL)"), ("rtl-share", "Share (RTL)")] {
            b.item(key, text);
        }
    });
    let rtl_share_items = use_collection(|b| {
        for (key, text) in [("rtl-email", "Email (RTL)"), ("rtl-sms", "SMS (RTL)")] {
            b.item(key, text);
        }
    });

    let context_items = use_collection(|b| {
        for (key, text) in [("cut", "Cut"), ("paste", "Paste")] {
            b.item(key, text);
        }
    });

    view! {
        <div id="test-page-atom-submenu">
            <h1>"Submenus"</h1>
            <button id="test-submenu-before">"Before"</button>
            <MenuTrigger>
                <Button id="test-submenu-trigger">"File"</Button>
                <Popover classes="test-popover">
                    <Menu collection=root_items on_action=on_action classes="test-menu">
                        <MenuItem key="open">"Open"</MenuItem>
                        <MenuItem key="rename">"Rename…"</MenuItem>
                        <SubmenuTrigger key="share">
                            <MenuItem key="share">"Share…"</MenuItem>
                            <Popover classes="test-popover">
                                <Menu
                                    collection=share_items
                                    on_action=on_action
                                    classes="test-menu"
                                >
                                    <SubmenuTrigger key="email">
                                        <MenuItem key="email">"Email…"</MenuItem>
                                        <Popover classes="test-popover">
                                            <Menu
                                                collection=email_items
                                                on_action=on_action
                                                classes="test-menu"
                                            >
                                                <MenuItems let:node>
                                                    {node.text_value.to_string()}
                                                </MenuItems>
                                            </Menu>
                                        </Popover>
                                    </SubmenuTrigger>
                                    <MenuItem key="sms">"SMS"</MenuItem>
                                    <MenuItem key="x">"X"</MenuItem>
                                </Menu>
                            </Popover>
                        </SubmenuTrigger>
                        // A subdialog (RAC's "should contain focus for subdialogs").
                        <SubmenuTrigger key="signup" kind=SubmenuKind::Dialog>
                            <MenuItem key="signup">"Sign up…"</MenuItem>
                            <Popover classes="test-popover">
                                <form>
                                    <label>"First name" <input id="test-signup-first" /></label>
                                    <label>"Last name" <input id="test-signup-last" /></label>
                                </form>
                            </Popover>
                        </SubmenuTrigger>
                        // A subdialog with a `Dialog` inside its popover (RAC's docs example).
                        <SubmenuTrigger key="properties" kind=SubmenuKind::Dialog>
                            <MenuItem key="properties">"Properties…"</MenuItem>
                            <Popover classes="test-popover">
                                <Dialog aria_label="Properties">
                                    <TextField value=name set_value=name>
                                        <Label>"Name"</Label>
                                        <Input attr:id="test-properties-input" />
                                    </TextField>
                                </Dialog>
                            </Popover>
                        </SubmenuTrigger>
                        <MenuItem key="delete">"Delete…"</MenuItem>
                    </Menu>
                </Popover>
            </MenuTrigger>
            <div>
                "Actions: "
                <span id="test-submenu-actions">{move || actions.get().join(", ")}</span>
            </div>

            // A context menu (RAC's "should support a context menu trigger").
            <MenuTrigger trigger=MenuTriggerType::ContextMenu>
                <Button id="test-context-trigger" attr:style="padding: 40px">
                    "Right click here"
                </Button>
                <Popover classes="test-popover">
                    <Menu collection=context_items on_action=on_action classes="test-menu">
                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>

            // A menu tree in a right-to-left subtree (ArrowLeft opens submenus).
            <I18nProvider locale=rtl>
                <MenuTrigger>
                    <Button id="test-submenu-rtl-trigger">"RTL"</Button>
                    <Popover classes="test-popover">
                        <Menu collection=rtl_items on_action=on_action classes="test-menu">
                            <MenuItem key="rtl-open">"Open (RTL)"</MenuItem>
                            <SubmenuTrigger key="rtl-share">
                                <MenuItem key="rtl-share">"Share (RTL)"</MenuItem>
                                <Popover classes="test-popover">
                                    <Menu
                                        collection=rtl_share_items
                                        on_action=on_action
                                        classes="test-menu"
                                    >
                                        <MenuItems let:node>
                                            {node.text_value.to_string()}
                                        </MenuItems>
                                    </Menu>
                                </Popover>
                            </SubmenuTrigger>
                        </Menu>
                    </Popover>
                </MenuTrigger>
            </I18nProvider>
            <ToolsMenu on_action=on_action />
        </div>
    }
}

/// "Tools" (`#test-submenu-tools-trigger`): Arrange… (a submenu with the sections "Align": Left,
/// Right, and "Order": Front, Back), Contact… (a submenu whose "Nested subdialog" opens a form,
/// next to B and C, and a "Test" button below the menu) and Quit.
#[component]
fn ToolsMenu(on_action: impl Fn(Key) + Copy + Send + Sync + 'static) -> impl IntoView {
    let tools_items = use_collection(|b| {
        for (key, text) in [
            ("arrange", "Arrange…"),
            ("contact", "Contact…"),
            ("quit", "Quit"),
        ] {
            b.item(key, text);
        }
    });
    let arrange_items = use_collection(|b| {
        b.section("align", |s| {
            s.header("align-header", "Align");
            s.item("left", "Left");
            s.item("right", "Right");
        });
        b.section("order", |s| {
            s.header("order-header", "Order");
            s.item("front", "Front");
            s.item("back", "Back");
        });
    });
    let contact_items = use_collection(|b| {
        for (key, text) in [("nested", "Nested subdialog"), ("b", "B"), ("c", "C")] {
            b.item(key, text);
        }
    });
    view! {
        <MenuTrigger>
            <Button id="test-submenu-tools-trigger">"Tools"</Button>
            <Popover classes="test-popover">
                <Menu collection=tools_items on_action=on_action classes="test-menu">
                    <SubmenuTrigger key="arrange">
                        <MenuItem key="arrange">"Arrange…"</MenuItem>
                        <Popover classes="test-popover">
                            <Menu collection=arrange_items on_action=on_action classes="test-menu">
                                <MenuSection key="align">
                                    <MenuItem key="left">"Left"</MenuItem>
                                    <MenuItem key="right">"Right"</MenuItem>
                                </MenuSection>
                                <MenuSection key="order">
                                    <MenuItem key="front">"Front"</MenuItem>
                                    <MenuItem key="back">"Back"</MenuItem>
                                </MenuSection>
                            </Menu>
                        </Popover>
                    </SubmenuTrigger>
                    <SubmenuTrigger key="contact">
                        <MenuItem key="contact">"Contact…"</MenuItem>
                        <Popover classes="test-popover">
                            <Menu collection=contact_items on_action=on_action classes="test-menu">
                                <SubmenuTrigger key="nested">
                                    <MenuItem key="nested">"Nested subdialog"</MenuItem>
                                    <Popover classes="test-popover">
                                        <form>
                                            <label>"Email" <input id="test-contact-email" /></label>
                                            <label>"Phone" <input id="test-contact-phone" /></label>
                                        </form>
                                    </Popover>
                                </SubmenuTrigger>
                                <MenuItem key="b">"B"</MenuItem>
                                <MenuItem key="c">"C"</MenuItem>
                            </Menu>
                            <button id="test-contact-button">"Test"</button>
                        </Popover>
                    </SubmenuTrigger>
                    <MenuItem key="quit">"Quit"</MenuItem>
                </Menu>
            </Popover>
        </MenuTrigger>
    }
}
