# Experiment template

Use this template when creating a new experiment README.

~~~markdown
# <experiment name>

## Goal

State the single systems concept this experiment isolates.

## Background

Explain the minimum context needed before reading the code.

## How to run

From the repository root:

```sh
cargo run -p <crate-name>
```

Or from this experiment directory:

```sh
cargo run
```

## What to observe

List the specific output, timing, address, error, or behavior the reader should
look for.

## Why it happens

Explain the mechanism behind the observation. Prefer concrete language over
general claims.

## Try changing it

Optional: suggest one or two small changes the reader can make to test the
concept further.

## Future connections

Optional: mention related systems concepts without implementing them here.
~~~

Each experiment should still contain:

```text
Cargo.toml
README.md
src/main.rs
```
