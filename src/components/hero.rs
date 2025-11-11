use dioxus::prelude::*;


const RESUME_PDF: Asset = asset!("assets/pawissanutt-resume.pdf");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "about",
            class: "relative min-h-screen flex items-center justify-center text-center overflow-hidden scroll-mt-20",
            div { class: "absolute inset-0 -z-10 bg-gradient-to-br from-sky-100 via-indigo-100 to-white dark:from-slate-950 dark:via-indigo-950 dark:to-black animate-gradient" }
            div {
                class: "pointer-events-none absolute -top-40 -right-32 h-96 w-96 rounded-full blur-3xl opacity-80 animate-float-slow",
                style: "background: radial-gradient(circle at center, rgba(59,130,246,0.35), rgba(37,99,235,0.0) 70%);"
            }
            div {
                class: "pointer-events-none absolute -bottom-44 -left-32 h-[28rem] w-[28rem] rounded-full blur-3xl opacity-70 animate-float-slow",
                style: "background: radial-gradient(circle at center, rgba(16,185,129,0.28), rgba(59,130,246,0.0) 75%); animation-delay: 1.5s;"
            }
            div {
                class: "relative z-10 flex flex-col items-center px-6 pt-24 pb-12",
                h1 {
                    class: "text-4xl md:text-6xl font-bold text-gray-900 dark:text-white",
                    "Pawissanutt Lertpongrujikorn"
                }
                h2 {
                    class: "text-xl md:text-2xl mt-4 text-gray-800 dark:text-gray-200",
                    "Ph.D., Researcher in Distributed Systems & Serverless Computing"
                }
                p {
                    class: "mt-6 max-w-2xl text-gray-600 dark:text-gray-400",
                    "Computer Science & Engineering Ph.D. candidate with 4+ years designing high-performance distributed and serverless systems. Originator of the Object-as-a-Service (OaaS) paradigm published across leading venues to streamline cloud-native development. Deep expertise in system architecture, performance optimization, and solving scaling challenges using Kubernetes, Java, Rust, and Python."
                }
                div {
                    class: "flex flex-wrap justify-center gap-4 mt-8",
                    a { href: RESUME_PDF, class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded", "View Resume" }
                    a { href: "https://scholar.google.com/citations?user=Y6jQnzcAAAAJ", class: "bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded", "Google Scholar" }
                    a { href: "https://github.com/pawissanutt", class: "bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded", "GitHub" }
                    a { href: "https://linkedin.com/in/pawissanutt", class: "bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded", "LinkedIn" }
                }
            }
        }
    }
}
