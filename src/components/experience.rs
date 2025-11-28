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
    let mut is_expanded = use_signal(|| false);

    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md transition-all duration-300 hover:shadow-lg",
            div {
                class: "flex justify-between items-start mb-2",
                div {
                    h3 {
                        class: "text-xl font-bold text-gray-900 dark:text-white",
                        "{experience.title}"
                    }
                    p {
                        class: "text-gray-700 dark:text-gray-300 font-semibold",
                        "{experience.company} ({experience.dates})"
                    }
                }
            }
            p {
                class: "text-gray-600 dark:text-gray-400 mb-4",
                "{experience.description}"
            }
            
            if is_expanded() {
                ul {
                    class: "list-disc list-inside mb-4 space-y-2 text-gray-600 dark:text-gray-400 animate-fade-in",
                    for detail in &experience.details {
                        li {
                            if let Some(label) = &detail.label {
                                span { class: "font-bold text-gray-900 dark:text-gray-200", "{label}: " }
                            }
                            "{detail.content}"
                            if let Some(link) = &detail.internal_link {
                                div {
                                    class: "mt-2",
                                    a {
                                        href: "{link}",
                                        class: "inline-flex items-center gap-1 px-3 py-1 rounded-md bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 text-sm font-semibold hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-colors border border-blue-200 dark:border-blue-800",
                                        "View Research Details →"
                                    }
                                }
                            }
                            if !detail.sub_details.is_empty() {
                                ul {
                                    class: "list-[circle] list-inside ml-6 mt-1 space-y-1",
                                    for sub in &detail.sub_details {
                                        li {
                                            if let Some(label) = &sub.label {
                                                span { class: "font-bold text-gray-900 dark:text-gray-200", "{label}: " }
                                            }
                                            "{sub.content}"
                                            if let Some(link) = &sub.internal_link {
                                                " "
                                                a {
                                                    href: "{link}",
                                                    class: "text-blue-600 dark:text-blue-400 hover:underline font-medium",
                                                    "Learn more"
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

            div {
                class: "flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4",
                div {
                    class: "flex flex-wrap gap-2",
                    for tag in &experience.tags {
                        span {
                            class: "bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded dark:bg-blue-200 dark:text-blue-800",
                            "{tag}"
                        }
                    }
                }
                
                button {
                    onclick: move |_| is_expanded.toggle(),
                    class: "text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 font-medium text-sm focus:outline-none whitespace-nowrap",
                    if is_expanded() { "Show Less" } else { "Read More" }
                }
            }
        }
    }
}
