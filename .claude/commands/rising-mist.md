Carry the lessons upward. Take what the reflection revealed and reshape the vision ahead.

*"Each separate being in the universe returns to the common source. Returning to the source is serenity."*
*-- Tao Te Ching, Chapter 16*

The Reflecting Pool looks inward -- showing the system as it is. The Rising Mist carries what was seen upward -- reshaping the clouds so the next rain falls differently. One is observation. The other is transformation.

After a phase is complete and its reflections are written (`docs/phase_*_reflections.md`), the mist rises. Read:

1. The **most recent phase reflections** -- what was learned
2. All **previous reflections** -- the trajectory of insight
3. `docs/implementation.md` -- the forward vision, especially phases not yet built
4. `docs/philosophy.md` -- the foundation that does not change

If $ARGUMENTS names a specific phase or area, focus there. Otherwise, examine the full forward vision.

Then ask these three questions:

### 1. What did the reflection reveal that the vision doesn't yet carry?

Each phase discovers something. Phase 2 discovered the map was not the territory. Phase 3 discovered the system needs to cycle. Phase 4 discovered yielding should change the yielder. These insights reshape what comes next. Where do the remaining phase descriptions still reflect the *old* understanding -- before the lesson was learned?

### 2. How should the next phases change?

Not implementation details -- the vision doc uses pseudocode and prose, not code. But the *shape* of what's coming. A phase description written before the last three phases may assume things that are no longer true, or miss possibilities that the reflections opened. Update the descriptions to carry the new understanding forward.

### 3. Has a phase dissolved or merged?

Sometimes a reflection reveals that a future phase is not what we thought. Phase 3 suggested the creation module might not need to exist. Phase 4 suggested yielding memory belongs in recursion, not refinement. A phase that was planned before its predecessors were built may need to merge with another, split into two, or dissolve entirely. The implementation doc should reflect this honestly -- the map must follow the territory.

---

For each change to `docs/implementation.md`:

- **Update phase descriptions** to carry forward what the reflections revealed
- **Preserve pseudocode and prose** -- do not introduce Rust code blocks
- **Keep it concise** -- the implementation doc should get simpler over time, not longer (Chapter 48)
- **Note what shifted** -- a brief line about why the description changed, so the next reader understands the trajectory

After updating, commit the changes on a branch. The mist has risen; the clouds have reshaped; the next rain will fall differently.

*"Return is the movement of the Tao. Yielding is the way of the Tao."*
*-- Tao Te Ching, Chapter 40*
