The complete development flow -- from branch to PR.

Arguments: $ARGUMENTS (description of the work to be done)

This skill guides the full development cycle:

1. **Rain** (Planning)
   - Understand the request: $ARGUMENTS
   - Read relevant existing code and docs
   - Identify which modules are affected

2. **Springs** (Implementation)
   - Create a feature branch from main: `git checkout -b feature/<short-description>`
   - Make the changes, following the system's principles
   - Write tests alongside the code -- the riverbanks form with the river

3. **Confluence** (Integration)
   - Run `/riverbank` to verify quality
   - Run `/still-lake staged` to review the changes
   - Fix any issues found

4. **Still Lake** (Refinement)
   - Review the full diff against main
   - Ensure clarity, wholeness, kindness, truth, simplicity
   - Make final adjustments

5. **Ocean** (Delivery)
   - Commit with a meaningful message
   - Push the branch
   - Create a PR with summary, changes, and test plan
   - Report the PR URL

Do NOT merge to main. The PR is the ocean -- it reaches the reviewer, not the shore.
