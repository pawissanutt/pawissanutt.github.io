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
                    class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                    for (category, skill_list) in skills() {
                        div {
                            class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md transition-all duration-300 hover:shadow-lg",
                            h3 {
                                class: "text-xl font-bold mb-4 text-gray-900 dark:text-white",
                                "{category}"
                            }
                            div {
                                class: "flex flex-wrap gap-2",
                                for skill in skill_list {
                                    span {
                                        class: "bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded dark:bg-blue-200 dark:text-blue-800 cursor-default",
                                        "{skill}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
