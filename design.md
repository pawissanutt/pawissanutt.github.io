# **Design Document: Pawissanutt Lertpongrujikorn**

## **1\. Project Overview & Goals**

### **1.1. Project Title**

Personal Portfolio Website for Pawissanutt Lertpongrujikorn

### **1.2. Primary Goals**

This website serves as the professional online presence for a Computer Science Ph.D. specializing in distributed systems, serverless computing, and cloud-native architecture. The primary goals are:

1. **Establish Authority:** Immediately convey expertise, credibility, and the high level of technical proficiency demonstrated by a Ph.D. and published researcher.  
2. **Highlight Key Research:** Elevate the "Object-as-a-Service (OaaS)" paradigm from a project line-item to a central, featured achievement.  
3. **Provide Proof:** Serve as a central hub for publications, technical projects, and professional links (Google Scholar, GitHub, LinkedIn).  
4. **Demonstrate Modern Skills:** The site itself will act as a project, showcasing skills in a high-performance stack (Rust/Dioxus/Tailwind).

### **1.3. Target Audience**

* Technical Recruiters (FAANG, cloud providers, high-tech companies)  
* Academic Peers & Researchers  
* Hiring Managers in R\&D and specialized engineering teams  
* Conference Reviewers and Committees

## **2\. Core Design Philosophy: "The Expert's Briefing"**

The design will be **minimalist, professional, and data-driven**. It must feel less like a "blog" and more like a high-quality "briefing" or "researcher's hub." The aesthetic is clean, fast, and authoritative. Flashy animations are avoided in favor of subtle, polished micro-interactions.

* **Core Principles:**  
  * **Clarity over Clutter:** Prioritize information hierarchy.  
  * **Performance:** The site must be extremely fast, reflecting the developer's focus on performance optimization.  
  * **Readability:** Content is dense. Typography is a top priority.  
  * **Subtle Polish:** Interactions feel modern and responsive.

## **3\. Technical Stack**

* **Framework:** Dioxus (Rust)  
  * **Reason:** Compiles to WebAssembly (Wasm) for near-native performance. The choice itself is a "flex" that demonstrates a deep understanding of modern, high-performance web technologies.  
* **Styling:** Tailwind CSS  
  * **Reason:** Utility-first for rapid, consistent design. The production build will be minimal, containing only the CSS used, which aids performance.  
* **Deployment:** GitHub Pages (via GitHub Actions)  
  * **Reason:** Provides a free, globally-distributed CDN for static files, ensuring fast load times worldwide. The CI/CD workflow will automate the entire build process.

## **4\. Visual Identity**

### **4.1. Color Palette**

The palette is simple, professional, and high-contrast.

