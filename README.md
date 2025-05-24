# Pluto

**Pluto** is an interpreted programming language implemented in Rust. It features a simple syntax, variable declarations, arithmetic expressions, function calls, and more. Pluto is designed for learning and experimentation.

---

## Features

| Feature                | Description                                      |
|------------------------|--------------------------------------------------|
| Variables              | `let` keyword for variable declaration           |
| Arithmetic Expressions | Supports `+`, `-`, `*`, `/`                      |
| Floating Point Support | Both integers and floats supported               |
| Function Calls         | Built-in functions like `print`                  |
| Simple Syntax          | Easy to read and write                           |
| Extensible             | Add your own built-in functions in Rust          |

---

## Example

Given the following Pluto code in `test.po`:

```pluto
let x = 5.5;
print(x + 5);
print(x);
```

**Output:**
```
10.5
5.5
```

---

## Getting Started

### Prerequisites

- Rust (edition 2021 or later)

### Build

```sh
cargo build --release
```

### Run

```sh
cargo run test.po
```

Or, after building:

```sh
./target/release/pluto test.po
```

---

## Language Syntax

| Syntax                | Example                        | Description                     |
|-----------------------|--------------------------------|---------------------------------|
| Variable Declaration  | `let x = 10;`                  | Declare variable `x`            |
| Arithmetic            | `let y = x * 2 + 3;`           | Expressions with `+ - * /`      |
| Function Call         | `print(y);`                    | Call built-in function          |
| Float Support         | `let z = 3.14;`                | Floating point numbers          |

---

## Built-in Functions

| Function | Description                | Example         |
|----------|----------------------------|-----------------|
| print    | Prints arguments to stdout | `print(x);`     |

---

## Contributing

Pull requests and suggestions are welcome!

---

## License

See [LICENSE](LICENSE).

---
