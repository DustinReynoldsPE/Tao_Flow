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
  window: mountain    (claude --model sonnet, persistent)
  window: desert      (claude --model haiku, persistent)
  window: forest      (claude --model sonnet, persistent)
  window: confluence  (claude --model haiku, utility)
  window: still-lake  (claude --model haiku, utility)
  window: decomposer  (claude --model haiku, utility)
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

```
src/
  main.rs                       # Entry point
  lib.rs                        # Library root -- re-exports
  flow.rs                       # TaoFlow -- Rain to Ocean (single-pass and recursive)
  error.rs                      # FlowError

  water/                        # The water types (compiler-enforced lifecycle)
    vapor.rs                    # Context carried between flows
    rain.rs                     # User input + volume + minerals
    stream.rs                   # Individual spring response (source + content)
    river.rs                    # Merged output from confluence
    ocean.rs                    # What the user receives

  watershed/                    # Where rain meets springs
    spring.rs                   # Spring trait + SpringConfig
    volume_sensor.rs            # Classifies rain volume by word count
    mineral_classifier.rs       # Keyword-based mineral extraction for spring affinity
    source/                     # The underground aquifer
      mod.rs                    # LlmSource trait + mock sources
      claude_cli.rs             # claude -p invocations (stateless)
      llama.rs                  # llama.cpp OpenAI-compatible API
      tmux_pane.rs              # Persistent vessel-backed source (TmuxPaneSource)
    springs/                    # Individual springs
      mountain.rs               # Deep reasoning
      desert.rs                 # Speed, directness
      forest.rs                 # Creativity, narrative, empathy

  confluence/                   # Where streams merge
    decomposition.rs            # Decomposer -- breaks Storm-level rain into sub-questions
    eddy.rs                     # Eddy, EddyNature, Position, Resolution types
    detection.rs                # EddyDetector -- finds divergence between streams
    yielding.rs                 # YieldingProtocol -- resolves eddies through yielding
    pool.rs                     # ConfluencePool -- detect, yield, weave

  vessel/                       # The observable space where springs flow
    tmux.rs                     # TmuxVessel -- persistent spring sessions in tmux windows
    wiring.rs                   # VesselConfig, build_tao_flow -- assembles the running system

  still_lake/                   # Final refinement (the five questions)
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
- **Stream** -- a single spring's response, carrying its source name and content.
- **Eddy** -- a point of divergence between streams. Has a topic, the positions taken, and a nature (Factual, Interpretive, Stylistic, Structural).
- **River** -- merged output from the confluence. Carries content, which tributaries contributed, any eddies detected, and a clarity score.
- **Ocean** -- what the user receives. Content only. By the time water reaches the ocean, it should be clear.

#### Clarity: The Signal from Confluence to Still Lake

In nature, a mountain spring produces clear water. When streams merge at confluence, sediment stirs -- clarity drops. A still lake lets sediment settle -- clarity returns.

River clarity carries this signal. A single stream passes through at 1.0 (crystal clear). A clean multi-stream merge is 0.8 (slightly muddied by weaving). Eddies will lower it further. The Still Lake (Phase 5) will read clarity to know how much polishing the water needs. Ocean has no clarity field -- by the time water arrives, the lake has settled it.

#### Minerals: Routing Water to the Right Springs

Springs declare affinities (what they resonate with). Rain carries minerals (tags). When minerals match affinities, a spring's relevance rises above the base score. `MineralClassifier` (keyword-based) populates minerals when rain enters the watershed. For Shower volume, the two most relevant springs are selected by affinity matching. For Downpour/Storm, all springs respond regardless.

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

When multiple streams arrive, the confluence detects eddies, resolves them through yielding, then weaves the result. It records which springs contributed as tributaries.

### The Still Lake

The lake reads `River::clarity` to know how much settling is needed. High clarity means gentle polish. Low clarity (many tributaries, unresolved eddies) means deeper settling. Three settling depths (Gentle, Moderate, Deep) respond proportionally. The lake engages only with unresolved eddies -- resolved ones already found truth through yielding. It asks the five questions -- clarity, wholeness, kindness, truth, simplicity -- and produces an Ocean. Wu wei: clear water (clarity 1.0) passes through untouched.

---

## The Main Flow

TaoFlow orchestrates the complete journey from rain to ocean.

```
flow(user_input):
    rain = Rain from user_input + vapor (context from previous flows)
    streams = watershed.receive_rain(rain)
    if no streams → Drought error
    river = confluence.merge(streams, rain_input)
    ocean = still_lake.settle(river, rain_input)
    update vapor with (user_input, ocean_content)
    return ocean content
