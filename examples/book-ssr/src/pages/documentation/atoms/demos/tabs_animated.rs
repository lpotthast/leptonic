use leptonic::{
    atoms::tabs::{Tab, TabList, TabPanel, TabPanels, Tabs},
    hooks::use_collection,
};
use leptos::prelude::*;

#[component]
pub fn TabsAnimatedAtomDemo() -> impl IntoView {
    let collection = use_collection(|b| {
        b.item("summary", "Summary");
        b.item("ingredients", "Ingredients");
        b.item("steps", "Steps");
    });

    view! {
        <Tabs collection=collection classes="demo-tabs">
            <TabList aria_label="Recipe" classes="demo-tab-list">
                <Tab key="summary" classes="demo-tab">"Summary"</Tab>
                <Tab key="ingredients" classes="demo-tab">"Ingredients"</Tab>
                <Tab key="steps" classes="demo-tab">"Steps"</Tab>
            </TabList>
            // While the selected tab changes, `TabPanels` sets `--tab-panel-height` from the old panel's height to the
            // new one's; its stylesheet transitions `height` to it.
            <TabPanels classes="demo-tab-panels">
                <TabPanel key="summary" classes="demo-tab-panel">
                    <p>"Tomato soup, ready in 30 minutes."</p>
                </TabPanel>
                <TabPanel key="ingredients" classes="demo-tab-panel">
                    <ul>
                        <li>"1 kg ripe tomatoes"</li>
                        <li>"1 onion"</li>
                        <li>"2 cloves of garlic"</li>
                        <li>"500 ml vegetable stock"</li>
                        <li>"Olive oil, salt, pepper"</li>
                    </ul>
                </TabPanel>
                <TabPanel key="steps" classes="demo-tab-panel">
                    <ol>
                        <li>"Chop the onion and garlic, and soften them in olive oil."</li>
                        <li>"Add the chopped tomatoes and cook them for 10 minutes."</li>
                        <li>"Pour in the stock and simmer for 15 minutes."</li>
                        <li>"Blend until smooth, then season with salt and pepper."</li>
                    </ol>
                </TabPanel>
            </TabPanels>
        </Tabs>
    }
}
