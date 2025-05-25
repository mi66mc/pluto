# Pluto

**Pluto** is an interpreted programming language implemented in Rust. It features a simple syntax, variable declarations, arithmetic expressions, string, float, and boolean support, function calls, and more. Pluto is designed for learning and experimentation.

---

## Features

| Feature                | Description                                      |
|------------------------|--------------------------------------------------|
| Variables              | `let` keyword for variable declaration           |
| Arithmetic Expressions | Supports `+`, `-`, `*`, `/`                      |
| Floating Point Support | Both integers and floats supported               |
| String Support         | String literals and concatenation                |
| Boolean Support        | `true` and `false` literals, type detection      |
| Function Calls         | Built-in functions like `print`, `type`          |
| Simple Syntax          | Easy to read and write                           |
| Extensible             | Add your own built-in functions in Rust          |

---

## Example

Given the following Pluto code in `test.po`:

```pluto
let x = 5.5;
print(x + 5);
print(x);
let a = "Result: ";
print(a);
let a = a + x;
print(a);
let abc = true;
let def = false;
print(abc, def, type(abc));
```

**Output:**
```
10.5
5.5
Result: 
Result: 5.5
true false Bool
```

---

## Types

Pluto supports the following value types:

| Type    | Description                        | Example         |
|---------|------------------------------------|-----------------|
| Number  | 64-bit signed integer              | `let x = 42;`   |
| Float   | 64-bit floating point number       | `let y = 3.14;` |
| String  | UTF-8 string                       | `let s = "hi";` |
| Bool    | Boolean (`true` or `false`)        | `let b = true;` |
| Module  | Built-in module (e.g. `Math`)      | `Math.pi`       |
| Function| Built-in function                  | `print(x);`     |

You can check the type of any value using the `type` built-in function:

```pluto
let x = 5.5;
print(type(x)); // Output: Float
let b = true;
print(type(b)); // Output: Bool
let s = "hello";
print(type(s)); // Output: String
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
| String Literal        | `let s = "hello";`             | String assignment               |
| String Concatenation  | `let t = s + " world";`        | Concatenate strings             |
| Boolean Literal       | `let b = true;`                | Boolean assignment              |
| Function Call         | `print(y);`                    | Call built-in function          |
| Float Support         | `let z = 3.14;`                | Floating point numbers          |

---

## Built-in Functions

| Function | Description                        | Example                |
|----------|------------------------------------|------------------------|
| print    | Prints arguments to stdout         | `print(x);`            |
| type     | Returns the type of the argument   | `print(type(x));`      |

---

## Contributing

Pull requests and suggestions are welcome!

---

## License

See [LICENSE](LICENSE).

---
