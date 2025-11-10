use dioxus::prelude::*;
use crate::data::get_skills;

#[component]
pub fn Skills() -> Element {
    let skills = get_skills();

    rsx! {
        section {
            id: "skills",
            class: "py-20 px-4 bg-white dark:bg-gray-900",
            div {
                class: "container mx-auto",
                h2 {
                    class: "text-3xl font-bold text-center mb-12 text-gray-900 dark:text-white",
                    "Skills"
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for (category, skill_list) in skills() {
                        div {
                            class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
                            h3 {
                                class: "text-xl font-bold mb-4 text-gray-900 dark:text-white",
                                "{category}"
                            }
                            ul {
                                class: "list-disc list-inside text-gray-700 dark:text-gray-300",
                                for skill in skill_list {
                                    li { "{skill}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