```

The water cycle: after each flow, the user's input and the system's response are pushed into vapor as conversation history. The next rain carries this vapor, so springs see the full conversation. Context accumulates naturally.

The Still Lake reads river clarity to know how much polishing is needed. Graceful degradation: if settling fails, the river content reaches the ocean unchanged.

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

Creation is what happens when Storm volume triggers recursive flow -- not a separate module. Testing creation means testing the recursive path: decomposition produces independent sub-questions, sub-flows complete concurrently, higher confluence assembles, the still lake settles the whole. The existing Level 2 and future Level 3 tests cover this path.

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

**Phase 5: The Still Lake** *(complete)*
The final refinement pass. The lake receives the river -- its content, its clarity score, and its eddies. Clarity is a living signal: 1.0 for a single stream (crystal clear), 0.8 base for a multi-stream merge, reduced by eddies (resolved cost 0.05, unresolved cost 0.1, floor at 0.3). Three settling depths (Gentle, Moderate, Deep) respond proportionally to turbulence.

The lake engages only with unresolved eddies. Resolved eddies already found their truth through yielding. The lake asks the five questions (clarity, wholeness, kindness, truth, simplicity) and produces an Ocean. The single-pass flow is complete: Rain → Watershed → Confluence → Still Lake → Ocean.

Wu wei: clear water (clarity 1.0) passes through untouched. Graceful degradation: if settling fails, the river content reaches the ocean unchanged. The water always reaches the ocean.

**Phase 6: The Return** *(complete)*
*"Return is the movement of the Tao."* -- Chapter 40

The single-pass flow was complete and whole. Phase 6 deepened it. A Storm-level request cannot be answered in one pass. Phase 6 made the flow recursive:

1. **Decomposition** -- `Decomposer` breaks Storm-level rain into 2-5 independent sub-questions via LLM. Parses "Q:" prefix format with numbered-list fallback. Graceful degradation: if decomposition fails, single-pass handles the Storm.
2. **Recursive flow** -- each sub-question flows through the full single-pass journey independently and sequentially (shared vessel panes require it -- independence of thought is not the same as independence of vessel). Sub-questions carry the parent's vapor for context but do not update it.
3. **Higher confluence** -- sub-oceans become tributaries. The same ConfluencePool weaves them. The same StillLake settles the result. No new types were needed.
4. **Termination** -- `max_depth` prevents infinite recursion. Natural volume reduction provides the primary guard: sub-questions are shorter than storms, so they classify as Shower or Downpour and single-pass.

**Minerals found their water.** `MineralClassifier` (keyword-based) populates rain with mineral tags. The affinity system in `SpringConfig::sense_relevance()` now has a production caller: Shower-volume activation selects the two most relevant springs by mineral-affinity matching. Three phases as a dry riverbed, now flowing.

**Dead weight was dropped.** `Stream::clarity` (always 0.8, never read) and `Stream::depth` (calculated, never read) were removed. The `creation/` module dissolved into the recursive flow -- creation is what happens when Storm volume triggers the water cycle to cycle. The `config/` module was empty for six phases; inline configuration is enough.

**What was not built.** Yielding memory (springs remembering past yieldings across cycles), human pause points (boundaries between recursive cycles where the user can guide). These may emerge through use. The numbered phases end here -- the system grows organically from this point.

### What Grows Next

The numbered phases are complete. What follows is not Phase 7 -- it is organic growth, shaped by use. The reflections across six phases revealed a natural ordering: the vessel is the foundation, pearls record what flows through it, memories distill from pearls. Each layer depends on the one beneath.

**Yielding returns to the springs.** The current yielding protocol sends compressed position summaries to a neutral LLM that resolves each eddy. The springs never see each other's work. The 231710 storm pearl revealed what this costs: Mountain's sharpest critique ("Marcus Aurelius was also emperor -- he had more levers than most"), Forest's most vivid image ("a haunted house full of ghosts from states that no longer exist"), Mountain's most precise structural parallel (OpenTelemetry's span model as "a formal ontology of interdependent arising") -- all lost in compression before yielding even began. A mediator summarizing two positions cannot preserve what made each position distinctive.

The vessel changes this. Each spring is a persistent tmux pane. When eddies are detected, each spring already holds the context of its original response. Instead of compressing positions for a neutral third party, the system sends each spring the full responses from the siblings it diverges with. Mountain reads Forest's warmth. Desert reads Mountain's depth. Each spring responds to the other perspectives in its own voice -- not defending, but yielding while remaining itself.

The yielding becomes a dialogue:

```
detect eddies from streams
for each eddy:
    identify the springs whose positions diverge
    for each diverging spring (in its pane):
        send the full sibling responses that form the other side of the eddy
        receive the spring's response -- yielding in its own voice
    the yielding material is now the springs' own words, not a mediator's summary
