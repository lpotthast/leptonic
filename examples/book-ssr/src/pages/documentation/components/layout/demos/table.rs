use leptonic::components::prelude::*;
use leptos::prelude::*;

#[derive(Clone)]
struct Minion {
    id: u32,
    name: String,
    appearance: String,
    num_eyes: u32,
}

#[component]
pub fn TableDemo() -> impl IntoView {
    let minions = RwSignal::new(vec![
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
        },
    ]);

    view! {
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
    }
}
