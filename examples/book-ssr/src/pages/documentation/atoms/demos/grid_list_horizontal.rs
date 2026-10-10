use leptonic::{
    Orientation,
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        grid_list::{GridList, GridListItem},
    },
    hooks::collections::{
        Key, ListLayout, Selection, SelectionMode, UseListCollectionInput, use_list_collection,
    },
};
use leptos::prelude::*;

/// A photo: key, name and icon.
type Photo = (&'static str, &'static str, &'static str);

const PHOTOS: [Photo; 8] = [
    ("beach", "Beach", "\u{1f3d6}"),
    ("forest", "Forest", "\u{1f332}"),
    ("mountain", "Mountain", "\u{1f3d4}"),
    ("city", "City", "\u{1f3d9}"),
    ("desert", "Desert", "\u{1f3dc}"),
    ("lake", "Lake", "\u{1f30a}"),
    ("snow", "Snow", "\u{2744}"),
    ("sunset", "Sunset", "\u{1f305}"),
];

#[component]
pub fn GridListHorizontalDemo() -> impl IntoView {
    let photos = use_list_collection(UseListCollectionInput {
        items: Signal::stored(PHOTOS.to_vec()),
        key: |(key, _, _)| Key::from(*key),
        text_value: |(_, name, _)| (*name).to_owned(),
    });
    let selection = RwSignal::new(Selection::default());
    let horizontal = RwSignal::new(true);
    // The arrow keys follow the orientation; the stylesheet lays the cards out by `data-orientation`.
    let orientation = Signal::derive(move || {
        if horizontal.get() {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        }
    });

    let status = move || {
        selection.with(|selection| match selection {
            Selection::Keys(keys) if !keys.is_empty() => {
                let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                keys.sort();
                format!("Selected: {}.", keys.join(", "))
            }
            _ => "Selected: none.".to_owned(),
        })
    };

    view! {
        <GridList
            collection=photos
            layout=ListLayout::Grid
            orientation=orientation
            selection_mode=SelectionMode::Single
            selection=selection
            set_selection=selection
            aria_label="Photos"
            classes="demo-grid-list-cards"
        >
            {PHOTOS
                .map(|(key, name, icon)| view! {
                    <GridListItem key=key classes="demo-grid-list-card">
                        <span class="demo-grid-list-card-icon" aria-hidden="true">{icon}</span>
                        <span>{name}</span>
                    </GridListItem>
                })
                .collect_view()}
        </GridList>
        <p class="demo-status">{status}</p>

        <div class="demo-controls">
            <CheckboxField is_selected=horizontal set_selected=horizontal>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Horizontal"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
