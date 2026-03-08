# End-to-End Testing Plan

*"Do you have the patience to wait till your mud settles and the water is clear?"*
*-- Tao Te Ching, Chapter 15*

---

## What E2E Tests Reveal

Unit tests (Level 1) verify the plumbing with mock water. Vessel integration tests (Level 2) verify the vessel carries echo-water. Neither proves the system works with real water.

Level 3 tests send real rain through real springs in real vessels and verify the ocean arrives clear. They test what no mock can: that real LLMs produce responses the confluence can weave, that real eddies emerge and yield, that the still lake settles real turbulence, and that the vessel makes the journey visible.

---

## Prerequisites

Three things must happen before Level 3 tests can run.

### 1. Discover Claude's Interactive Sentinel

TmuxPaneSource uses sentinel detection to know when a response is complete. The sentinel is a pattern that appears when the process is ready for new input.

**Discovery method:**

```
start a tmux session
run `claude` interactively in a pane
send a question
capture the pane output after the response
identify the consistent pattern that appears when claude is ready
```

This is empirical work -- done once, by hand, watching the pane.

**Fallback if no clean sentinel exists:**

Use a wrapper script that invokes `claude -p` (stateless) and echoes a known sentinel after each response:

```
while read input; do
    echo "$input" | claude -p --model <model> --system-prompt "<prompt>"
    echo "SENTINEL_READY"
done
```

This sacrifices conversation memory (each call is stateless) but provides a reliable sentinel. The vessel still carries observability -- the pane shows every exchange. True persistent conversation via interactive mode is preferable but not required for initial e2e validation.

### 2. Wire Springs Through the Vessel

Create a configuration that connects springs to TmuxPaneSource instances:

```
TaoFlow configured with:
    Mountain → TmuxPaneSource (vessel: "tao-flow", pane: "mountain", model: opus)
    Desert   → TmuxPaneSource (vessel: "tao-flow", pane: "desert",   model: haiku)
    Forest   → TmuxPaneSource (vessel: "tao-flow", pane: "forest",  model: sonnet)
    Confluence → ClaudeCliSource or TmuxPaneSource (weaving)
    StillLake  → ClaudeCliSource or TmuxPaneSource (settling)
    Decomposer → ClaudeCliSource (stateless, one-shot)
```

Springs use persistent vessel panes. The confluence, lake, and decomposer may use either -- persistent panes for observability, or stateless CLI for simplicity. The wiring will teach what works.

### 3. Session Management for Tests

Each e2e test:

1. Checks tmux is available (skip if not)
2. Creates a unique session (e.g., `tao-e2e-{test_name}`)
3. Prepares vessels for each spring
4. Runs the flow
5. Captures results and optionally captures pane content
6. Tears down the session (even on failure)

The existing vessel integration test infrastructure (`cleanup()`, `tmux_available()`, `session_exists()`) provides the pattern.

---

## Test Structure

E2e tests live in `tests/e2e_flow.rs`. All marked `#[ignore]` -- they are slow, require real LLM access, and never run in standard CI.

```
cargo test --test e2e_flow -- --ignored
```

Individual tests by name:

```
cargo test --test e2e_flow -- --ignored droplet_through_vessel
```

The quality gate (`/riverbank`) runs Level 1-2 only. E2e tests run manually or via dedicated CI trigger.

---

## Test Scenarios

### Tier 1: Single Spring (The Droplet)

**`droplet_through_vessel`**
- Input: a simple question ("What is the Tao?")
- Expected: desert spring responds through vessel, ocean has substance
- Verify:
  - Ocean content is non-empty
  - Ocean content does not contain system prompt text
  - Only one spring activated (desert)

**`droplet_with_mineral_routing`**
- Input: a philosophical question ("What is the nature of consciousness?")
- Expected: mountain spring responds (philosophy mineral matches mountain affinity)
- Verify:
  - Mountain activated instead of desert
  - Ocean content addresses the question

### Tier 2: Two Springs (The Shower)

**`shower_weaves_two_perspectives`**
- Input: a question touching two domains (~15 words)
- Expected: two springs respond, confluence weaves into one voice
- Verify:
  - River has exactly 2 tributaries
  - River clarity is 0.8 or lower (multi-stream merge)
  - Ocean reads as one voice, not a list of perspectives
  - Both active panes show conversation

**`shower_mineral_selects_best_two`**
- Input: a creative-philosophical question
- Expected: mountain and forest selected by affinity, not desert
- Verify:
  - Tributaries are mountain and forest
  - Affinity routing works in production, not just in unit tests

### Tier 3: Three Springs (The Downpour)

**`downpour_with_real_eddies`**
- Input: an opinion-laden question where springs will genuinely disagree (~40 words)
- Expected: all three springs respond, eddies detected, yielding occurs
- Verify:
  - River has 3 tributaries
  - At least one eddy detected (real disagreement between models)
  - Eddies have resolutions (yielding produced synthesis)
  - River clarity < 0.8 (eddies reduced it)
  - Ocean is coherent despite the disagreement
  - Still lake settled the result (settling depth matches clarity)

**`downpour_agreement_on_factual`**
- Input: a factual question where springs should agree (~40 words)
- Expected: three springs respond with similar content, few or no eddies
- Verify:
  - River clarity >= 0.75 (agreement)
  - Lake settling is gentle or pass-through

