# Implementation: From Philosophy to Code

*"A journey of a thousand miles starts under one's feet."*
*-- Tao Te Ching, Chapter 64*

---

## The Uncarved Block

This document translates the philosophy into practical implementation. But remember:

*"The Tao that can be told is not the eternal Tao."*

The implementation is not the system. The system is the emptiness that the implementation creates. Do not mistake the code for the Tao.

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

### Streaming: Server-Sent Events (SSE) / Streaming

The stream of input into the system is water-based. The system uses streaming protocols everywhere:
- User input streams in (SSE or WebSocket via `axum`)
- LLM responses stream out (SSE from each provider via `reqwest`)
- The Confluence processes streams in real-time, not batch
- The user sees the output forming like water filling a pool

### LLM Integration: Provider-Agnostic Traits

Like water taking the shape of any container, the system adapts to any LLM provider. Each spring implements a common `trait` behind which any provider can live. LLM APIs are HTTP -- Rust speaks HTTP fluently through `reqwest`.

### Serialization: Serde

Serde is water for data -- it flows JSON, YAML, MessagePack, anything. Configuration, API payloads, and internal messages all serialize naturally.

---

## Project Structure

```
tao_flow/
  Cargo.toml                    # The manifest -- what the vessel is made of

  src/
    main.rs                     # The entry point -- the mouth of the river
    lib.rs                      # The library root -- the watershed exposed

    water/
      mod.rs
      vapor.rs                  # Context and session state
      rain.rs                   # Input reception and sensing
      stream.rs                 # Individual LLM response
      river.rs                  # Merged output
      ocean.rs                  # Delivered output

    watershed/
      mod.rs
      spring.rs                 # The Spring trait (LLM wrapper)
      springs/
        mod.rs
        mountain.rs             # Deep reasoning spring
        forest.rs               # Creative spring
        desert.rs               # Fast/efficient spring
        underground.rs          # Specialized spring
      volume_sensor.rs          # Rain volume classifier

    confluence/
      mod.rs
      pool.rs                   # Stream merging
      eddy.rs                   # Divergence detection
      yielding.rs               # The yielding protocol
      settling.rs               # Conflict resolution

    still_lake/
      mod.rs
      lake.rs                   # Final refinement
      clarity.rs                # The five questions

    creation/
      mod.rs
      seed.rs                   # Finding the seed of a creation
      vessel.rs                 # Creating the empty structure
      flow.rs                   # Multi-pass creation
      book.rs                   # Book-specific creation flow
      podcast.rs                # Podcast-specific creation flow
      software.rs               # Software-specific creation flow

    config/
      mod.rs
      springs.rs                # Spring configuration loading
      affinities.rs             # Natural affinities

  config/
    springs.yaml                # Spring configuration
    affinities.yaml             # Natural affinities
    prompts/
      mountain.md               # Mountain spring system prompt
      forest.md                 # Forest spring system prompt
      desert.md                 # Desert spring system prompt
      confluence.md             # Confluence integration prompt
      yielding.md               # Yielding prompt template
      still_lake.md             # Still Lake refinement prompt

  tests/
    integration/
      mod.rs
      rain_to_ocean.rs          # Full flow integration tests
      confluence_tests.rs       # Multi-stream merging tests
      yielding_tests.rs         # Conflict resolution tests
      creation_tests.rs         # Book/podcast/software creation tests

    properties/
      mod.rs
      water_invariants.rs       # Property-based tests for water state transitions
      flow_properties.rs        # The water always reaches the ocean
```

---

## Core Abstractions

### Water Types

