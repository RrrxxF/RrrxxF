# DesignWorkbench agent instructions

Read `docs/DesignWorkbench_完整开发需求与Codex执行手册_v1.0.md` before each stage. Complete only the requested stage and report actual results against its acceptance criteria.

Run `git status` and inspect existing code before editing. Preserve user assets and others' changes; back up data and use migrations for schema changes. Make clear stage commits, lock dependencies, and never commit credentials.

Build real UI-to-Rust-to-SQLite paths. Fixtures are only examples. Verify uncertain permissions and platform behavior with a small real test, and state the environment and result.

Document each code module's purpose and important constraints with concise comments. Choose algorithms based on measured requirements and validate correctness and performance; avoid complexity with no benefit.

At each stage end, compile and test, update docs, capture a screenshot where a GUI is available, and record acceptance status. If GitHub, Figma, or Windows access is unavailable, provide the exact user action and continue independent work. Never request passwords or tokens in chat.

Do not enable paid services or upload private assets. Keep data local by default. Route file and SQLite access through Rust services with minimal capabilities. Preserve validation, safety checks, and user data even when they complicate a demo.
