use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageNavigation() -> impl IntoView {
    view! {
        <DocPage title="Navigation">
            <p>
                "Navigation concepts move users between pages and between the parts of a page: links to other pages, "
                "sites or sections, breadcrumbs showing where the current page sits in a hierarchy, tabs switching between "
                "panels that share one space, and disclosures expanding content in place."
            </p>
            <p>
                "They belong together because each one changes what the user sees without triggering an action. Concepts "
                "that trigger actions, such as buttons and menus, are in "<Link href=routes::doc::Buttons.materialize()>"Buttons"</Link>
                " and "<Link href=routes::doc::Collections.materialize()>"Collections"</Link>"."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Navigation.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::Breadcrumbs.materialize()>"Breadcrumbs"</Link>" are a list of "
                        <Link href=routes::doc::Link.materialize()>"links"</Link>". The last one is the current page: it is "
                        "marked with "<Code inline=true>"aria-current"</Code>" for screen readers and can\u{2019}t be followed."
                    </li>
                    <li>
                        <Link href=routes::doc::Link.materialize()>"Link"</Link>" navigates to pages of your app and "
                        "other sites; "<Link href=format!("{}#anchorlink", routes::doc::link::Atom.materialize())>"AnchorLink"</Link>
                        ", part of the same concept, scrolls to a section of the current page, and "
                        <Link href=format!("{}#linkbutton", routes::doc::link::Atom.materialize())>"LinkButton"</Link>
                        " is a link that looks like a button."
                    </li>
                    <li>
                        <Link href=routes::doc::Tabs.materialize()>"Tabs"</Link>" and "
                        <Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link>" both reveal content on the "
                        "current page. Tabs show exactly one of several parallel panels; disclosures expand and collapse "
                        "on their own, or as an accordion in which opening one closes the others."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Go to another page of your app, or to another site"</TableCell>
                        <TableCell><Link href=routes::doc::Link.materialize()>"Link"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Scroll to a section of the current page"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Link.materialize()>"Link"</Link>" ("<Code inline=true>"AnchorLink"</Code>")"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show where the current page sits in a hierarchy, with a way back up"</TableCell>
                        <TableCell><Link href=routes::doc::Breadcrumbs.materialize()>"Breadcrumbs"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Switch between panels of parallel content that share one space"</TableCell>
                        <TableCell><Link href=routes::doc::Tabs.materialize()>"Tabs"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show or hide details in place"</TableCell>
                        <TableCell><Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Keep only one of several sections open at a time"</TableCell>
                        <TableCell><Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link>" group (accordion)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Trigger an action, such as submit or delete"</TableCell>
                        <TableCell><Link href=routes::doc::Button.materialize()>"Button"</Link></TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "If an element navigates, it is a link, however it looks; if it triggers an action, it is a button."
                </p>
            </Section>
        </DocPage>
    }
}