* **Light Mode (Default):**  
  * **Background:** White (\#FFFFFF) or a very light gray (\#F9FAFB)  
  * **Text:** Near-black (\#111827)  
  * **Secondary Text (Subtitles, metadata):** Medium Gray (\#6B7280)  
  * **Accent (Links, buttons, highlights):** A strong, professional blue (\#2563EB)  
* **Dark Mode:**  
  * **Background:** Very dark blue/gray (\#111827)  
  * **Text:** Light gray (\#D1D5DB)  
  * **Secondary Text:** Medium-dark gray (\#9CA3AF)  
  * **Accent:** A slightly lighter, vibrant blue (\#3B82F6)

### **4.2. Typography**

* **Primary Font (Headings & Text):** Inter  
  * **Reason:** A highly-readable, modern sans-serif. It's crisp, professional, and scales beautifully from small text to large headlines.  
* **Monospace Font (Code snippets, if any):** JetBrains Mono  
  * **Reason:** Excellent readability for code.

## **5\. Site Architecture & Page Flow**

This is a **single-page application (SPA)**. The "pages" are logical sections on a single scrolling page. A simple navigation bar at the top will "jump" to the relevant section.

### **5.1. Component Breakdown**

#### **1\. Navigation Bar**

* **Content:**  
  * Left: Pawissanutt Lertpongrujikorn  
  * Right: About | Research | Publications | Experience | Skills  
* **Interaction:**  
  * Sticks to the top of the viewport on scroll.  
  * Links smoothly scroll to the corresponding section.  
  * Includes the **Light/Dark Mode Toggle** (a key feature).

#### **2\. Hero Section ("Above the Fold")**

* **Goal:** Establish authority in \< 5 seconds.  
* **Content:**  
  * **H1:** Pawissanutt Lertpongrujikorn  
  * **H2 (Sub-headline):** Ph.D., Researcher in Distributed Systems & Serverless Computing  
  * **Summary:** The 2-line summary from the resume.  
  * **CTA Buttons:**  
    * View Resume (PDF)  
    * Google Scholar  
    * GitHub  
    * LinkedIn

#### **3\. Featured Research (OaaS)**

* **Goal:** Highlight the most significant and unique research.  
* **Design:** A large, two-column "feature" section.  
* **Content:**  
  * **Title:** Featured Research: Object-as-a-Service (OaaS)  
  * **Summary:** A high-level 2-3 sentence summary of *what it is* and *why it matters*.  
  * **Key Links:** "Published In:" followed by a short list of the most prestigious publications (e.g., IEEE Transactions on Computers, ACM Symposium...).

#### **4\. Publications**

* **Goal:** Provide "proof" of research output.  
* **Design:** A clean, vertical list of "Publication Cards."  
* **Publication Card Component:**  
  * **Title:** Object as a Service: Simplifying Cloud-Native...  
  * **Metadata:** Lertpongrujikorn, P., & Salehi, M. A. (2025)  
  * **Venue:** IEEE Transactions on Computers  
  * **Links:** \[PDF\] \[DOI\] \[Google Scholar\]

#### **5\. Experience & Projects**

* **Goal:** Show professional and academic application of skills.  
* **Design:** A simple timeline or stacked "Experience Cards."  
* **Experience Card Component:**  
  * **Title:** Software Engineer  
  * **Company/Lab:** ByteArk (with dates: Jun 2019 \- Jul 2021\)  
  * **Description:** 2-3 key bullet points, re-written as sentences.  
  * **Keywords (Tags):** CDN, LL-HLS, CI/CD, Ansible  
* **Project: Action Engine:**  
  * This will be a "Project Card" styled similarly to the OaaS section but smaller.  
  * **Title:** Project: Action Engine  
  * **Description:** "Generates serverless workflows from natural language using LLMs."  
  * **Keywords (Tags):** LLMs, FaaS, Platform-Agnostic

#### **6\. Skills**

* **Goal:** Provide a scannable list of technical keywords.  
* **Design:** Categorized, plain-text lists. **No logos or progress bars.**  
* **Categories:** Technical, Programming Language, Cloud & DevOps, Database & Storage, Messaging.

#### **7\. Footer**

* **Content:**  
  * © 2025 Pawissanutt Lertpongrujikorn  
  * Built with Dioxus (Rust/Wasm) & Tailwind CSS.  
  * \[View Source on GitHub\] (This links to the portfolio's own repo).

## **6\. Micro-Interactions & "Tricks"**

Subtlety is key. These interactions add a layer of polish without being distracting.

1. **Light/Dark Mode Toggle:** The most important "feature." It will be an icon (sun/moon) in the nav, transitioning smoothly.  
2. **Hover States:**  
   * **Cards (Publication/Experience):** Subtle "lift" effect. (Tailwind: transition-all hover:shadow-lg hover:-translate-y-1)  
   * **Links & Buttons:** Subtle color/brightness transition. (Tailwind: transition-colors)  
3. **On-Scroll Fade-In:** As the user scrolls, section headings and cards will fade in and slide up slightly *once*. (Requires IntersectionObserver).  
4. **"Built With" Flex:** The footer line is a "trick" that makes the portfolio itself a testament to your skills.