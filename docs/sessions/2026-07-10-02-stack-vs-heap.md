# Stack vs. heap experiment

## Date

2026-07-10

## Summary

Started the first recommended experiment at
`experiments/memory/stack-vs-heap/`. It compares the address of a directly
stored local value with the addresses of a local `Box` owner and its
heap-allocated value.

## Decisions

- Keep the experiment focused on one distinction: direct stack storage versus
  heap storage through `Box`.
- Print the `Box` handle address as well as the boxed value address so the
  indirection is visible.
- Describe address grouping as an observation rather than promising exact
  addresses or numeric ordering across platforms.
- Use Rust edition 2021 for compatibility with the repository's current Cargo
  version.

## Error patterns

The experiment manifest initially specified Rust edition 2024. Cargo 1.77
failed to parse it with:

```text
feature `edition2024` is required
```

The fix was to use:

```toml
edition = "2021"
```

Future experiments should continue using edition 2021 unless the repository
intentionally upgrades and pins a toolchain that supports edition 2024.

## Follow-up

- Before changing the experiment, read `src/main.rs` and explain what each of
  the three address-printing expressions observes: the direct local value, the
  `Box` owner, or the owned heap allocation.
- Try repeated runs and compare address grouping rather than exact values.
- Later experiments can examine `Vec` or `String` allocation separately.

## Suggested next-session prompt

```text
Please read AGENTS.md and the latest retrospective, then help me read through
the stack-vs-heap experiment code before we change it. I want to explain what
each printed address represents and run it several times to compare the address
grouping.
```
