# Initial repository structure

## Date

2026-07-10

## Summary

Created the initial structure for a Rust systems learning repository before
adding the first experiment.

## Decisions

- Use a Cargo workspace at the repository root.
- Keep experiments under `experiments/<topic>/<experiment-name>/`.
- Keep each experiment independent with its own `Cargo.toml`, `README.md`, and
  `src/main.rs`.
- Use `docs/` for cross-experiment material.
- Keep session retrospectives under `docs/sessions/`.
- Keep `AGENTS.md` focused and move detailed Git and PR rules to
  `docs/git-workflow.md`.
- Use explicit workspace members rather than broad globs.
- Use topic-prefixed crate names such as `memory-stack-vs-heap`.
- Add `docs/experiment-template.md` so experiment READMEs start from a
  consistent shape.
- Add `docs/conventions.md` so future sessions can recover repository decisions
  without relying on chat history.
- Keep the root README short and move detailed conventions into focused docs.
- Add `.gitignore` with `/target/` for future Cargo build output.

## Error patterns

`Cargo.toml` initially used:

```toml
members = ["experiments/*/*"]
```

With no experiment crates present, `cargo metadata --no-deps` failed because
Cargo tried to load a non-existent glob path.

The fix was to start with:

```toml
members = []
```

Future experiments should add their workspace member paths explicitly.

## Follow-up

- Create the first experiment at `experiments/memory/stack-vs-heap/`.
- Use `docs/experiment-template.md` for the experiment README.
- Add the first experiment path explicitly to the root workspace members.

## Suggested next-session prompt

```text
Please read AGENTS.md first. Then read the latest retrospective under
docs/sessions/ and continue from there.

I want to start the first experiment recommended by the repository conventions.
Do not write code until you explain the goal, expected observations, and why the
experiment is useful.
```