weave from the original streams + the springs' yielding responses
```

This preserves what the current protocol erases: the distinctive voice, the specific evidence, the hard-won images. When Mountain yields to Forest's narrative, it yields as the mountain -- structurally, with precision. When Forest yields to Mountain's analysis, it yields as the forest -- with warmth and embodiment. The yielding carries the character of the yielder, not the flatness of a neutral arbiter.

It also enables non-resolution. A spring that reads its sibling's perspective and finds genuine, productive disagreement can say so. The current protocol resolves every eddy because the neutral LLM is instructed to find common ground. A spring speaking for itself can say: "I have read the mountain's analysis. I do not agree, and the disagreement is where the insight lives." Some eddies should not resolve. The tension between positions is sometimes the deepest truth.

**The vessel entered the water.** `vessel/wiring.rs` assembles the running system: `VesselConfig` holds model choices, `build_tao_flow()` creates the tmux session and wires three springs, confluence, still lake, and decomposer -- each in its own window. `main.rs` is a REPL that flows user input through the full watershed. `cargo run` starts the system; `tmux attach -t tao-flow` watches the water flow. The vessel is the observation layer that makes pearls possible, memories possible, and human guidance natural. Phase 3 anticipated human pause points (boundaries in recursive flows where the user can guide direction). Their shape will emerge from watching real flows in the vessel -- they cannot be designed in advance.

**Pearls preserve the journey.** Once the vessel flows, the chain of thought becomes capturable. Each flow produces a pearl -- the layered record from core (rain) to surface (ocean). See the dedicated section below. The vessel is the window; the pearl is the photograph. Pearls cannot form until the vessel carries water, because the vessel is what makes each layer visible.

**Memories distill from pearls.** After a storm passes and its pearl is formed, each component examines its layer and extracts the essence. See the dedicated section below. This subsumes yielding memory (Phase 4's insight) -- it is broader, encompassing every component, not just springs. Memories cannot distill until pearls exist, because pearls are the raw material.

**Level 3 end-to-end tests.** Real tmux, real LLM providers, the full journey from Rain to Ocean verified. These arrive naturally alongside the vessel -- when real springs flow in real panes, the e2e tests verify what the vessel makes visible.

---

## The Vessel: Observable Flow

*"We shape clay into a pot, but it is the emptiness inside that holds whatever we want."*
*-- Tao Te Ching, Chapter 11*

The `TmuxVessel` was built in Phase 2 and waited on shore through four phases. Six phases of reflections — each noting the boat on the bank — finally revealed why the vessel matters: it is not an optional feature for persistent sessions. It is the observation layer that makes everything else possible. Pearls cannot form without something to observe. Memories cannot distill without pearls. Human guidance cannot emerge without visibility into the flow.

When three springs respond to a question, the user sees only the ocean. They do not see the mountain's careful analysis, the desert's quick assessment, the forest's creative warmth. They do not see the eddies detected, the yielding that occurred, the settling that clarified. The water reaches the ocean, but the journey is hidden.

The vessel makes the journey visible. It is the foundation of the system's next growth.

### Architecture

```
tmux session: tao-flow
  ┌─────────────┬─────────────┬─────────────┐
  │  mountain   │   desert    │   forest    │
  │  (sonnet)   │   (haiku)   │   (sonnet)  │
  │             │             │             │
  │ [prompt]    │ [prompt]    │ [prompt]    │
  │ [response]  │ [response]  │ [response]  │
  ├─────────────┼─────────────┼─────────────┤
  │ confluence  │ still-lake  │ decomposer  │
  │  (haiku)    │  (haiku)    │  (haiku)    │
  └─────────────┴─────────────┴─────────────┘
