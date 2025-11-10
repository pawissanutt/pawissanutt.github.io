# Copilot Instructions for Personal Portfolio (Dioxus 0.7)

## Project Overview
This is a Dioxus 0.7 WebAssembly personal portfolio website for a Ph.D. researcher, built with Rust and Tailwind CSS. The goal is a high-performance, minimalist SPA showcasing research expertise, publications, and technical skills.

## Critical Architecture Patterns

### Dioxus 0.7 Specifics
- **NO legacy APIs**: `cx`, `Scope`, and `use_state` are removed in 0.7. Use `use_signal` for reactive state
- **Signal pattern**: Call signals like functions `count()` to read, use `.write()` for mutation, or `.with_mut()` for in-place updates
- **Component syntax**: Always use `#[component]` macro. Props must be owned types (use `String` not `&str`)
- **RSX formatting**: Elements use direct syntax. Conditionals/loops go directly in rsx: `for i in 0..5 { div { "{i}" } }`

### Asset System
All assets use `asset!()` macro with paths from project root:
```rust
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
```
Used in components: `img { src: HEADER_SVG }`

### Routing Structure
Routes defined in single enum with `#[derive(Routable)]`:
- Use `#[layout(Navbar)]` for shared layouts with `Outlet::<Route> {}` rendering child routes
- Dynamic segments: `#[route("/blog/:id")]` become struct fields `Blog { id: i32 }`
- Navigation: `Link { to: Route::Home {}, "Home" }`

## Development Workflow

### Build & Serve
```bash
dx serve                    # Default platform (web)
dx serve --platform desktop # For desktop builds
```
**No manual Tailwind setup needed** - Dioxus 0.7 auto-compiles `tailwind.css` in root

### Project Structure
```
src/main.rs         # Entry point with App, Route enum, and all components
assets/             # Static assets (CSS, images, icons)
tailwind.css        # Tailwind input (@import "tailwindcss")
assets/tailwind.css # Auto-generated output
design.md           # Complete design specification
AGENTS.md           # Dioxus 0.7 API reference
```

## Design System (from design.md)

### Visual Identity
- **Light mode**: White bg (#FFFFFF), near-black text (#111827), blue accent (#2563EB)
- **Dark mode**: Dark blue-gray bg (#111827), light gray text (#D1D5DB), vibrant blue (#3B82F6)
- **Typography**: Inter for body/headings, JetBrains Mono for code

### SPA Architecture
Single-page with smooth-scroll navigation to sections:
1. Navigation Bar (sticky)
2. Hero Section ("Above the fold" with name, title, CTAs)
3. Featured Research (OaaS highlight)
4. Publications, Experience, Skills sections

### Design Philosophy
"The Expert's Briefing" - minimalist, data-driven, performance-focused. Prioritize clarity and readability over flashy effects.

## Code Conventions

### Component Organization
Keep all components in `main.rs` for this small project. Pattern:
```rust
#[component]
fn ComponentName() -> Element {
    rsx! {
        div { class: "container", "content" }
    }
}
```

### State Management
Use `use_signal` for local state, `use_context_provider`/`use_context` for shared state:
```rust
let mut count = use_signal(|| 0);
// Read: count() or count.read()
// Write: *count.write() += 1 or count.with_mut(|c| *c += 1)
```

### Styling Approach
Dual CSS system:
- `assets/main.css` for app-wide base styles (body, reset, theme variables)
- Tailwind classes in RSX for component-specific styling

## Key Technical Decisions

### Why Dioxus + Wasm
Demonstrates expertise in modern, high-performance web tech. Compiles to WebAssembly for near-native speed.

### GitHub Pages Deployment
Target: Free, globally-distributed CDN. Build output goes to `dist/` for GitHub Actions workflow.

### Router Features
Enabled via `Cargo.toml`: `dioxus = { version = "0.7.1", features = ["router"] }`

## When Adding Features

1. **New routes**: Add variant to `Route` enum with proper attributes
2. **New sections**: Create component, add to appropriate route (likely `Home`)
3. **External links**: Use standard `a { href: "...", }` syntax
4. **Theme toggle**: Will need `use_signal` for theme state + CSS variables or Tailwind dark mode
5. **Interactivity**: Use event handlers like `onclick: move |_| { /* handler */ }`

## References
- Full design spec in `design.md`
- Dioxus 0.7 API reference in `AGENTS.md`
- Example routing/components in `src/main.rs`
