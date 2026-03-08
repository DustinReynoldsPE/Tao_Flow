# Implementation: From Philosophy to Code

*"A journey of a thousand miles starts under one's feet."*
*-- Tao Te Ching, Chapter 64*

---

## The Uncarved Block

This document is a vision -- the shape the water may take. The code is the water. Where they diverge, trust the code. The source of truth lives in `src/`. This doc describes *what* flows and *why* -- the code speaks for *how*.

---

## Why Rust: The Metal Element

*"To understand the universe... the Five Elements, symbolized by water, fire, wood, metal, and earth."*
*-- Hua Hu Ching, Chapter 61*

The system's flow is water. But what channels water? What holds it without yielding, so the water may yield freely? **Metal.** Rust is metal refined -- the vessel forged to hold the Tao's flow.

### The Riverbed Must Be Hard

*"Men are born soft and supple; dead, they are stiff and hard. Plants are born tender and pliant; dead, they are brittle and dry. Thus whoever is stiff and inflexible is a disciple of death. Whoever is soft and yielding is a disciple of life."*
*-- Tao Te Ching, Chapter 76*

One reads this and asks: is not Rust the way of death -- rigid, strict, unyielding? Look deeper. The *water* must be soft. The *riverbed* must be hard. If the riverbed were soft, the water would have no channel. It would dissipate into mud. Rust is not the water. Rust is the granite through which the river carves its eternal course.

### The Compiler as Master

*"She doesn't scheme to become a leader, but quietly shoulders whatever responsibilities fall to her... scolding them to awaken them, directing the streams of their lives toward the infinite ocean of the Tao."*
*-- Hua Hu Ching, Chapter 80*

The Rust compiler is the master who scolds to awaken. It refuses to compile what is unsafe. It refuses to let data races corrupt the stream. It refuses to let memory leak from the vessel. This is not rigidity -- it is virtue enforced at the deepest level. The developer who submits to the compiler's discipline emerges with code that is **correct by construction**.

### Tests as Riverbanks

The architecture document says: *"Tests: The riverbanks -- they don't create the flow, they ensure it stays in course."*

This system needs tests for two things:
1. **The things it creates** -- the books, podcasts, and software that flow through it
2. **The system itself** -- the watershed, the confluence, the still lake

Rust provides riverbanks at multiple levels:

- **The type system** -- compile-time tests that never need running. A `Stream` cannot be confused with a `River`. An `Eddy` must be resolved before it becomes part of the `Ocean`. The compiler enforces the water's journey.
- **Unit tests** -- inline with code, testing each spring, each confluence, each settling. `#[test]` lives beside the code it tests, like the bank beside the river.
- **Integration tests** -- in `tests/`, testing the full flow from Rain to Ocean. Does water that enters as a storm emerge as clarity?
- **Property-based tests** -- with `proptest`, testing that the system's invariants hold across thousands of random inputs. Like testing that water always flows downhill, no matter the terrain.
- **The borrow checker** -- a permanent, tireless test of memory safety. No garbage collector pausing the flow. No dangling references corrupting the stream. Wu wei applied to memory: the compiler does nothing extra, yet nothing is left unchecked.

### Zero-Cost Abstractions as Wu Wei

*"The Master does nothing, yet he leaves nothing undone."*
*-- Tao Te Ching, Chapter 38*

Rust's abstractions cost nothing at runtime. A `trait Spring` compiles to direct function calls. An `enum WaterState` compiles to a simple tag. The system pays only for what it uses -- no interpreter, no garbage collector, no runtime overhead standing between the input and the output. The code executes as directly as water flowing downhill.

---

## Technology Choices

### Core Runtime: Rust

Metal channels water. Rust's ownership model ensures that every drop of data flows through exactly one path at a time -- no duplication, no corruption, no waste. The type system models the water states precisely, and the compiler ensures every state transition is valid.

### Async Runtime: Tokio

Water flows simultaneously through all channels of a watershed. Tokio provides the concurrent flow -- lightweight tasks for each spring, each responding to rain at the same moment. Like `asyncio` but without the GIL, without the overhead, with true parallelism when the terrain demands it.

### LLM Integration: CLI-First, Provider-Agnostic

*"The supreme good is like water, which nourishes all things without trying to. It is content with the low places that people disdain."*
*-- Tao Te Ching, Chapter 8*

