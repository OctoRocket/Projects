use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, Meta, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" type="image/x-icon" href="/favicon.png"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // Injects a stylesheet into the document <head>
        // `id=leptos` means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/asta-leptos.css"/>

        // sets the document and page information
        <Title text="Asta!"/>
        <Meta name="author" content="Asta/OctoRocket"/>
        <Meta name="description" content="A satellite drifting through space"/>

        // content for this welcome page
        <Router>
            <Routes fallback=|| "Page not found.".into_view()> //TODO Make this a 404 page
                <Route path=path!("") view=Home/>
            </Routes>
        </Router>
    }
}

// Content below

/// The landing page
#[component]
fn Home() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        // Wrap everything in an outer bound to pad it
        <div id="outer-bound" class="clear column">
            <TitleHeading/>
            <h1>"Welcome to Leptos!"</h1>
            <div>
                <p>"Test:"<a href="https://example.com">"goob"</a></p>
                <button on:click=on_click>"Click Me: " {count}</button>
            </div>
        </div>
    }
}

#[component]
fn TitleHeading() -> impl IntoView {
    view! {
        <div class="column main-section">
            <marquee scrollamount="15" class="home-video">
                <p>
                    "♠"
                    "Do not judge someone based on anything but their actions."
                    "♣"
                    "Don't be a bigot or I'll personally ban your IP from accessing this server."
                    "♦"
                    "This is a website safe for all queer people; aspec and trans people are queer too."
                    "♥"
                </p>
            </marquee>
            <p id="title-intro-text">
                <span id="title-title" class="description">The Impure Function:</span>
                <span id="title-name">ASTA</span>
                <span id="title-title" class="description">, orbiting the abyss of some far off land...</span>
            </p>
        </div>
    }
}
