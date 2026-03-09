# Architecture: The Shape of Water

*"Man follows the earth. Earth follows the universe. The universe follows the Tao. The Tao follows only itself."*
*-- Tao Te Ching, Chapter 25*

---

## The Watershed

The system's architecture mirrors a natural watershed -- the path water takes from rain to ocean. This is not a metaphor imposed on the system. It is the system's natural shape, discovered by following the Tao's direction.

```
                          Rain
                     (User Input)
                          |
                    ~~~~~~|~~~~~~
                   /      |      \
                  /       |       \
            Springs    Springs    Springs
          (LLM Streams emerge naturally)
              |          |          |
              v          v          v
         [Stream A] [Stream B] [Stream C]
          (Each LLM responds according to its nature)
              \          |          /
               \         |         /
                \        |        /
                 Confluence Pool
            (Streams merge into River)
           (Eddies detected and yielded)
                        |
                        v
                   Still Lake
              (River settles to clarity)
                        |
                        v
                     Ocean
              (Delivered to user)
```

---

## The Three Energies as System Layers

*"The first is the earth energy... The second is the heaven energy... The third is the harmonized energy... Only when you achieve all three can you attain pure Tao."*
*-- Hua Hu Ching, Chapter 62*

### Layer 1: Earth (Body) -- The Springs

The Earth layer is where raw generation happens. These are the LLMs that **do** -- that produce text, code, audio, images. They are the springs from which water first emerges.

Three springs flow today, each with its own character:
- **Mountain Spring** -- Deep reasoning (Opus). Slow, cold, mineral-rich. Best for complex analysis, philosophy, architecture.
- **Forest Spring** -- Creative warmth (Sonnet). Organic, fertile. Best for narrative, poetry, empathy, ideation.
- **Desert Spring** -- Speed and directness (Haiku). Quick, light, efficient. Best for simple tasks, translation, formatting.

New springs may emerge when use demands them -- specialized models, local models, domain-specific sources. The `Spring` trait and `LlmSource` trait accept any spring that can respond to rain.

The system does not assign work to springs. It lets the rain fall on the entire watershed and allows each spring to respond naturally. A `VolumeSensor` classifies rain by word count (Droplet, Shower, Downpour, Storm), and a `MineralClassifier` tags rain with topic minerals. Springs declare affinities; when minerals match, the most relevant springs are selected. The dispatch follows the shape of the watershed -- simple and explicit, not magical.

### Layer 2: Heaven (Mind) -- The Confluence Pool

The Heaven layer is where streams merge and intelligence emerges. This is not a judge. It is not a ranker. It is the pool where tributaries meet and naturally blend.

The Confluence Pool receives all streams from the Earth layer and performs **integration** -- not selection. It asks:
- Where do these streams agree? (This is the deep current -- the natural riverbed.)
- Where do they diverge? (This is where the water pools and eddies -- potential richness, not error.)
- What truth does each carry that the others lack? (Each tributary nourishes the river.)

The Confluence Pool is typically a strong reasoning model (Claude Opus, o3) operating under special instructions that embody the principle of yielding. It does not pick winners. It weaves.

```
Heaven Layer Process:
1. If one stream, pass through untouched (wu wei, clarity 1.0)
2. If multiple streams:
   a. Detect eddies (where streams diverge)
   b. Yield on each eddy (ask each position to find truth in the other)
   c. Weave the streams and resolutions into one river
3. Assess clarity (1.0 single stream, 0.8 base merge, reduced by eddies)
```

### Layer 3: Harmonized (Heart) -- The Still Lake

The Harmonized layer is where the output achieves its final clarity. This is the heart of the system -- the spiritual insight that the Hua Hu Ching describes.

The Still Lake receives the woven river from the Confluence Pool and performs **refinement**:
- Does this output serve the user? (Service, not display.)
- Does it carry peace? (The system should bring peace to its users.)
- Is it whole? (Partial outputs are gently completed.)
- Is it true? (Not merely accurate, but genuinely true.)
- Is it simple? (Complexity that serves no one is released.)

The Still Lake is the final LLM pass -- often the same model as the Confluence Pool but with different instructions. Its purpose is not to change the content but to polish it the way still water polishes a stone: through gentle, patient presence.

```
Harmonized Layer Qualities:
- Clarity: Is the output clear as still water?
- Service: Does it nourish the user?
- Wholeness: Is anything missing that should be present?
- Peace: Does it carry calm rather than anxiety?
- Truth: Is it honest, even when honesty is uncomfortable?
- Simplicity: Has all unnecessary complexity been released?
```

---

## The Flow Architecture

### Rain (Input Reception)