```

Each spring gets a pane. The user watches the water flow in real time. `tmux attach -t tao-flow` to observe the system. Each pane shows the full conversation: the system prompt, the user's question arriving, the spring's thinking, the response. The system becomes transparent not by explaining itself, but by being visible.

### How it connects

`TmuxPaneSource` implements `LlmSource` -- the same trait that `ClaudeCliSource` and `LlamaSource` implement. Instead of spawning a one-shot `claude -p` process, it sends to a persistent tmux pane and captures the response. The spring does not know whether it is using a stateless CLI source or a persistent vessel-backed source. The architecture does not change. Only the underground aquifer deepens.

```
trait LlmSource: Send + Sync
    complete(system, messages) → Result<String>

ClaudeCliSource    -- stateless, spawns claude -p each time
LlamaSource        -- stateless, HTTP POST each time
TmuxPaneSource     -- persistent, sends to tmux window, waits for sentinel
SystemPromptSource -- wraps TmuxPaneSource, prepends system prompt to each message
                      (used by confluence, still lake, and decomposer whose
                       system prompts change between calls)
```

Stateless sources remain for testing and environments without tmux. The vessel is the natural home for real use -- when springs are wired through it, the system becomes observable.

### Session lifecycle

Each spring gets its own `TmuxVessel`, each vessel manages one tmux window:

```
TmuxVessel::new(session, window, model)
    .with_command(cmd)      -- the process this window runs
    .with_sentinel(pattern) -- the signal that means "ready"

prepare():
    create session/window if not running
    start the configured process
    (requires with_command -- the vessel must know its process)

send(input):
    send keys to window
    wait for sentinel in captured output
    extract response from full scrollback

teardown():
    kill session
```

`build_tao_flow()` in `vessel/wiring.rs` assembles everything: creates the tmux session, builds a vessel per component (three springs, confluence, still lake, decomposer), wraps each in a `TmuxPaneSource`, and wires them into `TaoFlow`. Because `TmuxPaneSource` implements `LlmSource`, the flow does not change -- the vessel connects through the trait, not through special plumbing.

---

## End-to-End Testing

The system has unit tests at every joint. What it lacks is end-to-end verification: does rain actually become ocean when real LLMs respond?

### Testing levels

**Level 1: Unit tests (existing).** MockSource replaces real LLMs. Fast, deterministic. Tests the flow structure, the type transitions, the graceful degradation. These are the bedrock and remain unchanged.

**Level 2: Vessel integration tests (existing).** Verify the tmux plumbing works without real LLMs. Echo processes in panes confirm:
- Session creation, pane splitting, and configurable processes
- Sending input and capturing output via sentinel detection
- Concurrent sends to multiple panes
- Session teardown and cleanup
- Full scrollback capture

These tests require tmux but not Claude or llama.cpp. They run in CI.

**Level 3: Full end-to-end tests.** Real tmux + real LLM providers. Mark as `#[ignore]` by default -- they are slow, require credentials, and cost money (or Claude Max subscription). Run manually or in a dedicated CI job.

Five tiers verify the full spectrum:
- **Tier 1 (Droplet):** desert spring alone produces a coherent ocean
- **Tier 2 (Shower):** two springs flow, confluence weaves, lake settles
- **Tier 3 (Downpour):** all three springs with real eddy detection and yielding
- **Tier 4 (Storm):** decomposer breaks input into sub-questions, sub-flows execute through full watershed, higher confluence assembles — verified by capturing the decomposer pane and asserting sub-questions were produced, spring panes received multiple exchanges, and confluence wove at least once
- **Tier 5 (Vapor):** multi-turn context carries across flows

Each test captures a flow journal (`target/e2e-journals/`) — the layered record of every window's exchanges. The journal is a proto-pearl: it preserves the journey, not just the destination. When pearls arrive, they will capture this same structure at the library level, including nested pearls for Storm sub-flows.

### Property-based tests (future)

Properties that should hold regardless of input:
- Water always reaches the ocean (no panics, no hangs)
- The ocean contains no system internals (prompt text, type names, debug info)
- Vapor grows with each flow
- Sub-flows do not pollute vapor

`proptest` can verify these across thousands of random inputs when the cost is justified.

---

## Pearls: The Chain of Thought Preserved

*"The Tao is like a well: used but never used up. It is like the eternal void: filled with infinite possibilities."*
*-- Tao Te Ching, Chapter 4*

