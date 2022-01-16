# bff - A Befunge 93 (and soon 98) interpreter in Rust

![CI](https://github.com/zehanort/bff/actions/workflows/ci.yml/badge.svg)
[![codecov](https://codecov.io/gh/zehanort/bff/branch/main/graph/badge.svg?token=IS0D12XPPA)](https://codecov.io/gh/zehanort/bff)

## What?

`bff` (which stands for *Befunge Forever*) is a toy project of mine, the secondary purpose of which is to interpret Befunge 93 (and soon 98) progams.

Its primary purpose is learning Rust.

## Befunge 93?

Befunge 93 is an awesome [esoteric](https://en.wikipedia.org/wiki/Esoteric_programming_language) stack-based programming language,
where the program is not a sequence but a *grid* of instructions, like the following program that reads an integer from stdin and prints its factorial:

```befunge
&>:1-:v v *_$.@
 ^    _$>\:^
```

The program counter / instruction pointer starts from the top left corner and moves around the grid, executing instructions and changing directions,
until it meets the `@` instruction, terminating the execution of the program.

You can read more about Befunge 93 [here](https://github.com/catseye/Befunge-93/blob/master/doc/Befunge-93.markdown).

## Befunge 98?

Befunge 98 is a turing-complete extension of Befunge 93. Tt expands the program grid infinitely (Befunge 93 limits it to 80x25), adds file I/O operations,
support for libraries (which it calls "fingerprints"), and much much more.

You can read more about Befunge 98 [here](https://github.com/catseye/Funge-98/blob/master/doc/funge98.markdown).

## State?

`bff` is under heavy development. It considers itself to be a working Befunge 93 interpreter, though
(thanks to the [Mycology testsuite](https://github.com/Deewiant/Mycology) for some of the tests).

Future goals are:
1. Proper error handling instead of `?` and `unwrap`ing everywhere.
2. Expand `bff` to be a fully fledged Befunge 98 interpreter, backwards compatible with Befunge 93.

**Any comments/remarks/criticism in the form of issues are welcome**.

**Any contributions in the form of pull requests are welcome**, if they are properly documented and accompanied by tests.

## Why?

Because I believe that writing a quirky interpreter is one of the best ways to learn a systems, memory safe, type strict programming language like Rust.

## Can I play with it, too?

To use `bff`, you have 2 options:

### 1. Download/clone this repository and build it:

```
$ cd bff
$ cargo build --release
```

The `bff` executable will now be under the `./target/release/` directory.

### 2. Install it with cargo from `crates.io`:

```
$ cargo install bff
```

## How?

You can use `bff` as an executable, if you installed it via `cargo`:

```
$ cat hello_world.bf
"!dlrow olleH">:#,_@
$ bff hello_world.bf
Hello world!
```

Or, if you downloaded/cloned the repo:

```
$ cargo run -- tests/bf93/hello_world.bf
[...]
Hello world!
```