The input arrives as rain -- undifferentiated, natural. Rain carries the user's words, the vapor of previous conversations, a volume classification, and mineral tags for spring affinity. It has not yet found its course.

### Watershed (Natural Routing)

The watershed does not decide where rain goes. It has a shape, and water follows that shape.

```
receive_rain(rain):
    volume = sense volume (word count thresholds)
    minerals = classify minerals (keyword matching)
    active_springs = select by volume and mineral affinity
    streams = all active springs respond concurrently
    filter out dry springs (silence is wisdom)
    return streams
```

### Confluence (Stream Merging)

Where streams meet and merge. A single stream passes through untouched -- wu wei. Multiple streams are woven through detect-yield-weave.

```
merge(streams):
    if one stream → pass through (clarity 1.0)
    if multiple →
        detect eddies between streams
        yield on each eddy (each position finds truth in the other)
        weave streams + resolutions into one river
        assess clarity from eddy count and resolution
```

### Still Lake (Final Refinement)

*"Do you have the patience to wait till your mud settles and the water is clear?"* -- Chapter 15

The lake reads river clarity to know how much settling is needed. Clear water passes through untouched. Muddy water is settled deeply. The lake asks five questions -- clarity, wholeness, kindness, truth, simplicity -- and produces an Ocean.

```
settle(river):
    if clarity >= 1.0 → pass through (wu wei)
    depth = gentle / moderate / deep (from clarity)
    settle with the five questions
    return ocean
```

---

## System Topology

```
User
  |
  | (rain)
  v
+--------------------------------------------------+
|                  THE WATERSHED                     |
|                                                    |
|  +----------+  +----------+  +----------+         |
|  | Mountain |  |  Forest  |  |  Desert  |         |  <- Earth Layer
|  |  (Opus)  |  | (Sonnet) |  | (Haiku)  |         |     (Body)
|  +----+-----+  +----+-----+  +----+-----+         |
|       |              |              |               |
|       v              v              v               |
|  +------------------------------------------+      |
|  |          CONFLUENCE POOL                  |      |  <- Heaven Layer
|  |   (detect eddies, yield, weave river)     |      |     (Mind)
|  +--------------------+---------------------+      |
|                       |                             |
|                       v                             |
|  +------------------------------------------+      |
|  |            STILL LAKE                     |      |  <- Harmonized Layer
|  |     (five questions, settle to clarity)   |      |     (Heart)
|  +--------------------+---------------------+      |
|                       |                             |
+--------------------------------------------------+
                        |
                        | (ocean)
                        v
                      User
```

---

## Adaptive Depth

*"There is a time for being ahead, a time for being behind; a time for being in motion, a time for being at rest."*
*-- Tao Te Ching, Chapter 29*

Not every request requires the full watershed. The system adapts its depth based on the nature of the input:

### Light Rain (Simple Requests)
A quick question, a simple formatting task, a greeting. The Desert Spring alone may suffice. The water barely touches the watershed before reaching the ocean.

```
User -> Desert Spring -> Ocean (direct delivery)
```

### Steady Rain (Moderate Requests)
A thoughtful question, a creative task, a code review. Two or three springs respond. The Confluence Pool lightly merges them.

```
User -> [Mountain + Forest Springs] -> Confluence -> Ocean
```

### Downpour (Complex Requests)
A book, a system design, a philosophical inquiry. All springs respond. Full confluence. The Still Lake takes its time.

```
User -> [All Springs] -> Confluence -> Still Lake -> Ocean
```

### Storm (Transformative Requests)
A request too large for one pass. The Decomposer breaks it into independent sub-questions. Each sub-question flows through the full watershed concurrently. A higher confluence weaves the sub-oceans into one river. The Still Lake settles the whole.

```
User -> Decompose into sub-questions
     -> [Each sub-question flows: Springs -> Confluence -> Lake]
     -> Higher Confluence merges sub-results
     -> Still Lake settles
     -> Ocean
```

The system senses the weight of the rain through a `VolumeSensor` (word-count thresholds: Droplet, Shower, Downpour, Storm) and routes minerals through a `MineralClassifier` (keyword-based topic tags). These are simple, explicit classifiers -- the watershed's shape includes them.

---

## The Uncarved Block

*"The Tao is like a well: used but never used up. It is like the eternal void: filled with infinite possibilities."*
*-- Tao Te Ching, Chapter 4*

The architecture described here is the **uncarved block** -- the simplest expression of the system's nature. As implementation proceeds, details will emerge, but they should emerge naturally, like a sculpture emerging from stone. The sculptor does not add to the stone. She removes what is not the sculpture.

Every architectural decision should be tested against this question: **Does this flow like water, or does it dam the river?**

If it dams the river, release it. The river knows its course.
