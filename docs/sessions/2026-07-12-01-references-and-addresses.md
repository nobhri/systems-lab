# References and addresses

## Date

2026-07-12

## Summary

Read the stack-vs-heap experiment output and connected the three printed
addresses to the objects they identify: the directly stored local `u64`, the
local `Box<u64>` owner, and the heap-allocated `u64`. Extended the experiment
to compare printing a value with printing the address held by a reference.

## Understanding reached

- A variable name is a source-level label and is separate from the value and
  its runtime storage location.
- Both stack- and heap-resident values have addresses. The important difference
  in this experiment is direct storage versus access through indirection.
- `Box<T>` is a stack-resident owner that points to and manages a `T` stored in
  a heap allocation.
- A pointer stores an address used to reach data elsewhere. `Box<T>` is a smart
  pointer because it also owns the allocation and arranges for its cleanup.
- `&value` creates a shared reference, `*reference` accesses its referenced
  value, `{}` formats a value, and `{:p}` formats an address in pointer form.
- The printed addresses are virtual addresses for the current process, not
  permanent physical-RAM identifiers. Exact values can change between runs.

## Experiment update

The program now names shared references to both the directly stored value and
the boxed value. It prints values through those references before printing the
addresses, making this relationship explicit:

```text
value --&--> shared reference --*--> value
```

## Checks for the next run

1. Predict whether `stack_value` and `*stack_reference` will print the same
   number.
2. Predict what `*boxed_reference` will print.
3. Identify which address belongs to the `Box` owner and which belongs to its
   heap allocation.
4. Repeat the run and distinguish changing exact addresses from the stable
   stack-versus-heap relationship being observed.

## Follow-up

- Practice reading the types `u64`, `&u64`, and `Box<u64>` separately.
- After the reference and dereference behavior is predictable, use `Vec<u64>`
  to examine a stack-resident pointer, length, and capacity that manage
  heap-resident elements.

## Suggested next-session prompt

```text
Please read AGENTS.md and the latest retrospective, then help me predict and
run the updated stack-vs-heap experiment. After checking &, *, {}, and {:p},
help me decide whether to start a focused Vec memory-layout experiment.
```
