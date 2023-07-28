## Overview

These project contains of:

* REPL for interpreting math-lang expressions
* *mathc* compiler, which compiles source code to IL
* Stack VM for executing IL code

### How to build
Repl:

```
cargo build --release --features=repl --bin repl
```

VM executable:

```
cargo build --release --features=cli --bin math-vm
```
IL Compiler:

```
cargo build --release --features=cli --bin mathc
```

## Example

Image we have these expression:
```rust
540 - 28 * 14 ^ sin(42) * ctg(11) \ 2 + log(100, 3.5) % 1.2
```

Let's evaluate these expression using REPL:
```
$ target/release/repl 
Calculator prompt. Expressions are line evaluated.
>> 540 - 28 * 14 ^ sin(42) * ctg(11) \ 2 + log(100, 3.5) % 1.2
541.076
>> 
```

Now we will compile these expression to IL code
```console
target/release/mathc -d "540 - 28 * 14 ^ sin(42) * ctg(11) \ 2 + log(100, 3.5) % 1.2" -o IL.mlang
```

**Our instuction set size is about 56 bytes**

And lastly, we get a result executing IL
```
$ target/release/math-vm -p IL.mlang 
Got result "541.076" in 19 executed commands
Exec time: 29.123µs
```