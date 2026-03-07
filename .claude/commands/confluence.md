Review and integrate changes across the codebase -- the merging of streams.

Arguments: $ARGUMENTS (optional: branch name or PR number to review, or empty for current changes)

Steps:

1. Examine the changes:
   - If a branch/PR is specified, compare it against main
   - If empty, examine all uncommitted changes

2. For each changed file, identify:
   - **Agreements** -- Changes that align with the system's architecture and philosophy
   - **Enrichments** -- New ideas or approaches that add value
   - **Eddies** -- Potential conflicts with existing code, tests, or design principles

3. For each eddy found:
   - State the eddy clearly (what conflicts)
   - Apply the yielding principle: what truth does each side carry?
   - Suggest the natural resolution

4. Verify integration:
   - Run `/riverbank` to check quality
   - Ensure new code follows the water metaphor and Taoist principles where natural
   - Ensure tests cover new behavior

5. Summarize the confluence: what merged cleanly, what needed resolution, what remains
