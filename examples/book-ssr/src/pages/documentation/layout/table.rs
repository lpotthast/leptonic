use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::*;

#[derive(Clone)]
pub struct Minion {
    id: u32,
    name: String,
    appearance: String,
    num_eyes: u32,
}

#[component]
pub fn PageTable() -> impl IntoView {
    let minions = create_rw_signal(vec![
        Minion {
            id: 1,
            name: String::from("Kevin"),
            appearance: String::from("Tall"),
            num_eyes: 2,
        },
        Minion {
            id: 2,
            name: String::from("Bob"),
            appearance: String::from("Short"),
            num_eyes: 2,
        },
        Minion {
            id: 3,
            name: String::from("Stuart"),
            appearance: String::from("Medium"),
            num_eyes: 1,
        },
        Minion {
            id: 4,
            name: String::from("Otto"),
            appearance: String::from("Round"),
            num_eyes: 2,
        }
    ]);

    view! {
        <H1>"Table"</H1>

        <P>"Tables..."</P>

        <Code>
            {indoc!(r##"
                <TableContainer>
                    <Table bordered=true hoverable=true>
                        <TableHeader>
                            <TableRow>
                                <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                                <TableHeaderCell>"Name"</TableHeaderCell>
                                <TableHeaderCell>"Appearance"</TableHeaderCell>
                                <TableHeaderCell>"Num. eyes"</TableHeaderCell>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <For
                                each=move || minions.get()
                                key=move |minion| minion.id
                                children=move |minion| view! {
                                    <TableRow>
                                        <TableCell>{minion.id}</TableCell>
                                        <TableCell>{minion.name}</TableCell>
                                        <TableCell>{minion.appearance}</TableCell>
                                        <TableCell>{minion.num_eyes}</TableCell>
                                    </TableRow>
                                }
                            />
                        </TableBody>
                    </Table>
                </TableContainer>
            "##)}
        </Code>

        <TableContainer>
            <Table bordered=true hoverable=true>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                        <TableHeaderCell>"Name"</TableHeaderCell>
                        <TableHeaderCell>"Appearance"</TableHeaderCell>
                        <TableHeaderCell>"Num. eyes"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <For
                        each=move || minions.get()
                        key=move |minion| minion.id
                        children=move |minion| view! {
                            <TableRow>
                                <TableCell>{minion.id}</TableCell>
                                <TableCell>{minion.name}</TableCell>
                                <TableCell>{minion.appearance}</TableCell>
                                <TableCell>{minion.num_eyes}</TableCell>
                            </TableRow>
                        }
                    />
                </TableBody>
            </Table>
        </TableContainer>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                // Table wrapper
                --table-wrapper-box-shadow-color

                // Table
                --table-color
                --table-background-color
                --table-background-color-on-hover
                --table-background-color-of-striped-rows
                --table-header-background-color
                --table-border-color
                --table-cell-box-shadow-on-hover
                --table-column-background-if-ordered
                --table-header-cell-padding
                --table-body-cell-padding
            ")}
        </Code>
    }
}
