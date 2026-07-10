# Stack vs. heap

## Goal

Observe the difference between a value stored directly in a stack frame and a
value stored in heap memory through `Box`.

## Background

A local value such as `u64` can be stored directly in the current function's
stack frame. `Box<T>` has two relevant pieces: a small owner stored as a local
variable and a `T` stored in a separate heap allocation.

```text
stack                               heap
+----------------------+           +----------------------+
| stack_value: u64     |           | boxed value: u64     |
+----------------------+           +----------------------+
| boxed_value: Box<u64>|---------->| 84                   |
+----------------------+           +----------------------+
```

The experiment prints one address for each of these three objects. It uses
references only to observe the addresses; it does not dereference raw pointers
or require `unsafe` code.

## How to run

From the repository root:

```sh
cargo run -p memory-stack-vs-heap
```

Or from this experiment directory:

```sh
cargo run
```

## What to observe

- The stack value and the `Box` handle usually have nearby addresses because
  both are local variables in `main`'s stack frame.
- The boxed value usually has an address in a noticeably different region
  because its bytes are in a heap allocation.
- Run the program more than once. Exact addresses may change because addresses
  are runtime details affected by the operating system and compiler.
- Do not infer a portable rule from which numeric address is larger. Stack
  direction and virtual-memory layout are platform details.

Example output will resemble this, but the addresses will differ:

```text
stack value address:  0x...
Box handle address:   0x...
boxed value address:  0x...
```

## Why it happens

Calling `main` creates a stack frame for its local variables. `stack_value` is
stored directly in that frame. `boxed_value` is also a local variable, but its
role is to own and point to memory requested from the heap allocator. Therefore
printing `&boxed_value` observes the stack-resident owner, while printing
`boxed_value.as_ref()` observes the separately allocated `u64`.

This is why ownership and storage location are related but distinct ideas: a
stack-resident value can own data stored elsewhere.

## Try changing it

Run the program several times and compare the three addresses. Then change the
two integer values and check whether changing the data changes the relationship
between their storage locations.

## Future connections

`Vec` and `String` also keep a small owner on the stack while their elements or
text live in heap allocations. Later experiments can examine their capacity,
reallocation, and memory layout separately.