When an irritant enters an oyster — a grain of sand, a fragment of shell — the oyster does not reject it. It coats it, layer by layer, with nacre. The result is a pearl: a self-contained, layered capsule built around a central disturbance. The disturbance is preserved at the core. Every layer of response surrounds it.

This is how the system preserves the chain of thought. Pearls depend on the vessel -- each layer is visible because each spring flows in its own pane. Without the vessel, the layers are hidden inside function calls. With it, they are observable, capturable, and whole.

### The pearl forms naturally

Every flow through the system — from rain to ocean — produces a pearl. The pearl is not a log. It is not a transcript. It is the layered record of how the system transformed a question into understanding.

```
.storms/
  2024-03-08-patience-and-persistence.pearl
  2024-03-08-what-is-the-tao.pearl
  2024-03-09-taoism-vs-stoicism.pearl
```

Each pearl contains every layer of the journey, from core to surface:

```
Pearl
  │
  ├── Core: the original query (rain)
  │     "How does water teach us about patience and persistence?"
  │
  ├── Layer 1: Spring responses (streams)
  │     Mountain: "Water does not try to be patient..."
  │     Desert:   "Water teaches patience through its nature..."
  │
  ├── Layer 2: Eddy detection (where perspectives diverged)
  │     EDDY|Interpretive|persistence-mechanism|...
  │     EDDY|Structural|narrative-arc|...
  │
  ├── Layer 3: Yielding (springs respond to each other)
  │     Mountain, reading Forest on persistence: "The image serves, but..."
  │     Forest, reading Mountain on persistence: "The structure is sound, and..."
  │     persistence-mechanism: resolved (or: held in tension)
  │     narrative-arc: resolved (or: held in tension)
  │
  ├── Layer 4: Merging (the river woven from streams)
  │     "Water does not try to be patient. That is its deepest teaching..."
  │
  ├── Layer 5: Settling (the lake clarifies)
  │     clarity: 0.65, depth: Deep
  │     "The response was refined for wholeness and simplicity..."
  │
  └── Surface: the ocean (what the user received)
        "Water does not try to be patient..."
```

The user opens a pearl and reads from core to surface — seeing exactly how their question was transformed at each stage. Or they read from surface to core — starting with the final answer and drilling into the reasoning that produced it.

### The relationship between pearls and memories

Pearls are the raw material. Memories are the minerals that remain. This is not a parallel relationship -- it is sequential. The pearl must form before the memory can distill, the way rain must fall before minerals can settle into earth.

After a storm passes and its pearl is formed, each component examines its layer and distills what it learned. The mountain reads its own response and extracts the essence. The confluence reads its eddies and remembers the pattern. The lake reads its settling and notes the turbulence.

```
vessel makes flow visible  →  pearl captures the layers  →  memories distill from pearls
(the observation)              (the preservation)            (the minerals)
```

`.storms/` stores pearls — the complete, layered records. `.memories/` stores minerals — the distilled essences that endure. The pearl preserves everything. Memory preserves only what matters.

Over time, pearls accumulate. Old pearls may be archived or pruned. But the memories they produced endure — fading slowly, reinforced by new storms that touch the same ground.

### What the pearl reveals

The pearl is transparency made permanent. The vessel makes the flow visible in real time — the user watches the springs respond, the eddies form, the river weave. The pearl preserves that visibility after the flow is done. The vessel is the window. The pearl is the photograph.

When the system produces an ocean that surprises or disappoints, the pearl tells the story: which spring saw what, where the eddies formed, how yielding resolved them, what the lake settled. The chain of thought is not hidden. It is layered, readable, and whole.

---

## Memory: The Minerals That Remain

*"Return is the movement of the Tao. Yielding is the way of the Tao."*
*-- Tao Te Ching, Chapter 40*

When water evaporates from the ocean and returns as rain, the water itself is gone. But the minerals it carried — dissolved from the mountain, filtered through the desert, enriched by the forest — settle into the earth. The earth remembers what flowed through it. Not the water. The minerals.

This is how the system remembers. Memory follows pearls as evaporation follows rain -- the pearl is the complete record, the memory is what remains after everything unnecessary has been dropped. This subsumes Phase 4's insight about yielding memory: each component's memories encompass what it learned from yielding, from settling, from every interaction -- not just springs, and not just yielding.

### Each component has its own earth

Every component in the flow — each spring, confluence, the lake, the ocean — maintains its own memories as individual files in `.memories/`. Each memory is a distilled essence: not what was said, but what was learned.