The low place that people disdain is the CLI that's already on the machine. A Claude Max user has the river already flowing -- the system should drink from it, not reach upward to API endpoints and pay for water that's already falling as rain.

**The primary provider is `claude -p`** -- Claude Code's print mode. Each spring is a CLI invocation with its own system prompt and model. `tokio::process::Command` spawns them concurrently. No API keys. No per-token pricing. No artificial barriers.

**For persistent sessions, tmux sustains the springs.** Each spring becomes a tmux window running an interactive `claude` process. The session remembers the conversation naturally -- vapor flows without explicit management. The mountain spring keeps its depth across exchanges. The desert spring stays quick.

```
tmux session: tao-flow
  window 0: mountain  (claude --model opus, persistent)
  window 1: desert    (claude --model haiku, persistent)
  window 2: forest    (claude --model sonnet, persistent)
```

This architecture provides:
- **No cost beyond subscription** -- Claude Max users already paid for the river
- **Natural vapor** -- tmux sessions remember conversation without explicit context management
- **Observability** -- `tmux attach -t tao-flow` to watch the springs flow in real time
- **Resilience** -- tmux sessions survive terminal disconnects
- **Debugging** -- switch to any spring's window to see its full conversation

The `LlmSource` trait is the underground aquifer -- the hidden water source that feeds each spring. Two sources fill it:
- `ClaudeCliSource` -- stateless `claude -p` calls (the default, the natural spring)
- `LlamaSource` -- llama.cpp server via its OpenAI-compatible API (for local models)

**tmux is the vessel, not the source.** It provides the walls of the space each agent occupies. Each window allows the user to perceive the agent -- much like you can see the mountain outside of a window, but the window is not the mountain. The vessel carries the conversation naturally; the spring does not need to carry its own memory.

### Serialization: Serde

Serde is water for data -- it flows JSON, YAML, MessagePack, anything. Configuration, API payloads, and internal messages all serialize naturally.

---

## Project Structure

Files that exist today. Future phases will add to this naturally.

```
src/
  main.rs                       # Entry point
  lib.rs                        # Library root -- re-exports
  flow.rs                       # TaoFlow -- Rain to Ocean
  error.rs                      # FlowError

  water/                        # The water types (compiler-enforced lifecycle)
    vapor.rs                    # Context carried between flows
    rain.rs                     # User input + volume + minerals
    stream.rs                   # Individual spring response
    river.rs                    # Merged output from confluence
    ocean.rs                    # What the user receives

  watershed/                    # Where rain meets springs
    spring.rs                   # Spring trait + SpringConfig
    volume_sensor.rs            # Classifies rain volume by word count
    source/                     # The underground aquifer
      mod.rs                    # LlmSource trait + mock sources
      claude_cli.rs             # claude -p invocations
      llama.rs                  # llama.cpp OpenAI-compatible API
    springs/                    # Individual springs
      mountain.rs               # Deep reasoning
      desert.rs                 # Speed, directness
      forest.rs                 # Creativity, narrative, empathy

  confluence/                   # Where streams merge
    eddy.rs                     # Eddy, EddyNature, Position, Resolution types
    detection.rs                # EddyDetector -- finds divergence between streams
    yielding.rs                 # YieldingProtocol -- resolves eddies through yielding
    pool.rs                     # ConfluencePool -- detect, yield, weave

  vessel/                       # tmux session management (not yet connected)
    tmux.rs                     # TmuxVessel -- persistent spring sessions

  still_lake/                   # Phase 5 placeholder
  creation/                     # Phase 6 placeholder
  config/                       # Config loading (inline for now)
```

---

## Core Abstractions

### Water Types

The system models data as water moving through a cycle. Each type is distinct -- the compiler enforces that a `Stream` cannot be confused with a `River`, and a `River` cannot reach the user without becoming an `Ocean`.

```
Vapor → Rain → Stream(s) → River → Ocean
  ↑                                   |
  └───────── update_vapor ────────────┘
```

- **Vapor** -- context carried between flows. Conversation history accumulates naturally.
- **Rain** -- user input with its vapor, a volume classification, and mineral tags for spring affinity.
- **Volume** -- how many springs should respond: Droplet (one), Shower (two), Downpour/Storm (all).
- **Stream** -- a single spring's response, carrying its source name, content, clarity, and depth.
- **Eddy** -- a point of divergence between streams. Has a topic, the positions taken, and a nature (Factual, Interpretive, Stylistic, Structural).
- **River** -- merged output from the confluence. Carries content, which tributaries contributed, any eddies detected, and a clarity score.
- **Ocean** -- what the user receives. Content only. By the time water reaches the ocean, it should be clear.