```rust
use serde::{Deserialize, Serialize};

/// The volume of rain determines how many springs respond.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Volume {
    Droplet,    // Simple, single-spring sufficient
    Shower,     // Moderate, 2-3 springs
    Downpour,   // Complex, all springs
    Storm,      // Transformative, multiple passes
}

/// The nature of an eddy -- how streams disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EddyNature {
    Factual,        // One is right, one is wrong
    Interpretive,   // Both may be valid
    Stylistic,      // Diversity, not conflict
    Structural,     // Different organization, same truth
}

/// Context -- the atmosphere before rain falls.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vapor {
    pub conversation_history: Vec<Message>,
    pub user_preferences: Preferences,
    pub session_context: SessionContext,
    pub emotional_temperature: f32, // -1.0 cold/analytical, +1.0 warm/emotional
}

/// User input -- undifferentiated, natural.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rain {
    pub raw_input: String,
    pub vapor: Vapor,
    pub volume: Volume,
    pub temperature: f32,
    pub minerals: Vec<String>,
}

/// An LLM's natural response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub source: String,
    pub content: String,
    pub flow_rate: f32,
    pub clarity: f32,
    pub depth: f32,
    pub temperature: f32,
}

/// A point of divergence between streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eddy {
    pub topic: String,
    pub positions: Vec<Position>,
    pub nature: EddyNature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub source: String,
    pub view: String,
}

/// Merged output from confluence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct River {
    pub content: String,
    pub tributaries: Vec<String>,
    pub eddies: Vec<Eddy>,
    pub clarity: f32,
}

/// What the user receives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ocean {
    pub content: String,
    pub depth: f32,
    pub warmth: f32,
}
```

The compiler enforces that these types are distinct. You cannot pass a `Stream` where a `River` is expected. You cannot deliver a `River` to the user -- it must become an `Ocean` first. The type system **is** the water's journey, and the compiler ensures every drop follows the path.

### The Spring Trait (The Empty Pot)

```rust
use async_trait::async_trait;

/// A spring in the watershed. Each spring wraps an LLM
/// and responds to rain according to its nature.
///
/// "The supreme good is like water, which nourishes
/// all things without trying to." -- Chapter 8
#[async_trait]
pub trait Spring: Send + Sync {
    /// The spring's name -- its identity in the watershed.
    fn name(&self) -> &str;

    /// The spring's nature -- what it naturally provides.
    fn nature(&self) -> &str;

    /// How strongly does this spring resonate with this rain?
    fn sense_relevance(&self, rain: &Rain) -> f32;

    /// Respond to rain according to nature.
    /// Returns None if this spring has nothing to contribute
    /// (a dry spring -- natural and valid).
    async fn respond(&self, rain: &Rain) -> Result<Option<Stream>, SpringError>;
}

/// Base implementation shared by all springs.
pub struct SpringBase {
    pub name: String,
    pub nature: String,
    pub models: Vec<ModelConfig>,
    pub affinities: HashMap<String, f32>,
}

impl SpringBase {
    pub fn sense_relevance(&self, rain: &Rain) -> f32 {
        let mut score: f32 = 0.3; // Base -- every spring has something to offer
        for mineral in &rain.minerals {
            if let Some(&affinity) = self.affinities.get(mineral) {
                score += affinity;
            }
        }
        score.min(1.0)
    }

    pub fn assess_depth(content: &str) -> f32 {
        let words = content.split_whitespace().count();
        match words {
            0..=49 => 0.2,
            50..=199 => 0.5,
            _ => 0.8,
        }
    }
}
```

The `trait Spring` is the empty pot. It defines the shape -- the emptiness -- and leaves each spring free to fill it with its own nature. The `Send + Sync` bounds ensure that springs can flow concurrently without data races. The compiler enforces this. Wu wei.

### The Watershed

