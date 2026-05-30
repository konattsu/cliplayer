# Agents

This is the primary repo instruction file for both Copilot and Codex.

## Core Rules

- Treat this file as the first stop for repo-specific guidance.
- Use `pnpm` instead of `npm`.
- Be careful about where you read and edit code; position mistakes are easy here.
- Refer to `README.md` for the project overview.

## Frontend Implementation Notes

This project uses Angular.
AI models tend to adopt React / Next.js conventions for syntax, naming, folder structure, and architectural thinking more easily, because there are many React / Next.js examples in their training data.

Therefore, when generating or modifying frontend code, do not directly apply the following React / Next.js habits:

- file / folder structure
- component splitting granularity
- state management approach
- naming conventions
- coding style and responsibility separation

For each implementation, always verify whether it is appropriate according to Angular’s standard design, conventions, and best practices.
Even if something looks natural in React / Next.js, it can still be inappropriate for Angular.
