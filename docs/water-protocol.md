# The Water Protocol

*"In dwelling, live close to the ground. In thinking, keep to the simple. In conflict, be fair and generous. In governing, don't try to control. In work, do what you enjoy. In family life, be completely present."*
*-- Tao Te Ching, Chapter 8*

---

## Overview

The Water Protocol defines how input enters, flows through, and exits the system. It is the Tao of the system -- the invisible channel through which everything moves. Like the Tao itself, the protocol should be felt rather than seen. The user should never be aware of the protocol, only of the nourishing output that reaches them.

*"When the Master governs, the people are hardly aware that he exists."*
*-- Tao Te Ching, Chapter 17*

---

## The Five States of Water

Water exists in different states as it moves through the system. Each state corresponds to a phase of processing.

### 1. Vapor -- Intention

Before the user types, there is intention. Vapor is the invisible precursor to rain. In the system, vapor represents **context** -- the conversation history, user preferences, session state, and ambient conditions that exist before any new input arrives.

```
Vapor = {
  conversation_history: [...],   // The clouds that have already formed
  user_preferences: {...},       // The atmospheric conditions
  session_context: {...},        // The weather pattern
  emotional_temperature: float,  // The warmth that drives evaporation
}
```

The system maintains vapor continuously. It does not wait for input to begin understanding the user. Like the atmosphere, it is always present, always subtly shifting.

### 2. Rain -- Input

The user's input falls like rain upon the watershed. Rain has several qualities:

- **Volume** -- How much is being asked? A word? A paragraph? A document?
- **Temperature** -- What is the emotional quality? Urgent? Curious? Playful? Distressed?
- **Mineral Content** -- What domain knowledge does it carry? Technical terms? Artistic references? Emotional language?
- **Patterns** -- Is it steady rain (a clear request)? Scattered showers (an ambiguous exploration)? A thunderstorm (an urgent crisis)?

```
Rain = {
  raw_input: string,             // The water itself
  volume: "droplet" | "shower" | "downpour" | "storm",
  temperature: float,            // -1.0 (ice cold, analytical) to 1.0 (warm, emotional)
  minerals: [string],            // Detected domains and themes
  pattern: "steady" | "scattered" | "storm",
}
```

**Crucially:** Rain analysis is gentle, not forceful. The system does not parse and categorize with rigid NLP pipelines. It *senses* the rain the way the earth senses water falling on it -- through contact, through absorption, through the natural response of the terrain.

In practice, this means a lightweight LLM pass (the Desert Spring) that produces the Rain qualities as a soft assessment, not a hard classification. The assessment can be wrong. Water finds its way regardless.

### 3. Streams -- LLM Responses

Each spring in the watershed produces a stream. A stream is an LLM's natural response to the rain.

```
Stream = {
  source: string,               // Which spring produced this
  water: string,                // The response content
  flow_rate: float,             // How quickly it was produced
  clarity: float,               // The model's confidence (honestly assessed)
  depth: float,                 // How deeply the response engages
  temperature: float,           // Emotional warmth of the response
  minerals: [string],           // Domains and themes present
  volume: int,                  // Length of response
}
```

Streams are the Earth layer's output. Each carries the character of its spring:
- Mountain streams are cold, clear, deep, and slow
- Forest streams are warm, rich, moderate, and organic
- Desert streams are light, quick, efficient, and sparse
- Underground streams are specialized, precise, and domain-specific

### 4. River -- Merged Output

When streams confluent, they become a river. The river carries elements from all tributaries but has its own unified character.

```
River = {
  water: string,                // The merged content
  tributaries: [string],        // Which streams contributed
  depth: float,                 // Overall depth
  clarity: float,               // How clear the merged output is
  coherence: float,             // How well the streams merged
  eddies: [Eddy],               // Remaining areas of divergence
}
```

The river may still carry eddies -- places where streams didn't fully merge, where perspectives still swirl. These are not errors. They are natural features of rivers. Some eddies are left as-is (healthy diversity of perspective). Others are settled in the Still Lake.

### 5. Ocean -- Delivered Output

The ocean is what the user receives. It is vast, deep, and unified. The user does not see the rain, the streams, or the river. They see only the ocean.

```
Ocean = {
  content: string,              // What the user receives
  depth: float,                 // How profound the response is
  warmth: float,                // Emotional quality
  nourishment: float,           // How well it serves the user's need
}
```

---

## Flow Dynamics

### Natural Routing (The Riverbed)

*"The Tao is like a bellows: it is empty yet infinitely capable."*
*-- Tao Te Ching, Chapter 5*

The system does not use explicit routing logic. Instead, each spring has **affinity** -- a natural tendency to respond strongly to certain types of rain and weakly to others.

```
Affinity Model:
  Mountain Spring:
    high_affinity: [philosophy, architecture, analysis, strategy, ethics]
    low_affinity: [formatting, translation, simple_math]

  Forest Spring:
    high_affinity: [narrative, poetry, empathy, brainstorming, dialogue]
    low_affinity: [debugging, data_analysis, formal_logic]

  Desert Spring:
    high_affinity: [quick_answers, formatting, translation, classification]
    low_affinity: [deep_analysis, creative_writing, complex_reasoning]

  Underground Spring:
    high_affinity: [code, audio, images, domain_specific]
    low_affinity: [general_conversation, philosophy, emotional_support]
```