#### Clarity: The Signal from Confluence to Still Lake

In nature, a mountain spring produces clear water. When streams merge at confluence, sediment stirs -- clarity drops. A still lake lets sediment settle -- clarity returns.

River clarity carries this signal. A single stream passes through at 1.0 (crystal clear). A clean multi-stream merge is 0.8 (slightly muddied by weaving). Eddies will lower it further. The Still Lake (Phase 5) will read clarity to know how much polishing the water needs. Ocean has no clarity field -- by the time water arrives, the lake has settled it.

#### Minerals: Infrastructure Awaiting Its Caller

Springs declare affinities (what they resonate with). Rain carries minerals (tags). When minerals match affinities, a spring's relevance rises above the base score. The infrastructure is built and tested, but no production code yet populates minerals. A mineral classifier will give this system its water.

### The Spring

The Spring trait is the empty pot -- it defines the shape and leaves each spring free to fill it with its own nature.

```
trait Spring:
    name() → who am I
    nature() → what do I naturally provide
    sense_relevance(rain) → how strongly do I resonate with this input (0.0 to 1.0)
    respond(rain) → my response, or None if I have nothing to contribute
```

Each spring holds a `SpringConfig` (name, nature, affinities) and a `LlmSource` (the underground aquifer that produces the actual LLM response). The config's `sense_relevance` starts at 0.3 (every spring has something to offer) and increases with matching minerals, capped at 1.0. Below 0.2, the spring stays dry -- silence is wisdom.

Three springs flow today: Mountain (depth, analysis), Desert (speed, directness), Forest (creativity, narrative, empathy). The `Send + Sync` bounds ensure springs flow concurrently without data races.

### The Watershed

The watershed does not decide where rain goes. It has a shape, and water follows that shape.

```
receive_rain(rain):
    volume = sense volume from rain (sync, word-count based)
    active_springs = activate springs by volume
    streams = all active springs respond concurrently
    filter out dry springs
    return streams
```

Volume determines activation: Droplet activates only the desert spring. Shower activates two. Downpour and Storm activate all. Springs respond concurrently -- the watershed does not sequence them.

### The Confluence

Where streams merge into a river.

```
merge(streams, rain_input):
    if no streams → empty river
    if one stream → pass through untouched (wu wei, clarity 1.0)
    if multiple → weave via LLM integrator (clarity 0.8)
```

The confluence holds its own LLM source. When multiple streams arrive, it builds a prompt containing all perspectives and asks the integrator to weave them into one voice. The result should read as a single coherent response, not a committee. It does not mention sources by name.

Eddy detection arrives with Phase 4 (Yielding). For now, the confluence weaves and records which springs contributed.

### The Still Lake

Phase 5. The module exists as a placeholder -- the shape of the vessel, waiting for water.

When it arrives, the lake will read `River::clarity` to know how much settling is needed. High clarity means gentle polish. Low clarity (many tributaries, unresolved eddies) means deeper settling. The lake asks the five questions -- clarity, wholeness, kindness, truth, simplicity -- and produces an Ocean.

---

## The Main Flow

TaoFlow orchestrates the complete journey from rain to ocean.

```
flow(user_input):
    rain = Rain from user_input + vapor (context from previous flows)
    streams = watershed.receive_rain(rain)
    if no streams → Drought error
    river = confluence.merge(streams, rain_input)
    ocean = Ocean from river content
    update vapor with (user_input, ocean_content)
    return ocean content
```

The water cycle: after each flow, the user's input and the system's response are pushed into vapor as conversation history. The next rain carries this vapor, so springs see the full conversation. Context accumulates naturally.

The Still Lake is not yet in the flow -- river becomes ocean directly. When Phase 5 arrives, the lake will sit between confluence and ocean, reading river clarity to know how much polishing is needed.

---

## Testing Philosophy: The Riverbanks

*"The ancient Masters were profound and subtle. Their wisdom was unfathomable."*
*-- Tao Te Ching, Chapter 15*

Testing in this system operates at four levels, each a different kind of riverbank:

