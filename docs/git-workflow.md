# Git workflow

- Before making or moving changes, always run `git status --short --branch`.
- Before editing, confirm whether the current branch matches the current task,
  whether it contains work from a previous session, whether there are existing
  uncommitted changes, and which files are safe to edit, stage, or commit.
- Use a new feature branch for each independent task or session. Do not continue
  work on a branch from a previous session unless the user explicitly asks to
  amend that branch.
- If the current branch is from a previous task, keep current changes safe with
  `git stash`, switch to latest `main`, pull with
  `git pull --ff-only origin main`, create a new feature branch, then re-apply
  the changes.
- Before committing, always run `git status --short --branch` and inspect the
  diff.
- Before committing, inspect both unstaged and staged changes with `git diff`
  and `git diff --cached`.
- Stage only files related to the requested task. Do not use broad staging when
  the worktree contains unrelated changes.
- Use feature branches. Never commit directly to `main`.
- Use `main` as the default PR base unless the user specifies another branch.
- Open draft PRs by default.
- Never force push.
- For phase completion, prefer annotated tags, for example
  `phase-1-complete`.

## PR workflow

- Prefer the GitHub CLI (`gh`) as the primary GitHub operation path for this
  repository.
- Before creating or inspecting PRs with `gh`, check `gh auth status`. If
  authentication is invalid, tell the user to re-authenticate with
  `gh auth login -h github.com`.
- Use `gh pr create --draft` for draft PR creation after pushing the feature
  branch, unless the user explicitly asks for a ready-for-review PR.
- Use `gh pr view`, `gh pr diff`, and `gh pr checks` when inspecting PR
  details, diffs, or CI status.
- Treat Codex GitHub connector tools as a fallback or supplement. If connector
  permissions fail, continue with `gh` when available.
- PR descriptions should include:
  - what changed
  - why it changed
  - how it was verified
  - any remaining risk or follow-up
- For documentation-only changes, state that no runtime validation was needed.
- For Rust changes, include the Cargo commands that were run, such as
  `cargo check`, `cargo test`, or focused `cargo run` commands.
- When using `gh` for PR work, do not treat sandboxed authentication or network
  failures as the final state. GitHub API commands such as `gh auth status`,
  `gh pr list`, `gh pr view`, and `gh pr create` may need normal local
  environment access to read keychain credentials and reach the API. If a
  sandboxed `gh` command reports an invalid token or API connectivity failure,
  retry the same `gh` command with the required permission instead of asking
  the user to re-authenticate first.
