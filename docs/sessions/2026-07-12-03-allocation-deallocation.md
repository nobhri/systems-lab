# Allocation, deallocation, and `Drop`

## Date

2026-07-12

## Summary

Added and ran the `memory-allocation-deallocation` experiment. The experiment
puts a `TrackedValue` inside a `Box`, gives `TrackedValue` a `Drop`
implementation, and uses an inner scope to make the destruction point easy to
see. The run confirmed that the destructor executes when the `Box` owner
leaves its scope and before execution continues in the outer scope.

## Goal and usefulness

The experiment isolates one concept: how scope-based ownership determines when
a boxed value is dropped. This connects the previous stack-versus-heap
experiment to Rust's automatic cleanup without introducing raw pointers or
allocator APIs.

## Verification

The following commands completed successfully:

```sh
cargo run -p memory-allocation-deallocation
cargo check --workspace
cargo test --workspace
```

The observed program output was:

```text
entering outer scope
entering inner scope
using: boxed value
leaving inner scope
dropping: boxed value
back in outer scope
```

The important sequence is:

```text
leaving inner scope
dropping: boxed value
back in outer scope
```

This shows that `TrackedValue::drop` runs at the end of the inner scope, before
the next statement in the outer scope. The `cargo test` output reported zero
tests because these binary experiments do not currently define automated
tests; the successful result still confirms that the workspace test targets
compile and run without test failures.

## Code-reading guide

Read the experiment in this order:

1. Find `TrackedValue` and identify what its `name` field is used for. It is a
   label that makes the destroyed value identifiable in the output.
2. Read `impl Drop for TrackedValue`. The `dropping: boxed value` line comes
   from this method, not from an explicit print in `main`.
3. Find `Box::new(...)`. The local `boxed_value` is the owner, while the
   `TrackedValue` it owns is stored in a heap allocation.
4. Match the lifetime of `boxed_value` to the braces of the inner scope. Its
   normal lifetime ends at the closing brace.
5. Compare the source order with the output order. This reveals where Rust's
   automatic drop behavior occurs even though `main` does not call
   `drop(boxed_value)` explicitly.

```text
stack                              heap
+-------------------------+       +-------------------------+
| boxed_value: Box<_>      |------>| TrackedValue            |
+-------------------------+       | name: "boxed value"     |
                                  +-------------------------+

inner scope ends
      |
      v
TrackedValue::drop runs
      |
      v
Box releases its allocation
```

## Understanding reached

- `Box<T>` owns the heap-allocated `T`; the local `Box<T>` handle is the owner
  whose scope controls cleanup in this experiment.
- Rust automatically performs drop behavior when an owning local value reaches
  the end of its scope.
- The custom `Drop` implementation makes destruction visible without requiring
  an explicit `drop` call in `main`.
- The destructor runs before execution continues past the scope's closing
  brace.
- Dropping the contained value and releasing the allocation are related steps,
  but they are not the same observation.
- The printed message directly demonstrates when `TrackedValue::drop` runs. It
  does not directly display the allocator operation.
- Releasing an allocation means returning it to the process allocator. It does
  not prove that the operating system immediately reclaims or reuses physical
  memory.

## Prediction exercise

Before changing the code, predict the output for each variation:

1. Move the outer-scope print into the inner scope. This changes print order
   but does not cause an earlier drop because the closing brace has not moved.
2. Add `drop(boxed_value);` before `leaving inner scope`. This consumes the
   owner and makes the destructor run at that explicit point.

The key questions when reading either version are:

- Who owns the boxed value?
- At which closing brace does that owner normally reach the end of its scope?
- Is cleanup happening because the scope ended or because the owner was
  explicitly passed to `drop`?

## Experiment status

The experiment now covers its intended concept: observing scope-based
destruction of a value owned by `Box`. Explicit early drop, `Vec`
reallocation, reference counting, and allocator internals should remain future
experiments rather than being added here.

## Suggested next-session prompt

```text
Please read AGENTS.md and the latest retrospective. Help me review the
allocation-and-deallocation experiment by predicting what changes when
drop(boxed_value) is called explicitly, then choose the next single-concept
memory experiment.
```
