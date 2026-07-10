# systems-lab

Small Rust experiments for understanding computer systems.

The goal of this repository is learning, not production-quality software. Each
experiment should isolate one concept, make the behavior observable, and explain
why that behavior happens.

## Layout

```text
systems-lab/
├── Cargo.toml
├── README.md
├── experiments/
│   ├── README.md
│   ├── memory/
│   │   └── <experiment>/
│   ├── processes/
│   │   └── <experiment>/
│   ├── filesystems/
│   │   └── <experiment>/
│   ├── networking/
│   │   └── <experiment>/
│   ├── concurrency/
│   │   └── <experiment>/
│   └── cpu/
│       └── <experiment>/
├── docs/
│   ├── conventions.md
│   ├── experiment-template.md
│   ├── git-workflow.md
│   ├── roadmap.md
│   └── sessions/
```

## Start Here

- `experiments/`: runnable Rust experiments grouped by systems topic.
- `docs/conventions.md`: repository conventions, crate naming, and workspace
  member policy.
- `docs/experiment-template.md`: template for a new experiment README.
- `docs/roadmap.md`: possible future experiments.
- `docs/sessions/`: optional learning session retrospectives.

## Running an Experiment

From the repository root:

```sh
cargo run -p <experiment-crate-name>
```

Or from an individual experiment directory:

```sh
cargo run
```
