# Aoi's World

A persistent AI character world for Windows 10/11 and macOS. Aoi lives on real local time, accumulates memories, relationships, skills, items, goals, XP, and evidence-backed personality changes.

## Project Overview
Tauri v2 + Rust + React + TypeScript + Vite. The desktop window is frameless, transparent, resizable, and always on top. The browser preview is also usable.

## Setup
Install Node.js 18+, npm, Rust stable, and platform Tauri prerequisites. On macOS install Xcode Command Line Tools. On Windows install WebView2 and Visual Studio C++ build tools.

```sh
npm install
npm run dev
npm run build
npm run tauri dev
npm run tauri build
cargo test --manifest-path src-tauri/Cargo.toml
```

## Sprite Format
Import a transparent PNG atlas. Default grid is 8 columns x 9 rows, but dimensions are configuration values and must be validated from the image. Animation states are configured as frame indexes rather than hardcoded image sizes.

## Character Files and World Rules
Edit `character/character.md`, `personality.md`, `relationships.md`, and `important_people/*.md`. Edit `world/rules.md` for readable rules. The engine reloads these files before event generation and preserves user edits.

## Custom LLM API
Configure an OpenAI-compatible `Base URL`, `Model`, and API key in Settings. The API contract is `POST /chat/completions`; the provider is intentionally abstracted behind an LLM interface in the Rust boundary. Never commit or log the API key. Production key storage should use macOS Keychain or Windows Credential Manager.

## Event System
Normal checks are scheduled every 20-90 minutes. Important-event candidate windows are probabilistic every 4-8 hours and balance daily count, cooldown, context, goals, relationships, and elapsed time. Offline mode uses deterministic normal events and cannot make major relationship changes.

## Personality Evolution
Only meaningful events can create personality evidence. Each trait delta is clamped to a small range and references an event. Repeated shared experiences gradually move relationships and can promote NPCs to important people.

## Memory and SQLite
Raw events remain in SQLite forever. Layered memories keep prompts bounded; a summary may be created every 50 events. SQLite is structured source of truth; Markdown is human/AI-readable memory.

## Import / Export and Troubleshooting
The Settings surface is reserved for JSON world export, Markdown export, import, reset confirmation, provider testing, sidebar count, animation FPS, scale, and startup behavior. If the API fails, the world continues offline. If the desktop shell does not start, run `npm run build` first and verify Tauri prerequisites.
