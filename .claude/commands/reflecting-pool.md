Look inward. Examine the system itself -- not its output, but its structure, its alignment, its health.

*"Without going outside, you may know the whole world. Without looking through the window, you may see the ways of heaven."*
*-- Tao Te Ching, Chapter 47*

The Still Lake looks outward -- polishing what the system produces for others. The Reflecting Pool looks inward -- showing the system itself, as it is. One is intelligence. The other is wisdom.

Read the source code in `src/`, the implementation doc (`docs/implementation.md`), `CLAUDE.md`, and `docs/philosophy.md`. If $ARGUMENTS names a specific area (e.g., "watershed", "types", "dependencies"), focus there. Otherwise, examine the whole system.

Then ask these five questions:

### 1. Does the map match the territory?

Compare `docs/implementation.md` with the actual source code. Where do they diverge? Types renamed but not updated in the doc. Structures described that don't exist. Code that exists but isn't documented. The map should follow the territory, not lead it astray.

### 2. Does everything carry water?

Look for fields initialized to defaults and never meaningfully used. Dependencies declared but not imported. Modules created but not connected. Code shaped before its water arrives. Chapter 48: *"In the practice of the Tao, every day something is dropped."*

### 3. Does the code speak, or does it explain?

Where does a comment restate what the code already says? Where does a Tao Te Ching quote justify what should be self-evident? Where does documentation describe what the types already enforce? The strongest code teaches without words.

### 4. Is the naming consistent?

Look for old names that survived a rename. Error variants that don't match their domain. Types whose names contradict their module. A name that was right in Phase 1 may be wrong in Phase 3. The naming should be one body, not a patchwork.

### 5. Has anything accumulated that should be dropped?

Test helpers that test nothing. Config structures for features not yet built. Abstractions created for a single use. Over-engineering disguised as preparation. What was added that hasn't earned its place?

---

For each finding, state:

- **What** -- the specific thing observed
- **Where** -- file and line
- **Why it matters** -- what drift or weight it introduces

Do not prescribe fixes. Observations, not directives. The water will find its own course.

*"Knowing others is intelligence; knowing yourself is true wisdom."*
*-- Tao Te Ching, Chapter 33*