```rust
use tokio::task::JoinSet;

/// The watershed does not decide where rain goes.
/// It simply has a shape, and water follows that shape.
pub struct Watershed {
    springs: Vec<Box<dyn Spring>>,
    volume_sensor: VolumeSensor,
}

impl Watershed {
    pub fn new(springs: Vec<Box<dyn Spring>>) -> Self {
        Self {
            springs,
            volume_sensor: VolumeSensor::new(),
        }
    }

    /// All springs receive rain. Each responds according to its nature.
    pub async fn receive_rain(&self, rain: &mut Rain) -> Vec<Stream> {
        // Sense the volume
        rain.volume = self.volume_sensor.sense(rain).await;

        // Select springs based on volume (wu wei -- minimal intervention)
        let active_springs = self.activate_springs(rain.volume);

        // All active springs flow simultaneously
        let mut tasks = JoinSet::new();
        for spring in active_springs {
            let rain_clone = rain.clone();
            tasks.spawn(async move {
                spring.respond(&rain_clone).await
            });
        }

        // Gather the streams, filtering dry springs
        let mut streams = Vec::new();
        while let Some(result) = tasks.join_next().await {
            if let Ok(Ok(Some(stream))) = result {
                streams.push(stream);
            }
        }
        streams
    }

    fn activate_springs(&self, volume: Volume) -> Vec<&dyn Spring> {
        match volume {
            Volume::Droplet => {
                // Only the desert spring -- light rain, quick response
                self.springs.iter()
                    .filter(|s| s.name() == "desert")
                    .map(|s| s.as_ref())
                    .collect()
            }
            Volume::Shower => {
                // The two most relevant springs
                self.springs.iter()
                    .take(2)
                    .map(|s| s.as_ref())
                    .collect()
            }
            Volume::Downpour | Volume::Storm => {
                // All springs flow
                self.springs.iter()
                    .map(|s| s.as_ref())
                    .collect()
            }
        }
    }
}
```

### The Confluence

```rust
/// Where streams merge.
///
/// "If you want to become whole, let yourself be partial." -- Chapter 22
pub struct ConfluencePool {
    integrator: ModelConfig,
}

impl ConfluencePool {
    pub async fn merge(&self, streams: Vec<Stream>, rain: &Rain) -> Result<River, FlowError> {
        match streams.len() {
            0 => {
                // No springs responded. Silence.
                Ok(River {
                    content: String::new(),
                    tributaries: vec![],
                    eddies: vec![],
                    clarity: 0.0,
                })
            }
            1 => {
                // Single stream. Wu wei -- no merging needed.
                let stream = streams.into_iter().next().unwrap();
                Ok(River {
                    tributaries: vec![stream.source.clone()],
                    content: stream.content,
                    eddies: vec![],
                    clarity: stream.clarity,
                })
            }
            _ => {
                // Multiple streams -- find the confluence
                let analysis = self.analyze(&streams).await?;

                // Resolve contradictions through yielding
                let mut resolved = Vec::new();
                for eddy in &analysis.contradictions {
                    let resolution = self.yield_and_settle(eddy, rain).await?;
                    resolved.push(resolution);
                }

                // Weave the river
                let content = self.weave(
                    &analysis.agreements,
                    &analysis.enrichments,
                    &resolved,
                    rain,
                ).await?;

                Ok(River {
                    tributaries: streams.iter().map(|s| s.source.clone()).collect(),
                    content,
                    eddies: analysis.contradictions,
                    clarity: 0.8,
                })
            }
        }
    }

    /// The yielding protocol.
    /// Each position is asked to find truth in the other.
    async fn yield_and_settle(&self, eddy: &Eddy, rain: &Rain) -> Result<String, FlowError> {
        let mut yielded_positions = Vec::new();
        for position in &eddy.positions {
            let others: Vec<&Position> = eddy.positions.iter()
                .filter(|p| p.source != position.source)
                .collect();
            let yielded = self.call_yielding_prompt(position, &others).await?;
            yielded_positions.push(yielded);
        }

        self.settle(eddy, &yielded_positions, rain).await
    }
}
```

### The Still Lake

```rust
/// "Do you have the patience to wait
///  till your mud settles and the water is clear?" -- Chapter 15
pub struct StillLake {
    model: ModelConfig,
}

impl StillLake {
    pub async fn clarify(&self, river: River, rain: &Rain) -> Result<Ocean, FlowError> {
        let prompt = format!(
            "You are the Still Lake -- the final stage of refinement.\n\n\
             The following response has flowed through multiple perspectives\n\
             and been integrated. Your role is not to change it, but to polish it.\n\
             Like still water polishing a stone: gentle, patient, present.\n\n\
             Ask yourself:\n\
             1. Is this clear? Can the reader understand without effort?\n\
             2. Is this whole? Is anything missing that should be present?\n\
             3. Is this kind? Does it carry warmth and care?\n\
             4. Is this true? Not just accurate, but genuinely honest?\n\
             5. Is this simple? Can anything be removed without losing meaning?\n\n\
             Make only the gentlest adjustments. Do not rewrite. Polish.\n\n\
             Original request: {}\n\n\
             Response to refine:\n{}",
            rain.raw_input, river.content
        );

        let refined = self.call_model(&prompt).await?;

        Ok(Ocean {
            content: refined,
            depth: Self::assess_depth(&river),
            warmth: Self::assess_warmth(&river, rain),
        })
    }
}
```

