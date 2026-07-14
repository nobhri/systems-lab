# Ownership move

## Goal

Observe that assigning a `String` to another binding moves ownership instead of
duplicating the owned value.

## Background

A `String` owns a heap allocation containing its text. Unlike simple types such
as `u32`, `String` does not implement the `Copy` trait. Assigning it to another
binding therefore transfers responsibility for eventually dropping that value.

```text
before assignment:                 after assignment:

first ----owns----> "hello"        first   (cannot be used)
                                  second ----owns----> "hello"
```

The `first` binding remains textually inside its scope after the assignment,
but it is no longer usable as the owner. This separates two ideas: where a
binding is in scope and which binding can access the owned value.

## How to run

From the repository root:

```sh
cargo run -p memory-ownership-move
```

Or from this experiment directory:

```sh
cargo run
```

To observe the compiler rejecting use after a move, temporarily add this line
at the end of `main`:

```rust,compile_fail
println!("previous owner: {first}");
```

Then run:

```sh
cargo check -p memory-ownership-move
```

Remove the line after observing the error so the workspace remains in a
compilable state.

## What to observe

The committed program prints:

```text
current owner: hello
```

When the temporary `println!` uses `first`, compilation fails with the central
diagnostic:

```text
error[E0382]: borrow of moved value: `first`
```

Read the diagnostic from top to bottom and identify these three locations:

1. Where `first` is created as a `String`.
2. Where the value is moved from `first` into `second`.
3. Where the compiler rejects the later use of `first`.

## Why it happens

If both bindings independently owned the same `String`, both would try to drop
the same heap allocation. Rust prevents that by making the old binding unusable
after ownership moves to the new binding.

The assignment does not clear the heap allocation or create a second copy of
the text. It transfers access to the existing `String` value from `first` to
`second`. At the end of `main`, only `second` drops it.

Cloning would be a different operation: `first.clone()` explicitly creates
another independently owned `String`. That distinction is intentionally left
out of the executable experiment so this experiment isolates ownership move.

## Try changing it

Before making each change, predict whether it will compile:

1. Add `println!("before move: {first}");` before `let second = first;`.
2. Add `println!("after move: {first}");` after `let second = first;`.

Run focused `cargo check` commands after each change, then restore the committed
version when finished.

## Future connections

A later experiment can introduce shared borrowing with `&String`, which permits
temporary access without transferring ownership.
