# Allocation and deallocation

## Goal

Observe when a value owned by `Box` is dropped as its owner leaves a scope.

## Background

`Box<T>` owns a `T` stored in a heap allocation. The `Box` owner itself is a
local value. When that owner reaches the end of its scope, Rust drops the
boxed `T` and then releases the allocation used by the `Box`.

This experiment gives the boxed value a `Drop` implementation. Its printed
message makes the destruction step visible; the allocator's release operation
is not printed directly.

```text
inner scope
+----------------------------+
| boxed_value: Box<T>         |------> heap allocation containing T
+----------------------------+
              |
              | end of scope
              v
       drop T, then release allocation
```

## How to run

From the repository root:

```sh
cargo run -p memory-allocation-deallocation
```

Or from this experiment directory:

```sh
cargo run
```

## What to observe

The output should follow this order:

```text
entering outer scope
entering inner scope
using: boxed value
leaving inner scope
dropping: boxed value
back in outer scope
```

- `dropping: boxed value` appears after the last statement in the inner scope.
- The drop message appears before execution continues in the outer scope.
- There is no explicit call to `drop` in `main`; reaching the closing brace is
  enough.

## Why it happens

Rust inserts drop behavior when an owning value reaches the end of its scope.
Here, `boxed_value` owns both the `TrackedValue` and the heap allocation that
stores it. Dropping the `Box` first runs `TrackedValue::drop`, which prints the
message, and then the `Box` releases its allocation.

The print proves when the value's destructor runs. It does not independently
measure the allocator or prove when physical memory is reused by the operating
system. Deallocation here means returning the allocation to the process's
allocator.

## Try changing it

Before editing the program, predict how the output will change:

1. Move `println!("back in outer scope")` into the inner scope, after
   `println!("leaving inner scope")`.
2. Add `drop(boxed_value);` before `println!("leaving inner scope")`.

The first change does not end the lifetime early, while the second explicitly
drops the owner before the scope ends.

## Future connections

Later experiments can compare this deterministic drop timing with `Vec`
reallocation, reference-counted ownership, or resources such as files and
locks that also rely on `Drop` for cleanup.
