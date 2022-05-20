# Brainfart

Brainfart is an optimizing interpreter written in Rust for the esoteric
programming language [brainfuck](https://en.wikipedia.org/wiki/Brainfuck).

Run `cargo build` to build the binary. The binary will be available at
`target/debug/bft`. If you want to install the binary, you can run `cargo
install --path .`.

You can also view annotated examples to run in the `examples/` directory.

```
$ bft hello.bf
Hello World!
```

Cells are implemented with `u32` numbers, meaning that the value ranges from 0
to a bit over 4 billion. This makes brainfuck algorithms that rely on wrapping
infeasible. Further, the amount of cells available increases as the pointer
moves right; as many cells will be allocated as possible. The cell structure is
not cyclical: the pointer cannot move to the left on the first cell to reach the
last one.

Brainfart further optimizes bf code by merging consecutive instructions and
removing cancelling ones:

```
+++++ => Add(5)
>><   => MoveRight(1)
```

For programs which contain nested loop blocks with repeated instructions, the
optimizations are noticeable. For an example of this program, view
`examples/slow.bf`.
