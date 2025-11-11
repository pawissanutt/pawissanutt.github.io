use dioxus::prelude::*;
use crate::data::{get_publications, Publication};

const DISSERTATION_PDF: Asset = asset!("assets/pawissanutt-dissertation.pdf");

#[component]
pub fn Publications() -> Element {
    let publications = get_publications();

    rsx! {
        section {
            id: "publications",
            class: "py-20 px-4 bg-gray-50 dark:bg-gray-900",
            div {
                class: "container mx-auto",
                h2 {
                    class: "text-3xl font-bold text-center mb-12 text-gray-900 dark:text-white",
                    "Publications"
                }
                div {
                    class: "flex flex-col gap-8",
                    DissertationCard {}
                    for pub_item in publications() {
                        PublicationCard { publication: pub_item }
                    }
                }
            }
        }
    }
}

#[component]
fn DissertationCard() -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md border border-blue-100 dark:border-blue-900/40",
            h3 {
                class: "text-xl font-bold text-gray-900 dark:text-white",
                "Ph.D. Dissertation"
            }
            p {
                class: "text-gray-600 dark:text-gray-400 mt-2",
                "Object Abstraction to Streamline Edge-Cloud-Native Application Development"

            }
            p {
                class: "text-gray-500 dark:text-gray-400 mt-1",
                em { "University of North Texas, 2025" }
            }
            div {
                class: "flex space-x-4 mt-4",
                a {
                    href: DISSERTATION_PDF,
                    class: "text-blue-500 hover:underline font-medium",
                    "Download PDF"
                }
            }
        }
    }
}

#[component]
pub fn PublicationCard(publication: Publication) -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md",
            h3 {
                class: "text-xl font-bold text-gray-900 dark:text-white",
                "{publication.title}"
            }
            p {
                class: "text-gray-600 dark:text-gray-400 mt-2",
                "{publication.authors}"
            }
            p {
                class: "text-gray-500 dark:text-gray-400 mt-1",
                em { "{publication.venue}" }
            }
            div {
                class: "flex space-x-4 mt-4",
                if publication.pdf_link != "#" {
                    a { href: "{publication.pdf_link}", class: "text-blue-500 hover:underline", "[PDF]" }
                }
                a { href: "{publication.doi_link}", class: "text-blue-500 hover:underline", "[DOI]" }
            }
        }
    }
}
