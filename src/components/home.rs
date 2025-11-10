use dioxus::prelude::*;
use crate::components::{hero::Hero, experience::Experience, education::Education, featured_research::FeaturedResearch, publications::Publications, skills::Skills, footer::Footer};

#[component]
pub fn Home() -> Element {
    rsx! {
    Hero {}
    Experience {}
    Education {}
    FeaturedResearch {}
    Publications {}
        Skills {}
        Footer {}
    }
}
