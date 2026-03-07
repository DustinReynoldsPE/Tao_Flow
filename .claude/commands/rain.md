Send test input through the system and observe the flow.

Input: $ARGUMENTS (the test input to flow through the system, or "suite" to run the standard test suite)

If $ARGUMENTS is "suite":
- Run `cargo test --all-targets -- --nocapture` to see full test output
- Summarize: which springs responded, what volume was sensed, how many eddies formed, what reached the ocean

If $ARGUMENTS is specific input:
- Write a temporary test that creates Rain from the input
- Trace the flow: What volume does the VolumeSensor assign? Which springs would activate? What minerals are detected?
- Report the journey the water would take through the watershed
- Clean up the temporary test

This skill is for understanding how input flows through the system -- observing the water's path without disturbing it.
