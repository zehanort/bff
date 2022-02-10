# bff - A complete Befunge 98 interpreter in Rust

![CI](https://github.com/zehanort/bff/actions/workflows/ci.yml/badge.svg)
![Lines of code](https://img.shields.io/tokei/lines/github/zehanort/bff)
[![codecov](https://codecov.io/gh/zehanort/bff/branch/main/graph/badge.svg?token=IS0D12XPPA)](https://codecov.io/gh/zehanort/bff)
![Crates.io](https://img.shields.io/crates/v/bff)

## What?

`bff` (which stands for _Befunge Forever_) is a toy project, the secondary purpose of which is to interpret Befunge 98 progams.

Its primary purpose is learning Rust.

## Befunge?

**Befunge 93** is an awesome [esoteric](https://en.wikipedia.org/wiki/Esoteric_programming_language) stack-based programming language,
where the program is not a sequence but a _grid_ of instructions, like the following program that reads an integer from stdin and prints its factorial:

```befunge
&>:1-:v v *_$.@
 ^    _$>\:^
```

The program counter / instruction pointer starts from the top left corner and moves around the grid, executing instructions and changing directions,
until it meets the `@` instruction, terminating the execution of the program.

You can read more about Befunge 93 [here](https://github.com/catseye/Befunge-93/blob/master/doc/Befunge-93.markdown).

**Befunge 98** is a Turing-complete extension of Befunge 93. It expands the program grid infinitely (Befunge 93 limits it to 80x25), adds file I/O operations,
support for libraries (which it calls "fingerprints"), and much much more.

You can read more about Befunge 98 [here](https://github.com/catseye/Funge-98/blob/master/doc/funge98.markdown).

## State?

`bff` is under heavy development (in the `dev` branch).

Currently, it is a Befunge 98 interpreter that completely conforms to the [official language specs](https://github.com/catseye/Funge-98/blob/master/doc/funge98.markdown) and successfully passes the [Mycology testsuite](https://github.com/Deewiant/Mycology).

It is planned to introduce additional features, like concurrency and file operations, in subsequent versions of `bff`.

### Implemented Fingerprints

| Name   | ID           | Documentation                                                                                                                                  | Since Version |
| ------ | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `BASE` | `0x42415345` | [link](https://www.rcfunge98.com/rcfunge2_manual.html#BASE)                                                                                    | 1.1.0         |
| `CPLI` | `0x43504C49` | [link](https://www.rcfunge98.com/rcfunge2_manual.html#CPLI)                                                                                    | 1.1.0         |
| `EVAR` | `0x45564152` | [link](https://www.rcfunge98.com/rcfunge2_manual.html#EVAR)                                                                                    | 1.1.0         |
| `MODU` | `0x4d4f4455` | [link 1](https://github.com/catseye/Funge-98/blob/master/library/MODU.markdown), [link 2](https://www.rcfunge98.com/rcfunge2_manual.html#MODU) | 1.1.0         |
| `NULL` | `0x4e554c4c` | [link 1](https://github.com/catseye/Funge-98/blob/master/library/NULL.markdown), [link 2](https://www.rcfunge98.com/rcfunge2_manual.html#NULL) | 1.1.0         |
| `ROMA` | `0x524f4d41` | [link 1](https://github.com/catseye/Funge-98/blob/master/library/ROMA.markdown), [link 2](https://www.rcfunge98.com/rcfunge2_manual.html#ROMA) | 1.1.0         |

**Any comments/remarks/criticism in the form of issues are welcome**.

**Any contributions in the form of pull requests are welcome**, if they are properly documented and accompanied by tests.

## Why?

Because I believe that writing a quirky interpreter is one of the best ways to learn a systems, memory safe, type strict programming language like Rust.

## Can I play with it, too?

To use `bff`, you have 2 options:

1. Download/clone this repository and build it (use the `--recurse-submodules` flag if you want to clone and test against the Mycology testsuite):

```
$ cd bff
$ cargo build --release
```

The `bff` executable will now be under the `./target/release/` directory.

2. Install it with cargo from `crates.io`:

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

A complete list of `bff` arguments can be found by executing `bff -h` or `cargo run -- -h`.

You may also use `bff` as a REPL. Note that each line is followed by an implicit `@` instruction, and that `0` is the exit code that is always returned to the OS:

```
$ cargo run
[...]
bff - Unefunge 98 REPL
version 1.0.0
(type "exit" or "quit" and press <Enter> or press <Ctrl> + C to quit)
> 2a*.
20
> exit
$
```

As a future goal, the REPL will be restricted to the Unefunge 98 instruction set, to avoid unwanted behavior, like the following infinite loop:

```
$ cargo run
[...]
bff - Unefunge 98 REPL
version 1.0.0
(type "exit" or "quit" and press <Enter> or press <Ctrl> + C to quit)
> ^

```
