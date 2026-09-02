# AI World Desktop Pet Architecture

The application follows the patterns studied from `agent-pet`, `agent-terrarium`, `agents-in-the-office`, and `hermes-quest`: Tauri owns the desktop shell, Rust owns the simulation and event pipeline, and React is a state renderer.

`Real time -> Scheduler -> Candidate events -> LLMProvider proposal -> JSON/schema/rule/state/cooldown validation -> World Engine -> SQLite + Markdown memory -> Tauri event -> React renderer`

The LLM has no database access. `EventProposal` is the only boundary. The Rust engine clamps XP and relationship deltas, rejects unknown types and oversized text, and calculates level from XP. Shared experiences and personality evidence are stored independently so future reflection can trace why a trait changed.

The frontend browser preview uses a local persistence fallback for fast UI development. The Tauri build exposes `get_world`, `apply_proposal`, and `test_provider` commands; provider credentials should be stored in the platform keychain/Credential Manager integration before release packaging.