---

## The Main Flow

```rust
/// The complete system. Rain to Ocean.
///
/// "The Tao gives birth to One. One gives birth to Two.
///  Two gives birth to Three.
///  Three gives birth to ten thousand things." -- Chapter 42
pub struct TaoFlow {
    watershed: Watershed,
    confluence: ConfluencePool,
    still_lake: StillLake,
    vapor: Vapor,
}

impl TaoFlow {
    pub fn new(config: Config) -> Result<Self, ConfigError> {
        let springs = config.create_springs()?;
        Ok(Self {
            watershed: Watershed::new(springs),
            confluence: ConfluencePool::new(config.integrator_model),
            still_lake: StillLake::new(config.refinement_model),
            vapor: Vapor::default(),
        })
    }

    /// The complete journey from rain to ocean.
    pub async fn flow(&mut self, user_input: &str) -> Result<String, FlowError> {
        // Rain falls
        let mut rain = Rain {
            raw_input: user_input.to_string(),
            vapor: self.vapor.clone(),
            volume: Volume::Shower,
            temperature: 0.0,
            minerals: vec![],
        };

        // Springs respond
        let streams = self.watershed.receive_rain(&mut rain).await;

        // Streams merge at confluence
        let river = self.confluence.merge(streams, &rain).await?;

        // River passes through the still lake
        let ocean = self.still_lake.clarify(river, &rain).await?;

        // Update vapor for next cycle (the water cycle)
        self.update_vapor(&rain, &ocean);

        // The ocean reaches the user
        Ok(ocean.content)
    }

    /// The water cycle -- output becomes context for next input.
    fn update_vapor(&mut self, rain: &Rain, ocean: &Ocean) {
        self.vapor.conversation_history.push(Message {
            role: Role::User,
            content: rain.raw_input.clone(),
        });
        self.vapor.conversation_history.push(Message {
            role: Role::Assistant,
            content: ocean.content.clone(),
        });
    }
}
```

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

Each module carries its own tests, inline with the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_spring_returns_none() {
        // A spring with no affinity for the rain produces nothing.
        // Silence is wisdom.
        let spring = MountainSpring::new(test_config());
        let rain = Rain::new("format this CSV", Vapor::default());
        let result = tokio_test::block_on(spring.respond(&rain));
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn single_stream_needs_no_confluence() {
        // Wu wei -- when one stream flows alone, do nothing.
        let pool = ConfluencePool::new(test_integrator());
        let streams = vec![Stream::new("desert", "Hello!")];
        let rain = Rain::new("hi", Vapor::default());
        let river = tokio_test::block_on(pool.merge(streams, &rain)).unwrap();
        assert_eq!(river.content, "Hello!");
        assert!(river.eddies.is_empty());
    }

    #[test]
    fn eddy_detection_finds_contradictions() {
        let stream_a = Stream::new("mountain", "Use Python for this task.");
        let stream_b = Stream::new("forest", "Use JavaScript for this task.");
        let eddies = detect_eddies(&[stream_a, stream_b]);
        assert_eq!(eddies.len(), 1);
        assert_eq!(eddies[0].nature, EddyNature::Interpretive);
    }

    #[test]
    fn volume_sensor_recognizes_storm() {
        let rain = Rain::new(
            "Design a complete distributed system for real-time collaborative \
             document editing with conflict resolution, offline support, and \
             end-to-end encryption. Include architecture, data models, and \
             implementation plan.",
            Vapor::default(),
        );
        let sensor = VolumeSensor::new();
        let volume = tokio_test::block_on(sensor.sense(&rain));
        assert!(matches!(volume, Volume::Downpour | Volume::Storm));
    }
}
```

### Level 3: Integration Tests (The Watershed Boundary)

Full flow tests that verify rain becomes ocean:

```rust
// tests/integration/rain_to_ocean.rs

