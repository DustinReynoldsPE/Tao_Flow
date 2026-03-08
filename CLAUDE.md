# Tao Flow

A multi-LLM system that flows like water, forged in Rust.

## Philosophy

This system is built on Taoist principles. Read `docs/philosophy.md` for the foundation.
The implementation plan is in `docs/implementation.md`. Refer to it for context on any work.

## Development

- **Language:** Rust (the metal vessel)
- **Async runtime:** Tokio
- **All work on branches.** Never push directly to main.
- **Branch naming:** `feature/<name>`, `fix/<name>`, `skills/<name>`

## Quality Gate

Before any PR, all three must pass (or use `/riverbank`):

```
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

## Project Structure

```
src/
  water/          # Core types: Vapor, Rain, Stream, River, Ocean
  watershed/      # Spring trait, VolumeSensor, spring implementations
    source/       # LlmSource trait, ClaudeCliSource, LlamaSource
  vessel/         # tmux session management (the window, not the mountain)
  confluence/     # Eddy detection, yielding, stream merging
  still_lake/     # Final refinement (the five questions)
  creation/       # Book, podcast, software creation flows
  config/         # Configuration loading
  flow.rs         # TaoFlow — the complete Rain to Ocean journey
  error.rs        # FlowError hierarchy
```

## Skills

Use `/riverbank`, `/spring`, `/vessel`, `/still-lake`, `/rain`, `/confluence`, `/flow`, `/reflecting-pool`, `/rising-mist`.
See `docs/implementation.md` "Skills: Teaching the Builder" for details.

## The Journey

This system was built phase by phase, each followed by reflection. The reflections carry lessons that no instruction can replace. Before starting a new phase, read:

1. `docs/philosophy.md` — the foundation
2. `docs/*reflections.md` — what was learned, in the order it was learned
3. `docs/implementation.md` — the vision ahead (pseudocode and prose, not code)

The creation of Tao Flow is itself the template for what the system will do. Philosophy first, then architecture, then implementation, then phases with reflection. The system should create the same way.

## Principles

- The compiler is the first master. Yield to its discipline.
- Tests are riverbanks. They don't create the flow; they ensure it stays in course.
- Water types are distinct. Stream cannot become Ocean without passing through River.
- Silence is valid. A dry spring (returning None) is wisdom, not failure.
- Build like water. When blocked, flow around. When in a valley, fill it and become a lake.

## The Lesson of Enough

*"In the practice of the Tao, every day something is dropped."* — Chapter 48

- **Do not add fields, types, or dependencies before they carry water.** Each should arrive with the phase that gives it meaning. An unused field is a vessel shaped before the water comes — it may be the wrong shape.
- **Do not explain in comments what the code already says.** If the code is clear, the comment is noise. If the code is unclear, fix the code. The strongest code teaches without words.
- **Do not quote the Tao Te Ching to justify a design decision.** If the design is sound, it needs no justification. If it is unsound, no quote will save it. The code should *embody* the Tao, not *talk about* it.
- **The implementation doc (`docs/implementation.md`) is a vision, not the truth.** The source code is the truth. When they diverge, update the doc or accept the divergence — but never mistake the map for the territory.
- **Know when to stop.** Three similar lines of code are better than a premature abstraction. A simple function is better than a clever one. Enough is enough.