### Level 1: The Compiler (The Bedrock)

The type system prevents entire categories of errors without a single test:
- A `Stream` cannot be delivered to a user -- it must become `Ocean` first
- An `Eddy` must declare its `EddyNature` -- no unclassified conflicts
- `Send + Sync` bounds on `Spring` prevent data races between concurrent springs
- Exhaustive `match` on `Volume` ensures every rain level is handled
- The borrow checker ensures no spring holds stale references to rain that has already flowed past

These are tests that run at compile time, always, without effort. Wu wei.

### Level 2: Unit Tests (The Banks)

Each module carries its own tests inline. The things worth testing:

- A dry spring returns None -- silence is wisdom, not failure
- A single stream needs no confluence -- wu wei
- Volume sensor classifies correctly across the spectrum
- Eddy detection finds contradictions between streams (Phase 4)
- Springs carry conversation context from vapor
- Three streams merge into one river through the confluence
- Vapor accumulates across multiple flows

Mock sources (`MockSource`, `EchoSource`, `DrySource`) replace real LLMs in tests. Springs and the confluence pool accept any `LlmSource`, so mocks flow through the same paths as real sources.

### Level 3: Integration Tests (The Watershed Boundary)

Full flow tests that verify rain becomes ocean:

- Simple input flows through the system and produces non-empty output
- Vapor carries context between flows (the water cycle)
- Drought is detected when all springs are dry
- Three springs merge correctly for complex input
- Droplet volume activates only the desert spring

### Level 4: Property-Based Tests (The Laws of Nature)

Properties that must hold regardless of input -- the invariants of water:

- Water always reaches the ocean -- no input should cause the system to hang or crash
- The ocean is never turbid -- output should not contain system internals (prompt text, type names, debug info)
- Vapor accumulates -- context grows with each flow

These arrive when `proptest` carries water (currently not in dependencies).

### Testing the Creations

Phase 6. Each creation type (book, podcast, software) will need its own tests. The system that creates software must create tested software -- tests creating tests, the recursive riverbank.

---

## System Prompts (The Riverbed)

System prompts are inline `const` strings within each spring and the confluence pool. They are the riverbed -- they shape the water without being the water. Each spring's prompt defines its nature: the mountain is deep and analytical, the desert is fast and direct, the forest is warm and creative. The confluence prompt tells the integrator to weave without naming sources -- the river should read as one voice.

---

## Configuration

Springs are currently configured in code -- each spring is constructed with a `SpringConfig` (name, nature, affinities) and an `LlmSource`. External configuration (YAML, environment) will emerge when the number of springs outgrows inline construction. Dependencies arrive when they carry water.

---

## Deployment

### The Minimal Viable Watershed

Start small. Like a spring emerging from rock.

All work flows through branches. Main is the ocean floor -- stable, settled, tested. Each feature emerges as a branch, is refined through PR review, and merges only when clear. Use `/flow` to guide the full cycle.

**Phase 1: The Vessel** *(complete)*
Set up the Rust project. Define the water types. Implement the `Spring` trait. Write the first tests. The compiler is the first master. `cargo test` passes green. CI enforces the riverbanks on every push. Skills are defined: `/riverbank`, `/spring`, `/vessel`, `/still-lake`, `/rain`, `/confluence`, `/flow`.

**Phase 2: Two Springs** *(complete)*
Mountain and Desert implement the Spring trait. `ClaudeCliSource` uses `claude -p` -- the natural spring. `LlamaSource` connects to local llama.cpp servers. tmux sustains the vessel. Simple merge selects the deepest stream. Watershed dispatches concurrently.

**Phase 3: The Confluence** *(complete)*
Forest Spring joins mountain and desert -- three streams now flow. The Confluence Pool weaves multiple perspectives into one river through an LLM integrator. Single streams pass through untouched (wu wei). `simple_merge` is dropped. The `/reflecting-pool` skill emerges -- the system learns to look inward.

**Phase 4: Yielding** *(complete)*
When three streams merge, they sometimes disagree. `EddyDetector` analyzes streams for divergence and classifies each eddy (Factual, Interpretive, Stylistic, Structural). `YieldingProtocol` asks each position to find truth in the other -- resolution through yielding, not voting. `ConfluencePool` orchestrates: detect, yield, weave. Clarity now varies with turbulence -- resolved eddies cost less clarity than unresolved ones. Graceful degradation at every step: if detection or yielding fails, the system falls back to clean weaving.

