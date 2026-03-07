Scaffold a new module or component in the Tao Flow system.

Arguments: $ARGUMENTS (expected: module path, e.g., "confluence/yielding" or "water/tide")

Steps:

1. Determine where this module belongs in the project structure
2. Create the source file at the appropriate path under `src/`
3. Include:
   - Module documentation with a Tao Te Ching or Hua Hu Ching quote that captures its essence
   - Core struct(s) with `Debug, Clone, Serialize, Deserialize` derives
   - Meaningful methods with documentation
   - Inline `#[cfg(test)] mod tests` with tests that verify the shape holds
4. Register the module in its parent `mod.rs`
5. If this module introduces new public types, re-export them appropriately
6. Run `/riverbank` to verify everything compiles and tests pass

The vessel is the emptiness that holds the water. Define the shape clearly.
The water will fill it in time.
