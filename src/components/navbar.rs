use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "sticky top-0 z-50 bg-white dark:bg-gray-800 shadow-md text-gray-900 dark:text-gray-100",
            div {
                class: "container mx-auto px-4",
                div {
                    class: "flex justify-between items-center py-4",
                    div {
                        class: "text-lg font-bold",
                        "Pawissanutt Lertpongrujikorn"
                    }
                    div {
                        class: "hidden md:flex items-center space-x-8",
                        a { href: "#about", class: "hover:text-blue-500", "About" }
                        a { href: "#experience", class: "hover:text-blue-500", "Experience" }
                        a { href: "#education", class: "hover:text-blue-500", "Education" }
                        a { href: "#research", class: "hover:text-blue-500", "Research" }
                        a { href: "#publications", class: "hover:text-blue-500", "Publications" }
                        a { href: "#skills", class: "hover:text-blue-500", "Skills" }
                    }
                }
            }
        }
        // Spacer to offset fixed navbar height
        div { class: "pt-20" }
        Outlet::<Route> {}
    }
}
