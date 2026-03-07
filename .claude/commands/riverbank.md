Run the full quality gate -- the riverbanks that keep the water in course.

Execute these checks in sequence, stopping at the first failure:

1. `cargo fmt --all -- --check` (Is the form clean?)
2. `cargo clippy --all-targets -- -D warnings` (Does the master approve?)
3. `cargo test --all-targets` (Do the riverbanks hold?)

If all three pass, report: "The riverbanks hold. N tests passed."

If any check fails:
- Show the failure clearly
- Suggest the fix (for fmt issues, offer to run `cargo fmt`; for clippy, show the suggestion; for test failures, investigate the root cause)
- Do NOT proceed to subsequent checks after a failure