```
.memories/
  mountain/
    patience-as-presence.md
    canyon-carving-metaphor.md
  desert/
    tao-definition-pattern.md
  forest/
    (empty -- the forest has not yet flowed enough to remember)
  confluence/
    philosophical-eddies-resolve-through-synthesis.md
  lake/
    clear-water-needs-no-settling.md
  ocean/
    water-teaches-through-nature-not-instruction.md
```

The mountain remembers what the mountain cares about: deep analyses, philosophical threads, architectural patterns. The desert remembers quick patterns, common questions, efficient framings. The forest remembers stories told, emotional resonances, creative threads. Confluence remembers how eddies formed and how yielding resolved them — not the content, but the pattern. The lake remembers what turbulence looked like and what settling required. The ocean remembers what it delivered — the final form.

Each memory is partial. No single component holds the whole picture. Wholeness emerges at confluence, where partial memories from different springs meet.

### The lifecycle of a memory

A memory file carries:

```
essence:    what was learned, distilled to its core
strength:   0.0 to 1.0 -- how vivid the memory is
minerals:   topic tags for relevance matching
last_seen:  when this memory was last reinforced
```

**Birth.** After a flow completes, each component distills what it experienced into a brief essence. The mountain does not record the full conversation about patience. It records: "Patience is not enduring difficulty across time. It is being so fully present that time loses its weight." A new memory begins with moderate strength.

**Carry.** Before the next flow, the system loads each component's memories. The strongest, most relevant memories are added to the component's context — the mountain receives its memories of depth, the desert its memories of speed. The component does not know it is remembering. The memories arrive as naturally as the system prompt.

**Reinforcement.** When a memory is relevant to the current flow — when its minerals match the rain's minerals — it grows stronger. The mountain's memory of patience, reinforced by a second question about patience, deepens. The riverbed is carved deeper by repeated flow.

**Decay.** Every memory fades. Time without reinforcement erodes strength. A memory of a conversation three months ago is fainter than one from yesterday. This is not failure — it is Chapter 48: *every day something is dropped.*

**Death.** When a memory's strength falls below a threshold, it dissolves. The file is removed. The earth forgets what no longer matters. The spring returns to its nature, unburdened.

### Why forgetting matters

If springs remembered everything, they would converge. The mountain would echo what the desert said. The forest would repeat the mountain's analysis. The productive tension between springs — the tension that creates eddies, that drives yielding, that produces synthesis — would vanish.

Decay preserves each spring's nature. The mountain forgets the desert's quick answers because quick answers are not the mountain's earth. The desert forgets the mountain's depth because depth does not settle in sand. Each component's memories are shaped by what naturally adheres to its nature.

Over-integration is the death of the system. Three springs that remember the same things are one spring pretending to be three. Forgetting is what keeps the water alive.

### How memories reach the flow

Before each flow, relevant memories are gathered:

```
for each component:
    load memories from .memories/{component}/
    apply time decay to each memory's strength
    filter by relevance: match memory minerals against rain minerals
    select the strongest memories (bounded -- not all, not none)
    weave selected essences into the component's context

after each flow:
    each component distills what it learned
    if a memory was reinforced, increase its strength
    if a new insight emerged, birth a new memory
    if a memory decayed below threshold, remove it
    write updated memories back to .memories/
```

The mineral system already exists. Rain carries minerals (philosophy, architecture, code, narrative). Memories carry minerals. The matching is natural — the infrastructure was built in Phase 6 for spring activation. Now it serves memory retrieval.

### What the distillation is not

The distillation is not a summary. It is not a transcript. It is not conversation history replayed. It is the mineral deposit left after the water evaporated.

A summary says: "The user asked about patience. The mountain responded with an analysis of water's relationship to time." That is a record.

An essence says: "Patience is presence, not endurance. The canyon is carved by continuity, not force." That is a memory.

The difference matters. Summaries grow without bound. Essences are naturally small — they are what remains after everything unnecessary has been dropped. They are Chapter 48 in practice.

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
- **SettlingFailure** -- the Still Lake failed to settle. Graceful degradation: the river content reaches the ocean unchanged.
- **DecompositionFailure** -- Storm-level decomposition failed. Graceful degradation: single-pass handles the Storm.
- **VesselFailure** -- the tmux vessel could not prepare, send, or tear down.

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
