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
  confluence/     # Eddy detection, yielding, stream merging
  still_lake/     # Final refinement (the five questions)
  creation/       # Book, podcast, software creation flows
  config/         # Configuration loading
  error.rs        # FlowError hierarchy
```

## Skills

Use `/riverbank`, `/spring`, `/vessel`, `/still-lake`, `/rain`, `/confluence`, `/flow`.
See `docs/implementation.md` "Skills: Teaching the Builder" for details.

## Principles

- The compiler is the first master. Yield to its discipline.
- Tests are riverbanks. They don't create the flow; they ensure it stays in course.
- Water types are distinct. Stream cannot become Ocean without passing through River.
- Silence is valid. A dry spring (returning None) is wisdom, not failure.
- Build like water. When blocked, flow around. When in a valley, fill it and become a lake.
