# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Shinnku-com is a monorepo containing a galgame resource website with three main components:

- **Frontend**: Next.js 15 app with TypeScript, Tailwind CSS, and MDX support
- **Backend**: Rust web service using Axum framework with Redis integration
- **AI**: Python-based AI service using Falcon ASGI with LangChain and ChromaDB

## Development Commands

### Frontend (in `frontend/` directory)

- **Dev server**: `pnpm run dev` (uses Turbopack)
- **Build**: `pnpm run build`
- **Lint**: `pnpm run lint` (ESLint with TypeScript)
- **Format**: `pnpm run format` (Prettier)

### Backend (in `backend/` directory)

- **Run server**: `cargo run -p server`
- **Build**: `cargo build`
- **Test**: `cargo test`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy` (uses strict Clippy rules from Cargo.toml)

### AI Service (in root directory)

- **Install dependencies**: `uv sync` or `pip install -r requirements.txt`
- **Run AI service**: `python -m ai.serve` or `shinnku-ai-serve`
- **Format**: `black ai/` and `isort ai/`
- **Test**: `pytest` (with coverage via `pytest --cov=ai`)

## Architecture Overview

### Frontend Structure

- **App Router**: Next.js 15 with app directory structure
- **Internationalization**: Built-in routing for `zh-cn`, `zh-tw`, and `en-us` locales
- **UI Components**: Radix UI primitives with Tailwind CSS styling
- **Content**: MDX-based blog posts in `posts/` directory
- **Key API Routes**: `/api/wiki`, `/api/aiintro`, `/api/image-proxy`

### Backend Architecture

- **Framework**: Axum web framework with Redis for caching
- **Configuration**: TOML-based config in `backend/config.toml`
- **Custom Fuse Search**: Fuzzy search implementation in `backend/fuse/` crate
- **Workspace Structure**: Two crates - `server` (main API) and `fuse` (search library)
- **Server runs on**: `127.0.0.1:2999`

### AI Service

- **Framework**: Falcon ASGI for HTTP API
- **Vector Database**: ChromaDB with BAAI/bge-large-zh-v1.5 embeddings
- **Endpoints**: `/intro` (game introductions), `/findname` (game search)

## Code Conventions

### TypeScript/Frontend

- 2-space indentation
- Absolute imports with `@/` path mapping
- React 19 patterns with app directory structure

### Rust/Backend

- Strict Clippy configuration enforced (see `backend/Cargo.toml`)
- 4-space indentation via rustfmt
- `Result<T, AppError>` for error handling
- Document public functions and avoid `unwrap()` in production

### Python/AI

- Black formatting (88 character line length)
- Type hints required for all functions
- async/await for I/O operations
