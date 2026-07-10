# Repository conventions

This document records decisions that should guide future experiments.

## Experiment directories

Place each experiment under:

```text
experiments/<topic>/<experiment-name>/
```

Use broad topic directories such as:

- `memory/`
- `processes/`
- `filesystems/`
- `networking/`
- `concurrency/`
- `cpu/`

Each experiment should contain:

```text
Cargo.toml
README.md
src/main.rs
```

Each experiment README should explain:

- Goal
- Background
- How to run
- What to observe
- Why it happens

ASCII diagrams are encouraged when they make the system behavior easier to see.

## Crate names

Use topic-prefixed crate names:

```text
<topic>-<experiment-name>
```

Examples:

- `memory-stack-vs-heap`
- `memory-alignment`
- `processes-exit-status`

This avoids collisions when the workspace grows to many experiments.

## Workspace members

Add workspace members explicitly in the root `Cargo.toml`.

Start with:

```toml
[workspace]
members = []
resolver = "2"
```

When adding an experiment, add its path explicitly:

```toml
members = [
    "experiments/memory/stack-vs-heap",
]
```

Avoid broad workspace globs until there is a clear need. Explicit members make
Cargo behavior easier to understand while learning.

## First experiment

A good first experiment is:

```text
experiments/memory/stack-vs-heap/
```

It is small, observable, and establishes the standard shape for future
experiments.

## Session retrospectives

Retrospectives are optional. Create one when a session produces reusable
context, such as:

- an error pattern and fix
- a repository structure decision
- a connection between experiments
- follow-up questions for later

Store retrospectives under `docs/sessions/` with names like:

```text
YYYY-MM-DD-NN-topic.md
```
