use dioxus::prelude::*;
use crate::data::{get_education, EducationItem};

#[component]
pub fn Education() -> Element {
    let education = get_education();

    rsx! {
        section {
            id: "education",
            class: "py-20 px-4 bg-white dark:bg-gray-900",
            div {
                class: "container mx-auto",
                h2 {
                    class: "text-3xl font-bold text-center mb-12 text-gray-900 dark:text-white",
                    "Education"
                }
                div {
                    class: "flex flex-col gap-8",
                    for edu in education() {
                        EducationCard { education: edu }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EducationCard(education: EducationItem) -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
            h3 {
                class: "text-xl font-bold text-gray-900 dark:text-white",
                "{education.degree}"
            }
            p {
                class: "text-gray-700 dark:text-gray-300 font-semibold mt-2",
                "{education.university}"
            }
            if !education.dates.is_empty() {
                p {
                    class: "text-gray-600 dark:text-gray-400 mt-1",
                    "{education.dates}"
                }
            }
        }
    }
}