When rain falls, all springs receive it. But affinity determines **how much water each spring produces**. A coding question will produce a torrent from the Underground Spring and a trickle from the Forest Spring. A poem request reverses this. The routing is in the response, not in the dispatch.

**Implementation:** Each spring is called with the same input. The spring's system prompt naturally shapes its response. Springs with low affinity for the input may produce a brief or empty response, which is naturally filtered out (a dry spring in summer). This is wu wei -- the routing happens without routing logic.

### Parallel Streams (Wu Wei in Action)

All springs flow simultaneously. There is no sequential pipeline where one LLM waits for another. Water does not wait.

```
async function watershed(rain: Rain): Promise<Stream[]> {
  // All springs receive rain at the same moment
  // Each responds according to its nature
  // Wu wei: we do not tell them what to do
  const streams = await Promise.all(
    springs.map(spring => spring.respond(rain))
  );

  // Filter dry springs (those with nothing to contribute)
  return streams.filter(stream => stream.has_water());
}
```

### Stream Merging (The Confluence)

Streams merge at the Confluence Pool. The merging process follows these natural laws:

**Law of Agreement:** Where all streams say the same thing, that is the deep riverbed -- the undeniable current. It passes through unchanged.

**Law of Enrichment:** Where one stream carries minerals (knowledge, perspective, detail) that others lack, it enriches the river. Nothing is discarded simply because only one stream carried it.

**Law of Contradiction:** Where streams directly contradict, an eddy forms. The eddy is not forced to resolve immediately. It is held gently (see [Conflict Resolution](conflict-resolution.md)).

**Law of Volume:** Streams with more water (more confident, more detailed, more relevant responses) naturally have more influence on the river's course. But even small streams shape the riverbed.

```
function confluence(streams: Stream[]): River {
  // Find the deep current
  const agreements = findAgreements(streams);

  // Find enrichments (unique contributions)
  const enrichments = findEnrichments(streams);

  // Find contradictions (eddies)
  const eddies = findContradictions(streams);

  // Weave the river
  return weaveRiver(agreements, enrichments, eddies);
}
```

### The Still Lake (Refinement)

Before reaching the ocean, the river passes through the Still Lake. This is a single LLM pass with a specific instruction: **be still**.

The Still Lake does not transform the river. It settles the mud. It lets clarity emerge. It asks:

1. **Is this clear?** Can the user understand it without effort?
2. **Is this whole?** Is anything missing that the user needs?
3. **Is this kind?** Does it carry warmth and care?
4. **Is this true?** Not just factually correct, but genuinely honest?
5. **Is this simple?** Can anything be removed without losing meaning?

If the answer to all five is yes, the water passes to the ocean. If not, the Still Lake makes gentle adjustments -- never rewriting, only polishing.

---

## Stream Volume Control

*"The ancient Masters were profound and subtle. Their wisdom was unfathomable. There is no way to describe it; all we can describe is their appearance. They were careful as someone crossing an iced-over stream."*
*-- Tao Te Ching, Chapter 15*

Not every request needs all springs. The system practices economy -- using only the water that is needed.

### The Volume Sensor

A lightweight pre-pass (the most minimal LLM call possible -- a single classification) determines the **volume** of the rain:

| Volume | Description | Springs Activated | Full Flow? |
|--------|-------------|-------------------|------------|
| Droplet | Simple greeting, yes/no question | Desert only | No confluence needed |
| Shower | Clear question, moderate complexity | Best 2 springs | Light confluence |
| Downpour | Complex task, multi-domain | All springs | Full confluence + Still Lake |
| Storm | Transformative/recursive task | All springs, multiple passes | Full cycle, may recurse |

The Volume Sensor is the one place where the system makes an active routing decision. But even this follows wu wei -- it is the gentlest possible touch, a single classification that opens or closes valves. It does not control where the water goes, only how much is released.

---

## The Cycle of Water

*"Returning is the motion of the Tao."*
*-- Tao Te Ching, Chapter 40*

Water cycles. Rain becomes streams, streams become rivers, rivers reach the ocean, and the ocean evaporates to become vapor and rain again.

In the system, this cycle manifests as **conversation continuity**. The ocean (delivered output) becomes new vapor (context for the next input). The user's response is new rain that falls on a watershed already shaped by previous flows.

Each cycle deepens the riverbed. The channels become more defined. The springs learn (through context) what this particular user needs. The system becomes more efficient and more attuned with each pass -- not through explicit learning, but through the natural deepening that happens when water flows repeatedly over the same terrain.

```
Cycle:
  Rain -> Streams -> River -> Lake -> Ocean
    ^                                   |
    |          (evaporation)            |
    +---- Vapor <-----------------------+
```

This is the eternal return. The Tao moving in its circle. The system breathing in and out.