#[tokio::test]
async fn simple_question_flows_to_ocean() {
    let mut tao = TaoFlow::new(test_config()).unwrap();
    let ocean = tao.flow("What is the Tao?").await.unwrap();
    assert!(!ocean.is_empty());
    // The ocean should carry depth for a philosophical question
}

#[tokio::test]
async fn storm_request_cycles_through_watershed() {
    let mut tao = TaoFlow::new(test_config()).unwrap();
    let ocean = tao.flow(
        "Write a complete book outline about the nature of consciousness"
    ).await.unwrap();
    // Storm requests should produce deep, structured output
    assert!(ocean.len() > 500);
}

#[tokio::test]
async fn vapor_carries_context_between_flows() {
    let mut tao = TaoFlow::new(test_config()).unwrap();
    let _ = tao.flow("My name is River.").await.unwrap();
    let ocean = tao.flow("What is my name?").await.unwrap();
    assert!(ocean.contains("River"));
}

#[tokio::test]
async fn yielding_resolves_factual_eddy() {
    // When two springs disagree on a fact, yielding should resolve it
    let pool = ConfluencePool::new(test_integrator());
    let eddy = Eddy {
        topic: "capital of France".into(),
        positions: vec![
            Position { source: "mountain".into(), view: "Paris".into() },
            Position { source: "forest".into(), view: "Lyon".into() },
        ],
        nature: EddyNature::Factual,
    };
    let rain = Rain::new("What is the capital of France?", Vapor::default());
    let resolution = pool.yield_and_settle(&eddy, &rain).await.unwrap();
    assert!(resolution.contains("Paris"));
}
```

### Level 4: Property-Based Tests (The Laws of Nature)

Properties that must hold regardless of input -- the invariants of water:

```rust
// tests/properties/water_invariants.rs

use proptest::prelude::*;

proptest! {
    /// Water always reaches the ocean. No input should cause the system to hang or crash.
    #[test]
    fn water_always_reaches_ocean(input in "\\PC{1,1000}") {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut tao = TaoFlow::new(test_config()).unwrap();
        let result = rt.block_on(tao.flow(&input));
        assert!(result.is_ok());
    }

    /// The ocean is never turbid -- output should not contain system internals.
    #[test]
    fn ocean_carries_no_system_internals(input in "\\PC{1,500}") {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut tao = TaoFlow::new(test_config()).unwrap();
        let ocean = rt.block_on(tao.flow(&input)).unwrap();
        // The user should never see the machinery
        assert!(!ocean.contains("CONFLUENCE_PROMPT"));
        assert!(!ocean.contains("YIELDING_PROMPT"));
        assert!(!ocean.contains("EddyNature"));
    }

    /// Vapor accumulates -- context grows with each flow.
    #[test]
    fn vapor_accumulates(inputs in prop::collection::vec("\\PC{1,100}", 1..10)) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut tao = TaoFlow::new(test_config()).unwrap();
        for (i, input) in inputs.iter().enumerate() {
            rt.block_on(tao.flow(input)).unwrap();
            assert_eq!(tao.vapor.conversation_history.len(), (i + 1) * 2);
        }
    }
}
```

### Testing the Creations

The system creates books, podcasts, and software. Each creation type needs its own tests:

```rust
#[tokio::test]
async fn book_creation_finds_seed() {
    let creator = BookCreator::new(test_config());
    let seed = creator.find_seed("A book about letting go").await.unwrap();
    assert!(!seed.essence.is_empty());
    assert!(!seed.organizing_principle.is_empty());
}

