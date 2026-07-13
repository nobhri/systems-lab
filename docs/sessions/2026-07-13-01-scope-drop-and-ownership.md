# Scope, `Drop`, and ownership

## Date

2026-07-13

## Summary

Reviewed the `memory-allocation-deallocation` experiment and connected its
observed drop timing to Rust's broader ownership model. The session clarified
the differences between scopes, variable bindings, pointers, heap allocations,
ownership, and borrowing.

The current experiment is now understood well enough to move on. The next
recommended single-concept experiment is ownership transfer through a move.

## Experiment reviewed

The experiment creates a boxed `TrackedValue` inside an inner scope:

```rust
{
    let boxed_value = Box::new(TrackedValue {
        name: "boxed value",
    });

    println!("using: {}", boxed_value.name);
}
```

Conceptually, the relationships are:

```text
variable binding                 heap allocation
+-------------------------+      +--------------------------+
| boxed_value             |      | TrackedValue             |
| Box<TrackedValue>        |----->| name: &'static str       |
+-------------------------+      +-------------+------------+
                                               |
                                               v
                                  static string literal
                                  "boxed value"
```

When the inner scope ends, Rust automatically begins dropping
`boxed_value`. The contained `TrackedValue::drop` method prints the visible
message, and the `Box` then releases its heap allocation.

## Understanding reached

### Scope and drop

- A block delimited by braces can introduce a scope.
- A local binding declared in an inner scope cannot be used after that scope
  ends.
- If an initialized local value still owns a resource when its scope ends,
  Rust normally drops it automatically.
- The scope determines where a binding can be used, while drop is the cleanup
  process applied to the owned value.
- Scope and ownership often interact, but they are not the same concept.

### What the custom `Drop` implementation does

The experiment defines:

```rust
impl Drop for TrackedValue {
    fn drop(&mut self) {
        println!("dropping: {}", self.name);
    }
}
```

- The `impl` block defines behavior; the whole block is not itself executed.
- Rust calls the `drop` method while dropping a `TrackedValue`.
- This particular method only prints a message. It makes destruction timing
  observable but does not directly release the `Box` allocation.
- The `Box` cleanup machinery releases the heap allocation after the contained
  value is dropped.
- The static string literal referenced by `name` is not owned by
  `TrackedValue`, so dropping the `TrackedValue` does not free that literal.

### Why automatic drop is useful

Automatic cleanup reduces the chance of:

- Forgetting to release heap memory
- Releasing the same allocation twice
- Accessing memory after it has been released
- Forgetting to close or release non-memory resources such as files and locks

This was compared with Python's `with` statement. A Python context manager
provides cleanup around a specific block, while Rust applies scope-based
cleanup broadly to owning values such as `Box`, `Vec`, files, and lock guards.

Rust does not make every possible memory leak impossible. For example,
reference-counting cycles can intentionally or accidentally retain memory. The
important result is that normal ownership-based cleanup is automatic and that
many invalid memory accesses are rejected before execution.

### Variable bindings, pointers, heap, and ownership

- A variable binding is the source-level name used to access a value. Its
  scope determines where that name is valid.
- A pointer is a value containing or representing a memory address.
- The heap is a region from which dynamically sized or dynamically lived
  allocations can be obtained.
- Ownership is not a memory region. It is Rust's rule for identifying which
  value is responsible for cleanup.
- Merely pointing to a value does not necessarily mean owning it. References
  point to values without taking ownership of them.

An ownership move is therefore not best understood as clearing an old variable
to save memory. It transfers responsibility for the same value to another
binding, preventing both bindings from independently cleaning it up.

```text
before move:

first  ----owns----> value

after move:

first  (cannot be used)
second ----owns----> same value
```

### Initial model of borrowing

Borrowing means temporarily accessing a value without taking ownership:

```rust
let owner = String::from("hello");
let borrowed = &owner;
```

The symbolic-link analogy was useful as a starting point because both provide
another route to existing data without copying it. The important difference is
that Rust checks that a reference cannot outlive the value it refers to.
Borrowing is therefore better understood as temporary, compiler-checked access
permission.

The two forms introduced were:

```text
&T      shared, read-only borrowing
&mut T  exclusive, mutable borrowing
```

Borrowing was discussed only conceptually. It should remain separate from the
next ownership-move experiment.

## Learning perspective

Rust's ownership rules initially appeared to be extra complexity compared with
Python. The session reframed them as safety equipment for an environment with
different risks: direct resource management, predictable destruction, and
low-level or concurrent execution without relying on a garbage collector.

Python hides much of this management behind its runtime, reference counting,
garbage collection, and context managers. Rust exposes the relevant
responsibilities to the type system and checks them during compilation. The
difficulty is not invented by Rust; much of it already exists in systems
programming, and Rust makes it explicit and checkable.

## Current assessment

The `memory-allocation-deallocation` experiment has achieved its intended
learning goal. The following points are sufficiently understood to proceed:

- Inner-scope cleanup and the observed output order
- The distinction between custom drop behavior and heap deallocation
- The role of `Box` as an owning pointer
- The distinction between scope, a binding, and ownership
- The safety motivation for deterministic cleanup

The next areas that need hands-on observation are:

- Moving ownership between bindings
- The difference between a move and a copy
- Passing ownership into a function
- Later, borrowing without transferring ownership

## Recommended next experiment

### Goal

Observe that assigning a `String` to another binding moves ownership rather
than duplicating the owned heap allocation.

### Expected observations

Start from a small example like:

```rust
let first = String::from("hello");
let second = first;

println!("{second}");
println!("{first}");
```

The final line should fail to compile even though it remains textually inside
`first`'s scope. Removing that line should allow the program to compile and
show that `second` is the current owner.

### Why it is useful

This isolates the difference between scope and ownership. It also prepares for
borrowing, which can then be introduced as a way to access a value without
moving its ownership.

The recommended sequence is:

```text
scope and automatic drop (complete)
             |
             v
ownership move
             |
             v
shared borrowing with &T
             |
             v
mutable borrowing with &mut T
             |
             v
explicit lifetime relationships
```

## Verification

This session changed documentation only. No Rust source code was changed, so
no Cargo validation was needed.

## Suggested next-session prompt

```text
Please read AGENTS.md and the latest retrospective. Help me create one small
experiment that observes ownership moving from one String binding to another.
Start by explaining the goal, expected observations, and why the experiment is
useful before writing code.
```
