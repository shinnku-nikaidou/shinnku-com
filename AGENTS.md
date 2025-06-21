# Repository Guidelines for Contributors

This monorepo hosts a Next.js frontend and a Rust backend.

## Directory Overview

- `frontend/` – Next.js 15 project written in TypeScript.
- `backend/` – Rust web service using the Axum framework.

## Development Setup

- Use **pnpm** for managing frontend dependencies.
- Use **Cargo** for the Rust backend.

## Formatting and Linting

- Format TypeScript/JavaScript and CSS using `pnpm run format`.
- Build the frontend with `pnpm build`.
- only if you change the backend, run the code below:
- Format Rust code with `cargo fmt` and verify builds with `cargo check`.
- Lint Rust code with `cargo clippy`.

## Important Notes

- `frontend/node_modules/` and `backend/target/` are intentionally ignored and should not be committed.
- TypeScript/TSX files follow two‑space indentation as defined in `.editorconfig`.
- Rust files use the default `rustfmt` style (four‑space indentation).
- Update `README.md` if you change setup steps or add major features.