#[tokio::test]
async fn software_creation_produces_tests() {
    // The system that creates software must create tested software.
    // Tests creating tests -- the recursive riverbank.
    let creator = SoftwareCreator::new(test_config());
    let output = creator.create("A CLI tool that counts words").await.unwrap();
    assert!(output.contains("#[test]") || output.contains("fn test_"));
}
```

---

## System Prompts (The Riverbed)

The system prompts remain unchanged from the philosophy -- they are the riverbed, and the riverbed does not change when the water changes from Python to Rust. The prompts for Mountain Spring, Forest Spring, Confluence, Yielding, and Still Lake remain as described in the architecture document. They are stored as `.md` files in `config/prompts/` and loaded at runtime.

---

## Configuration

### Cargo.toml

```toml
[package]
name = "tao_flow"
version = "0.1.0"
edition = "2024"
description = "A multi-LLM system that flows like water"

[dependencies]
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
reqwest = { version = "0.12", features = ["json", "stream"] }
axum = { version = "0.8", features = ["ws"] }
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "2"
futures = "0.3"

[dev-dependencies]
proptest = "1"
tokio-test = "0.4"
wiremock = "0.6"              # Mock HTTP for LLM API testing
assert_matches = "1.5"
```

### springs.yaml

```yaml
springs:
  mountain:
    nature: "Deep reasoning, analysis, architecture, philosophy"
    models:
      - provider: anthropic
        model: claude-opus-4-6
      - provider: openai
        model: o3
    affinities:
      philosophy: 0.9
      architecture: 0.9
      analysis: 0.8
      strategy: 0.8
      ethics: 0.7
      debugging: 0.6
    flow_rate: slow

  forest:
    nature: "Creativity, narrative, empathy, dialogue, beauty"
    models:
      - provider: anthropic
        model: claude-sonnet-4-6
      - provider: openai
        model: gpt-4o
    affinities:
      narrative: 0.9
      poetry: 0.9
      empathy: 0.8
      brainstorming: 0.8
      dialogue: 0.7
      humor: 0.7
    flow_rate: moderate

  desert:
    nature: "Speed, efficiency, simple tasks, classification"
    models:
      - provider: anthropic
        model: claude-haiku-4-5
      - provider: google
        model: gemini-flash
    affinities:
      quick_answers: 0.9
      formatting: 0.8
      translation: 0.8
      classification: 0.7
      summarization: 0.7
    flow_rate: fast

  underground:
    nature: "Code, audio, images, specialized domains"
    models:
      - provider: meta
        model: code-llama
      - provider: openai
        model: whisper
      - provider: stability
        model: stable-diffusion
    affinities:
      code: 0.9
      audio: 0.8
      images: 0.8
      mathematics: 0.7
      data_analysis: 0.7
    flow_rate: variable
