Scaffold a new Spring -- an LLM provider adapter for the watershed.

Arguments: $ARGUMENTS (expected: spring name, e.g., "mountain" or "coral_reef")

Steps:

1. Read `src/watershed/spring.rs` to understand the Spring trait
2. Create `src/watershed/springs/$ARGUMENTS.rs` with:
   - A struct implementing the `Spring` trait
   - A `SpringConfig` with appropriate affinities for this spring's nature
   - The `respond` method (initially returning a placeholder; actual LLM integration comes later)
   - Inline `#[cfg(test)] mod tests` with at least these tests:
     - The spring responds to rain matching its affinities
     - The spring stays dry (returns None) for rain outside its nature
     - Relevance scoring works correctly for this spring's minerals
3. Register the new spring in `src/watershed/springs/mod.rs`
4. Run `/riverbank` to verify everything compiles and tests pass

Follow the naming convention of the existing springs (mountain, forest, desert, underground).
The spring's nature should be clearly expressed in its struct documentation.
