Review code with the five questions of the Still Lake.

Target: $ARGUMENTS (a file path, module name, or "staged" for staged changes)

For each piece of code under review, ask:

1. **Clarity** -- Can a reader understand this without effort? Look for unclear variable names, convoluted logic, missing context. The code should read like still water -- transparent to the bottom.

2. **Wholeness** -- Is anything essential missing? Check for missing error handling at boundaries, missing tests for new behavior, incomplete trait implementations, unhandled enum variants.

3. **Kindness** -- Will the developer who maintains this feel calm or stressed? Look for overly clever code, unnecessary complexity, poor error messages. Code should bring peace.

4. **Truth** -- Is it honest? Check for misleading names, comments that contradict code, TODO comments hiding real problems, tests that don't actually verify behavior.

5. **Simplicity** -- Can anything be removed without losing meaning? Look for dead code, unnecessary abstractions, over-engineering, premature optimization.

Present findings organized by the five questions. For each finding, suggest the gentlest adjustment -- polish, don't rewrite. Trust the work that has already been done.