**Phase 5: The Still Lake**
The final refinement pass. The lake receives the river -- its content, its clarity score, and its eddies. Clarity is now a living signal: 1.0 for a single stream (crystal clear), 0.8 base for a multi-stream merge, reduced by eddies (resolved cost 0.05, unresolved cost 0.1, floor at 0.3). The lake reads this to calibrate its depth of engagement -- high clarity means gentle polish, low clarity means deeper settling.

The lake also receives the eddies themselves -- but only engages with unresolved ones. Resolved eddies already found their truth through yielding; the lake need not revisit them. An unresolved factual eddy (the springs disagreed on a verifiable fact and yielding could not resolve it) is a different problem than a stylistic one. The lake is the last chance for clarity before the ocean.

The lake asks the five questions (clarity, wholeness, kindness, truth, simplicity) and produces an Ocean. The flow is now complete for a single pass: Rain → Watershed → Confluence → Still Lake → Ocean.

When the water is already clear (clarity 1.0, a single stream), the lake does nothing -- wu wei. If settling fails, the river content reaches the ocean unchanged -- graceful degradation. The water always reaches the ocean.

*(Updated after Phase 5: the lake now engages with unresolved eddies only, settling depth is calibrated to clarity, and wu wei applies to clear water.)*

**Phase 6: The Return**
*"Return is the movement of the Tao."* -- Chapter 40

The water cycle must cycle. A Storm-level request cannot be answered in one pass. Phase 6 makes the flow recursive:

1. **Decomposition** -- a River that is too wide and shallow is broken into parts. Each part becomes new Rain, carrying the context of the whole.
2. **Recursive flow** -- each part flows through the watershed independently. Springs respond, streams merge, rivers form. This is the same flow, operating at a finer grain.
3. **Higher confluence** -- the sub-rivers merge at a higher level. The same ConfluencePool weaves them, but now it weaves refined parts rather than raw streams.
4. **Termination** -- clarity is the signal. A river with clarity above a threshold has settled enough to reach the ocean. Below that, another cycle is warranted. If the Still Lake can polish the river to clarity, one cycle is enough. If not, the river returns to rain.

**Yielding memory.** When the water cycle actually cycles, the springs respond multiple times to related sub-problems. Phase 4's reflection revealed: yielding should change the one who yields. Each spring should carry a memory of its yieldings -- loaded into its prompt so lessons persist across cycles. The mountain that yielded to the forest in the first cycle responds differently in the second -- not by becoming the forest, but by being a mountain that knows the forest has something true to say. The risk is over-integration: if springs converge, the productive tension that creates eddies disappears. Springs must remain partial but *aware* of the other partials. Chapter 22: *"If you want to become whole, let yourself be partial."*

**Human guidance is a spectrum.** The system yields to the human's natural level of engagement. The pause points are the boundaries between recursive cycles: after decomposition (show the structure), after each sub-river completes (show progress), after higher confluence (show the assembled whole). The human can engage at any of these or none. Vapor carries the human's presence -- how much they've guided informs how much the system pauses.

The system should create the way it was created. Tao Flow was built through artifacts -- philosophy first, then architecture, then implementation plan, then phases of work. A Storm-level request follows the same pattern.

*(Updated by `/rising-mist` after Phase 4: yielding memory now has a home -- it belongs here, where recursive flow gives it leverage. Termination through clarity is concrete, not abstract. Human pause points are named.)*

**Phase 7: Creation**
Not separate machinery. With recursive flow in place, creation is what happens naturally when Storm volume triggers the water cycle to actually cycle. A book is a Storm with book-shaped riverbeds. Software is a Storm with code-shaped riverbeds. The specialization lives in system prompts, not in separate creation modules. The `creation/` directory may dissolve into the watershed itself.

What Phase 7 actually is: the phase where the system meets real use. New riverbeds (system prompts for specific creation types). New pause-point patterns (where different kinds of creation need human guidance). New skills that emerge as creation patterns solidify. The machinery is the same machinery. The water is the same water. Only the riverbed changes shape.

*(Updated by `/rising-mist` after Phase 4: Phase 3's reflection suggested creation might not need separate machinery. Phase 4's yielding and the prospect of recursive flow in Phase 6 confirm this. Phase 7 is the use of the system, not a new system.)*

