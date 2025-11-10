use dioxus::prelude::*;
use crate::data::{get_experiences, ExperienceItem};

#[component]
pub fn Experience() -> Element {
    let experiences = get_experiences();

    rsx! {
        section {
            id: "experience",
            class: "py-20 px-4 bg-white dark:bg-gray-900",
            div {
                class: "container mx-auto",
                h2 {
                    class: "text-3xl font-bold text-center mb-12 text-gray-900 dark:text-white",
                    "Experience"
                }
                div {
                    class: "flex flex-col gap-8",
                    for exp in experiences() {
                        ExperienceCard { experience: exp }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ExperienceCard(experience: ExperienceItem) -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
            h3 {
                class: "text-xl font-bold text-gray-900 dark:text-white",
                "{experience.title}"
            }
            p {
                class: "text-gray-700 dark:text-gray-300 font-semibold",
                "{experience.company} ({experience.dates})"
            }
            p {
                class: "text-gray-600 dark:text-gray-400 mt-2",
                "{experience.description}"
            }
            div {
                class: "flex flex-wrap gap-2 mt-4",
                for tag in &experience.tags {
                    span {
                        class: "bg-blue-100 text-blue-800 text-xs font-semibold mr-2 px-2.5 py-0.5 rounded dark:bg-blue-200 dark:text-blue-800",
                        "{tag}"
                    }
                }
            }
        }
    }
}
