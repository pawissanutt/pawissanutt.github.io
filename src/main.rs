use dioxus::prelude::*;

mod components;
mod data;
use components::{home::Home, navbar::Navbar};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

static FAVICON: Asset = asset!("assets/favicon.ico");
static MAIN_CSS: Asset = asset!("assets/main.css", AssetOptions::css().with_preload(true));
static TAILWIND_CSS: Asset = asset!("assets/tailwind.css", AssetOptions::css().with_preload(true));

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
