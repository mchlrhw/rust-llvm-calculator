# The calculator using Inkwell and Rust Peg

This is my first llvm project.

This project uses `peg` to parse and `inkwell` to compile.

Using `thiserror` and `anyhow`, this project handles errors effectively.

You can run:

```shell
$ cargo run '5 * 4 + 20 * 4 - 50'
problem is 5 * 4 + 20 * 4 - 50
The answer is 50
```

## Dependencies

-   llvm 13.0.x
