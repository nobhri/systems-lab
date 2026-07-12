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

The experiment prints values both directly and through shared references, then
prints one address for each of these three objects. `&value` creates a shared
reference, while `*reference` accesses the value behind that reference. These
are safe Rust operations and do not require raw pointers or `unsafe` code.

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

- `stack_value` and `*stack_reference` both print `42`. Taking a reference does
  not copy or change the referenced value.
- `*boxed_reference` prints `84`. Dereferencing the shared reference accesses
  the value stored in the heap allocation.
- `stack_reference` printed with `{:p}` gives the address of `stack_value`, and
  `boxed_reference` printed with `{:p}` gives the address of the boxed value.
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
stack value:           42
stack value via ref:   42
boxed value via ref:   84
stack value address:   0x...
Box handle address:    0x...
boxed value address:   0x...
```

## Why it happens

Calling `main` creates a stack frame for its local variables. `stack_value` is
stored directly in that frame. `boxed_value` is also a local variable, but its
role is to own and point to memory requested from the heap allocator. Therefore
printing `&boxed_value` observes the stack-resident owner, while printing
`boxed_value.as_ref()` observes the separately allocated `u64`.

The variables `stack_reference` and `boxed_reference` have types `&u64`. A
shared reference identifies an existing `u64` without taking ownership of it.
Applying `*` follows the reference to access the value, while formatting the
reference with `{:p}` displays the address it contains.

This is why ownership and storage location are related but distinct ideas: a
stack-resident value can own data stored elsewhere.

## Try changing it

Before running the program, predict the answers to these checks:

1. Do `stack_value` and `*stack_reference` print the same value?
2. Does `*boxed_reference` print the value passed to `Box::new`?
3. Which two printed addresses should be near each other, and which address
   should usually be in a different region?

Then run the program several times and compare the results with the prediction.
Change the two integer values and check whether the values change without
changing the relationship between their storage locations.

## Future connections

`Vec` and `String` also keep a small owner on the stack while their elements or
text live in heap allocations. Later experiments can examine their capacity,
reallocation, and memory layout separately.
