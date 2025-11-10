use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "bg-gray-800 text-white py-8 px-4",
            div {
                class: "container mx-auto text-center",
                p { "© 2025 Pawissanutt Lertpongrujikorn" }
                p {
                    class: "mt-2",
                    "Built with Dioxus (Rust/Wasm) & Tailwind CSS. "
                    // a {
                    //     href: "https://github.com/pawissanutt/personal-web",
                    //     class: "text-blue-400 hover:underline",
                    //     "[View Source on GitHub]"
                    // }
                }
            }
        }
    }
}
