# Tao Flow

A multi-LLM system that flows like water, forged in Rust.

```
                                Rain
                            (user input)
                                 │
                                 ▼
                    ┌────────────────────────┐
                    │       Watershed        │
                    │                        │
                    │  volume: how complex?  │
                    │  minerals: what kind?  │
                    │                        │
                    │   ┌──────┬──────┐      │
                    │   │  Mt  │ Dsrt │ Frst │
                    │   │depth │speed │warmth│
                    │   └──┬───┴──┬───┴──┬───┘
                    │      │      │      │    │
                    └──────┼──────┼──────┼────┘
                           │      │      │
                        streams (each spring's voice)
                           │      │      │
                    ┌──────▼──────▼──────▼────┐
                    │      Confluence         │
                    │                         │
                    │  detect eddies          │
                    │  yield (resolve or hold)│
                    │  weave into one river   │
                    └────────────┬────────────┘
                                 │
                              river
                     (clarity, tributaries, eddies)
                                 │
                    ┌────────────▼────────────┐
                    │       Still Lake        │
                    │                         │
                    │  six questions:         │
                    │  clarity, wholeness,    │
                    │  kindness, truth,       │
                    │  simplicity, fidelity   │
                    └────────────┬────────────┘
                                 │
                               Ocean
                          (what the user sees)
```

Three LLMs respond to every question — not competing, but each offering what its nature provides. Mountain thinks deeply. Desert moves quickly. Forest tells stories. Their streams merge at the confluence, where divergences (eddies) are detected and resolved through yielding — not voting, not picking a winner, but letting each perspective find truth in the others.

The volume of rain determines how many springs respond. A greeting (Droplet) needs only the desert. A moderate question (Shower) activates the two most relevant springs. A complex request (Downpour) or a transformative one (Storm) activates all three. Storm-level input decomposes into sub-questions, each flowing through the full system independently before reassembling at a higher confluence.

The Still Lake is the final refinement. It asks six questions of the river before releasing it as ocean: Is it clear? Is it whole? Is it kind? Is it true? Is it simple? Is it faithful to the strongest content from the springs?

```
                    ┌─────────────────────────────────────┐
                    │         tmux session: tao-flow      │
                    │                                     │
                    │  mountain │  desert  │   forest     │
                    │  (sonnet) │ (sonnet) │  (sonnet)    │
                    │           │          │              │
                    │  confluence│ still-lake│ decomposer  │
                    │  (sonnet) │ (sonnet) │  (sonnet)    │
                    └─────────────────────────────────────┘
                                    │
                              the vessel
                       (tmux attach -t tao-flow)
```

Every spring runs in a persistent tmux pane. The vessel makes the entire journey visible — watch the mountain think, the desert cut to the point, the forest weave its narrative, the confluence detect where they diverge, the lake settle the result. The system is transparent not by explaining itself, but by being observable.

Each flow produces a pearl — a layered record from core (the question) to surface (the answer), with every intermediate layer preserved. Pearls are not logs. They are the chain of thought made permanent: which springs responded, where they diverged, how yielding resolved or held the tension, what the lake settled. Storm flows nest sub-pearls inside, one per sub-question.

```
.storms/
  patience-and-persistence-20240308-143022/
    core.md           the question
    streams/
      mountain.md     depth
      desert.md       speed
      forest.md       warmth
    river.md          woven, with eddy metadata
    ocean.md          what the user received
    pearl.json        full structured record
```

The system is built on five Taoist principles: wu wei (the orchestrator creates conditions, not commands), the way of water (input flows naturally to where it belongs), yielding (disagreement resolves through mutual recognition, not voting), emptiness (the architecture lives in the channels between LLMs, not in the LLMs themselves), and the integral way (wholeness emerges from partial perspectives, never from a single source).

Built in Rust — the metal vessel that holds the water without yielding, so the water may yield freely.