---

## Skills: Teaching the Builder

*"So find a teacher who is an integral being, a beacon who extends his light and virtue with equal ease to those who appreciate him and those who don't."*
*-- Hua Hu Ching, Chapter 75*

The system teaches its builders through skills -- slash commands that embody the Tao's principles while guiding real development workflows. Each skill is a small master, carrying one aspect of the practice. They live in `.claude/commands/` and are invoked as `/skill-name`.

### The Nine Skills

| Skill | The Water | Purpose |
|-------|-----------|---------|
| `/riverbank` | The banks that hold the river | Run all quality checks: `fmt`, `clippy`, `test`. The CI gate, locally. |
| `/spring <name>` | A new source of water | Scaffold a new Spring (LLM adapter) with trait impl, config, and tests. |
| `/vessel <path>` | The empty pot | Scaffold a new module with structure, docs, tests, and module registration. |
| `/still-lake <target>` | Clarity through stillness | Code review through the five questions: clarity, wholeness, kindness, truth, simplicity. |
| `/rain <input>` | Water falling on the watershed | Test input flow -- observe how rain would travel through the system. |
| `/confluence [branch]` | Where streams merge | Review and integrate changes, identifying agreements, enrichments, and eddies. |
| `/flow <description>` | The complete journey | Full dev cycle: branch, implement, test, review, PR. Rain to ocean. |
| `/reflecting-pool` | The inward gaze | Examine the system itself: map vs. territory, unused weight, naming drift, accumulated excess. |
| `/rising-mist` | Lessons carried upward | Take what reflection revealed and reshape the forward vision. Update phase descriptions to carry new understanding. |

### How Skills Grow

Skills are not static. As the system evolves, new skills emerge naturally:

- When a pattern repeats three times, it may want to become a skill
- When a phase is complete, its scaffolding skill may retire or transform
- When builders struggle with a workflow, a skill can smooth the path

Skills follow the same principle as the system itself: they should do nothing unnecessary, yet leave nothing undone. A skill that tries too hard is worse than no skill at all.

### The Branch Discipline

All development flows through branches. Main is the ocean floor -- stable, settled, tested.

```
main (stable)
  |
  +-- feature/two-springs       (Phase 2)
  +-- feature/confluence-pool   (Phase 3)
  +-- feature/yielding          (Phase 4)
  +-- skills/new-skill-name     (skill additions)
  +-- fix/eddy-description      (bug fixes)
```

The `/flow` skill enforces this discipline: it creates branches, guides implementation, runs quality checks, and produces PRs. It never pushes directly to main.

---

## Error Handling: The Way of the Eddy

*"Failure is an opportunity."*
*-- Tao Te Ching, Chapter 79*

Errors are not failures -- they are eddies in the flow. A single `FlowError` type covers all phases:

- **SpringFailure** -- a spring failed to respond. The watershed continues; the remaining springs still flow.
- **Drought** -- no springs produced water. Only when ALL springs fail does the system stop.
- **ConfluenceFailure** -- the merge failed. The streams existed but could not be woven.
- **ClarityFailure** -- the Still Lake failed to polish. (Phase 5)
- **SourceError** -- the underlying LLM source (HTTP, CLI) failed.
- **ConfigError** -- configuration loading failed.

When a spring fails, the watershed filters it out and continues. A single dry spring does not dam the river. This resilience is natural to the architecture.

---

## Monitoring (Watching the Water)

Structured observability will arrive when `tracing` carries water. Tracing spans will follow the journey from Rain to Ocean -- as unobtrusive as watching a river from the bank.

---

## The Final Word

*"With all this talking, what has been said? The subtle truth can be pointed at with words, but it can't be contained by them."*
*-- Hua Hu Ching, Chapter 81*

This implementation document points at the system. It does not contain it. The true system will emerge through building -- through the act of coding, compiling, testing, yielding to the compiler's wisdom, and flowing again.

Rust is the metal vessel. The Tao is the water. The tests are the riverbanks. The compiler is the master.

Build like water. When the compiler rejects your code, do not fight it -- yield, and find the path it reveals. When a test fails, do not force it to pass -- understand what the failure teaches. When the system resists, step back and let the design emerge.

*"Do you have the patience to wait till your mud settles and the water is clear?"*

The code will teach you its shape, as the river teaches the riverbed.
