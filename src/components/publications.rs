use dioxus::prelude::*;
use crate::data::{get_publications, Publication};

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
                    for pub_item in publications() {
                        PublicationCard { publication: pub_item }
                    }
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