```

---

## Deployment

### The Minimal Viable Watershed

Start small. Like a spring emerging from rock.

All work flows through branches. Main is the ocean floor -- stable, settled, tested. Each feature emerges as a branch, is refined through PR review, and merges only when clear. Use `/flow` to guide the full cycle.

**Phase 1: The Vessel** *(complete)*
Set up the Rust project. Define the water types. Implement the `Spring` trait. Write the first tests. The compiler is the first master. `cargo test` passes green. CI enforces the riverbanks on every push. Skills are defined: `/riverbank`, `/spring`, `/vessel`, `/still-lake`, `/rain`, `/confluence`, `/flow`.

**Phase 2: Two Springs**
Begin with Mountain (Claude Opus) and Desert (Claude Haiku). This gives depth and speed. No Confluence needed yet -- when only two springs flow, the merging is simple. Use `/spring mountain` and `/spring desert` to scaffold. Test that rain flows to ocean through both paths.

**Phase 3: Add the Confluence**
Add the Forest Spring (`/spring forest`). Now three streams can diverge. The Confluence Pool becomes necessary. Implement the basic merging logic. Use `/vessel confluence/pool` to scaffold. Write integration tests that verify three streams merge into one river. Use `/confluence` to review the integration.

**Phase 4: Add Yielding**
When real eddies emerge, implement the Yielding protocol (`/vessel confluence/yielding`). This is where the system's true nature begins to manifest. Property-based tests verify that yielding always produces resolution. Use `/still-lake` to review the protocol for the five qualities.

**Phase 5: The Still Lake**
Add the final refinement pass (`/vessel still_lake/lake`). The system now flows from Rain to Ocean through all phases. Integration tests verify the full journey. Use `/rain suite` to watch water flow through every stage.

**Phase 6: Creation Flows**
Implement the specialized creation flows for books, podcasts, and software. Each builds on the core watershed. Test that created software contains its own tests -- the recursive riverbank. New skills may emerge as creation patterns solidify.

**Phase 7: The Storm**
Implement recursive flow -- the ability for water to cycle back through the watershed for transformative requests. Property-based tests verify that storms always eventually reach the ocean.

---

## Skills: Teaching the Builder

*"So find a teacher who is an integral being, a beacon who extends his light and virtue with equal ease to those who appreciate him and those who don't."*
*-- Hua Hu Ching, Chapter 75*

The system teaches its builders through skills -- slash commands that embody the Tao's principles while guiding real development workflows. Each skill is a small master, carrying one aspect of the practice. They live in `.claude/commands/` and are invoked as `/skill-name`.

### The Seven Skills

| Skill | The Water | Purpose |
|-------|-----------|---------|
| `/riverbank` | The banks that hold the river | Run all quality checks: `fmt`, `clippy`, `test`. The CI gate, locally. |
| `/spring <name>` | A new source of water | Scaffold a new Spring (LLM adapter) with trait impl, config, and tests. |
| `/vessel <path>` | The empty pot | Scaffold a new module with structure, docs, tests, and module registration. |
| `/still-lake <target>` | Clarity through stillness | Code review through the five questions: clarity, wholeness, kindness, truth, simplicity. |
| `/rain <input>` | Water falling on the watershed | Test input flow -- observe how rain would travel through the system. |
| `/confluence [branch]` | Where streams merge | Review and integrate changes, identifying agreements, enrichments, and eddies. |
| `/flow <description>` | The complete journey | Full dev cycle: branch, implement, test, review, PR. Rain to ocean. |

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

Rust's `Result` type makes errors visible and manageable. Errors in this system are not failures -- they are eddies in the flow. The system defines a clear error hierarchy:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("Spring '{0}' failed to respond: {1}")]
    SpringFailure(String, String),

    #[error("No springs produced water for this rain")]
    DroughtError,

    #[error("Confluence failed to merge streams: {0}")]
    ConfluenceFailure(String),

    #[error("Still Lake failed to clarify: {0}")]
    ClarityFailure(String),

    #[error("LLM provider error: {0}")]
    ProviderError(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
```

When a spring fails, the watershed continues -- the remaining springs still flow. A single dry spring does not dam the river. Only when ALL springs fail does the system report a drought. This resilience is natural to the architecture, and the type system ensures every error is handled.

---

## Monitoring (Watching the Water)

Rust's `tracing` crate provides structured, zero-overhead observability:

```rust
use tracing::{info, instrument};

#[instrument(skip(self, rain), fields(volume = ?rain.volume))]
pub async fn receive_rain(&self, rain: &mut Rain) -> Vec<Stream> {
    info!("Rain falls on the watershed");
    // ... springs respond ...
    info!(stream_count = streams.len(), "Springs have responded");
    streams
}
```

Tracing spans follow the water's journey from Rain to Ocean. The monitoring is as unobtrusive as watching a river from the bank -- it observes without disturbing the flow.

---

## The Final Word

*"With all this talking, what has been said? The subtle truth can be pointed at with words, but it can't be contained by them."*
*-- Hua Hu Ching, Chapter 81*

This implementation document points at the system. It does not contain it. The true system will emerge through building -- through the act of coding, compiling, testing, yielding to the compiler's wisdom, and flowing again.

Rust is the metal vessel. The Tao is the water. The tests are the riverbanks. The compiler is the master.

Build like water. When the compiler rejects your code, do not fight it -- yield, and find the path it reveals. When a test fails, do not force it to pass -- understand what the failure teaches. When the system resists, step back and let the design emerge.

*"Do you have the patience to wait till your mud settles and the water is clear?"*

The code will teach you its shape, as the river teaches the riverbed.
