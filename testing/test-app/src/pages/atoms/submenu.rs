use leptonic::{
    atoms::{
        field::Label,
        input::Input,
        prelude::{
            Button, Dialog, Menu, MenuItem, MenuItems, MenuTrigger, Popover, SubmenuTrigger,
        },
        text_field::TextField,
    },
    hooks::{
        MenuTriggerType, SubmenuKind,
        collections::{Key, use_collection},
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
                <Button attr:id="test-submenu-trigger">"File"</Button>
                <Popover classes="test-popover">
                    <Menu collection=root_items on_action=on_action classes="test-menu">
                        <MenuItem key="open">"Open"</MenuItem>
                        <MenuItem key="rename">"Rename…"</MenuItem>
                        <SubmenuTrigger key="share">
                            <MenuItem key="share">"Share…"</MenuItem>
                            <Popover classes="test-popover">
                                <Menu collection=share_items on_action=on_action classes="test-menu">
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
            <div>"Actions: " <span id="test-submenu-actions">{move || actions.get().join(", ")}</span></div>
            <button id="test-submenu-outside">"Outside"</button>

            // A context menu (RAC's "should support a context menu trigger").
            <MenuTrigger trigger=MenuTriggerType::ContextMenu>
                <Button attr:id="test-context-trigger" attr:style="padding: 40px">
                    "Right click here"
                </Button>
                <Popover classes="test-popover">
                    <Menu collection=context_items on_action=on_action classes="test-menu">
                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>

        </div>
    }
}
