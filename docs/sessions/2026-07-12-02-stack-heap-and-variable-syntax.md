# Stack, heap, and variable syntax

## Date

2026-07-12

## Summary

Ran the updated stack-vs-heap experiment and checked both values and addresses.
The results confirmed that references reach the expected values, while the
local `u64` and the `Box<u64>` owner are stored near each other on the stack and
the boxed `u64` is stored in a separate heap allocation. Also reviewed the Rust
syntax used to declare, initialize, and update variables.

## Observed output

```text
stack value:           42
stack value via ref:   42
boxed value via ref:   84
stack value address:   0x16d3da790
Box handle address:    0x16d3da798
boxed value address:   0x1032d5a50
```

The exact addresses are specific to this run. The useful observation is their
relationship: the stack value and `Box` owner have nearby addresses, while the
boxed value has a clearly different address.

```text
stack                                  heap

stack_value: 42
stack_reference --------> stack_value
boxed_value (Box owner) --------------> 84
```

## Understanding reached

- Reading through a reference does not change the value: `stack_value` and
  `*stack_reference` both produced `42`.
- A reference to the boxed value reached the heap-resident `u64` and produced
  `84`.
- `&value` creates a shared reference and `*reference` accesses the referenced
  value.
- `Box<T>` owns heap-allocated data, while the local `Box<T>` handle itself is
  stored on the stack in this experiment.
- `42_u64` is an integer literal with the `u64` suffix. The underscore is an
  optional separator, so `42u64` means the same thing.
- `let value = 42` uses type inference, while `let value: u64 = 42` states the
  variable's type explicitly.
- `let value: u64; value = 42;` separates declaration from initialization. It
  does not permit a second assignment unless the binding is declared `mut`.
- `let mut value = 42` allows reassignment without changing the variable's
  type.
- Repeating `let` with the same name is shadowing: it creates a new binding and
  can therefore introduce a different type.
- `let` is an English keyword meaning roughly “let this name be this value,”
  not an abbreviation. `const` abbreviates “constant,” and `mut` abbreviates
  “mutable.”

## Useful distinctions

```rust
let inferred = 42;              // inferred as i32 in this context
let suffixed = 42_u64;          // the literal specifies u64
let annotated: u64 = 42;        // the binding specifies u64

let initialized_later: u64;
initialized_later = 42;         // first and only initialization

let mut reassigned = 42;
reassigned = 84;                // same binding and same type

let shadowed = 42;
let shadowed = shadowed.to_string(); // new binding and new type

const ANSWER: u64 = 42;         // named compile-time constant
```

## Experiment status

The focused stack-vs-heap experiment has covered its intended concept: direct
stack storage, indirection through `Box`, references, dereferencing, value
formatting, and address formatting. The supporting variable syntax needed to
read the program is also understood well enough to move on.

Ownership moves, mutable borrowing, `Drop`, and container layouts such as
`Vec<T>` should remain separate experiments so that each introduces one main
concept.

## Suggested next-session prompt

```text
Please read AGENTS.md and the latest retrospective. Help me choose one focused
next experiment: ownership moves, shared versus mutable borrowing, Box and
Drop, or Vec memory layout. Explain the goal and expected observations before
writing code.
```