### Tier 4: Recursive Flow (The Storm)

**`storm_decomposes_and_reassembles`**
- Input: a complex, multi-part question (>100 words)
- Expected: decomposition into 2-5 sub-questions, each flowing independently, higher confluence assembles
- Verify:
  - Decomposition produced 2-5 sub-questions
  - Each sub-question flowed (no drought on any sub-flow)
  - Higher confluence wove sub-results into a coherent whole
  - Final ocean addresses the full original question
  - Vapor records only the top-level exchange, not sub-flow artifacts

### Tier 5: Multi-Turn (The Water Cycle)

**`vapor_carries_context_across_real_flows`**
- Flow 1: ask a question ("Describe the philosophy of wu wei.")
- Flow 2: ask a follow-up requiring context ("How does this apply to software?")
- Verify:
  - Flow 2's ocean references or builds on Flow 1's content
  - Vapor contains both exchanges after Flow 2
  - Springs received the conversation history (visible in pane content)

### Tier 6: Observability (The Vessel's Purpose)

**`vessel_panes_show_full_journey`**
- Run a Shower-level flow
- After ocean is produced, capture each pane's content via `capture-pane`
- Verify:
  - Each active pane contains the question that was asked
  - Each active pane contains a response
  - The pane content is human-readable

This tier tests the vessel's reason for existence: making the journey visible.

---

## Verification Strategy

Real LLM output is non-deterministic. Tests cannot assert exact strings.

### Structural (Automated)

These run as normal `assert!` checks:

- Ocean has substance (non-empty, non-whitespace)
- Ocean does not contain system internals (system prompt fragments, spring names used in prompts, type names, debug artifacts)
- River tributaries match expected spring count
- River clarity is within expected range for the scenario
- Eddies have valid structure (topic, nature, positions all present)
- Vapor length grows across flows
- Sub-flows do not pollute vapor

### Visual (Manual, First Runs)

The first run of each tier should be watched:

- `tmux attach -t tao-e2e-*` to watch the flow in real time
- Verify panes show conversation naturally
- Verify ocean content is coherent and relevant
- Verify eddy topics relate to actual disagreements

Once patterns are established, structural checks carry the ongoing verification. The vessel itself is the best debugging tool -- when a test fails, attach to the session before teardown and look.

---

## Operational Concerns

### Timeouts

Real LLM calls are slow. Springs respond in 2-30 seconds depending on model and load. A full Downpour flow makes 6+ LLM calls (3 springs + detection + yielding + weaving + settling). A Storm with recursive sub-flows could make 20+.

Wrap each test's flow in `tokio::time::timeout`. Generous limits:
- Tier 1 (Droplet): 60 seconds
- Tier 2 (Shower): 120 seconds
- Tier 3 (Downpour): 180 seconds
- Tier 4 (Storm): 300 seconds
- Tier 5 (Multi-turn): 180 seconds
- Tier 6 (Observability): 120 seconds

### Cost

For Claude Max users, the cost is zero -- subscription covers all CLI usage. For API-based sources (LlamaSource against a paid endpoint), each test costs real money. Run API-based e2e tests sparingly.

### Isolation

Each test uses its own tmux session name. Tests must not share sessions. Teardown must run even on failure -- implement cleanup in a guard struct with `Drop`, or use explicit cleanup at test end.

### Lazy Initialization

TmuxPaneSource prepares the vessel on first `complete()` call. The first call per spring is slower (process startup + potential model loading). Tests should not interpret startup latency as failure.

### CI

E2e tests do not run in standard CI. They require:
- tmux (available in most CI environments)
- `claude` CLI with valid authentication (not available in public CI)
- Time (minutes per test, not milliseconds)

A dedicated CI job with manual trigger, or local-only execution. The pre-commit quality gate runs Level 1-2 only.

---

## Order of Implementation

1. **Discover the sentinel** -- empirical, done once by hand
2. **Build wiring helpers** -- functions that create TaoFlow with TmuxPaneSource-backed springs
3. **Tier 1 tests** -- simplest e2e, single spring, single flow
4. **Tier 6 tests** -- observability, the vessel's reason for existence
5. **Tier 2-3 tests** -- multi-spring flows, confluence and yielding with real water
6. **Tier 5 tests** -- multi-turn, vapor carries real context
7. **Tier 4 tests** -- recursive flow, most complex, most expensive

Start simple. Let each tier teach what the next needs.

---

## What This Plan Does Not Cover

**Semantic evaluation.** Judging whether the ocean's content is *good* -- relevant, accurate, well-written -- requires human judgment or an LLM evaluator. This plan verifies the water reaches the ocean and the ocean has substance. It does not judge the ocean's depth. That judgment belongs to the user, watching through the vessel.

**Performance benchmarks.** How fast is the flow? How much memory does it use? These questions matter but belong to a separate concern. The first priority is: does real water flow through real channels?

**Prompt engineering.** The system prompts that shape each spring are the riverbed. E2e tests may reveal that a spring's prompt produces poor results, or that the confluence prompt fails to weave coherently. When this happens, the fix is in the prompt, not in the test. The test reveals; the prompt evolves.
