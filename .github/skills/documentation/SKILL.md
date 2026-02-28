---
name: documentation
description: Documentation skill for writing clear and concise documentation for the project.
---

# Documentation Skill

This skill applies **only** to changes made under the `docs/` directory. The instructions here are **not** meant for inline code comments or code files.

## Goal

The main objective of documentation is to clearly and accurately communicate the system's structure, data flows, and design decisions to other developers and to your future self. Good docs reduce onboarding time, prevent misunderstandings, and act as a reliable reference.

When authoring or updating docs, keep the following principles in mind:

- **Keep it up to date.** When you change behavior or architecture, update the corresponding documentation immediately; outdated docs are worse than none.
- **Stay implementation‑driven.** Describe what the code actually does; avoid speculation or high‑level platitudes.
- **Be concise.** Avoid excessive code listings—use short snippets only to illustrate key points, not entire files.
- **Use visual aids wisely.** Mermaid diagrams, flowcharts, and other markdown features are helpful, but they should support the narrative rather than replace it.
- **Organize for readability.** Break longer topics into sections, add headings, and use bullet lists to make the text scannable.

## Style Tips

- **Write for other humans.** Imagine explaining the system to a colleague who has never seen the code before.
- **Link to code.** Where appropriate, reference specific files or functions so readers can quickly jump to the implementation.
  - When you link to this repository's code, do not use `https` links; instead, use relative links to the codebase (e.g., `../src/module/file.rs`).
- **Leverage markdown features.** Use headings, emphasis, tables, and lists to structure content; don’t overuse them.

Well‑written docs are a cornerstone of a healthy codebase. Treat them with the same care you give to code.
