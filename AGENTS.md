# AGENTS.md


# Purpose

This repository exists to understand computer systems through small Rust experiments.

Learning is always more important than production-quality software.

---

# Principles

- One experiment = one concept.
- Keep experiments independent.
- Prefer simple code over clever code.
- Explain why, not only how.
- Build understanding through experimentation.

---

# Structure

The repository currently uses a Cargo Workspace.

Repository-wide conventions are documented in `docs/conventions.md`.

Each experiment contains:

- Cargo.toml
- src/main.rs
- README.md

---

# Before Writing Code

Always explain:

- Goal
- Expected observations
- Why the experiment is useful

before generating code.

---

# README

Every experiment should contain:

- Goal
- Background
- How to run
- What to observe
- Why it happens

ASCII diagrams are encouraged.

Use `docs/experiment-template.md` when creating a new experiment README.

---

# Supporting Docs

- `docs/conventions.md`: repository conventions, crate naming, workspace member
  policy, and first-experiment recommendation.
- `docs/experiment-template.md`: template for new experiment READMEs.
- `docs/roadmap.md`: possible future experiments grouped by systems topic.
- `docs/sessions/`: optional session retrospectives.
- `docs/git-workflow.md`: detailed Git and PR workflow.

---

# Scope

Keep experiments small.

Avoid introducing multiple concepts unless requested.

Mention future connections briefly without implementing them.

---

# Learning Workflow

- Offer code-reading prompts when they help the user understand the next Rust or
  systems concept.
- It is acceptable to suggest safe failure experiments on throwaway branches,
  using `cargo check`, `cargo test`, or focused `cargo run` commands before
  expanding the experiment.
- When a real error occurs, capture the error pattern and fix in the experiment
  README or a session retrospective if it is likely to help later.
- At the end of each learning session, offer to create or update a short
  retrospective under `docs/sessions/`.
- Name retrospective files as `YYYY-MM-DD-NN-topic.md`, where `NN` is a
  two-digit sequence number for that date, for example
  `2026-07-10-01-memory-layout.md`.
- Do not read all retrospectives by default. Read the latest one only when the
  user asks to continue from the previous session or when context is unclear.

# Git and PR Workflow

Before making changes, committing, pushing, or opening a PR, read and follow
`docs/git-workflow.md`.
