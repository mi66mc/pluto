# Pluto

<p align="center">
  <img src="./public/logos/Pluto_DARK TEXT BACKGROUND.png" alt="Pluto Logo" width="120" style="background: #fff; border-radius: 16px; box-shadow: 0 2px 8px #0002; background-color: #fff;"/>
</p>

<h1 align="center" style="color:#222; font-size:2.8em; font-weight:800; letter-spacing:-2px; margin-bottom:0.2em;">
  Pluto <span style="font-size:0.6em; font-weight:400; color:#888;">Programming Language</span>
</h1>
<p align="center" style="color:#444; font-size:1.2em;">
  <b>Pluto</b> is a modern interpreted programming language implemented in Rust.<br>
  Designed for learning, experimentation, and rapid prototyping.<br>
  <i>Simple syntax. Dynamic typing. Extensible. Fun.</i>
</p>

---

## Installation
To install Pluto, you need to have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).
Once Rust is installed, you can clone the repository and build Pluto:

```sh
git clone https://github.com/mi66mc/pluto
cd pluto
cargo install --path .
```

or use [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install Pluto directly:

```sh
cargo install pluto-lang
```

it is executed by the command `pluto-lang` in your terminal.

### Run

To run Pluto, you can use the following command:

```sh
pluto-lang
```

or

```sh
pluto-lang <filename>
```

Where `<filename>` is the path to a Pluto source file you want to execute.

---

## Features

| Feature                | Description                                      |
|------------------------|--------------------------------------------------|
| <b>Variables</b>              | `let` keyword for variable declaration           |
| <b>Block Scoping</b>          | Variables are scoped to their block              |
| <b>Arithmetic Expressions</b> | Supports `+`, `-`, `*`, `/`, `%`, `++`, `--`, `+=`, `-=`, `*=`, `/=` |
| <b>Floating Point Support</b> | Both integers and floats supported               |
| <b>String Support</b>         | String literals and concatenation                |
| <b>Boolean Support</b>        | `true` and `false` literals, type detection      |
| <b>Function Calls</b>         | Built-in and user-defined functions              |
| <b>Anonymous Functions</b>    | Lambda/arrow functions with `->` syntax          |
| <b>Method Calls</b>           | Call methods on strings, numbers, arrays, etc.   |
| <b>Member Access</b>          | Access module members (e.g. `Math.pi`)           |
| <b>If Statements</b>          | Conditional execution with `if` and `else`       |
| <b>While Loops</b>            | `while` loops with condition in parentheses      |
| <b>For Loops</b>              | C-style `for` loops with init, condition, increment |
| <b>Arrays</b>                 | Array literals, indexing, assignment, methods    |
| <b>Comments</b>               | Block comments using `/* ... */`                 |
| <b>Extensible</b>             | Add your own built-in functions in Rust          |
| <b>Simple Syntax</b>          | Easy to read and write                           |
| <b>REPL Mode</b>              | Interactive Read-Eval-Print Loop                 |

---

## REPL Mode

Pluto supports an interactive REPL (Read-Eval-Print Loop) mode. Simply run Pluto without any arguments:

```sh
cargo run
```

or

```sh
pluto-lang
```

You will see a prompt (`>`) where you can type Pluto code and see results immediately.

## Anonymous Functions

Pluto supports anonymous (lambda/arrow) functions using the `->` syntax. You can assign them to variables and pass them as arguments.

### Syntax

```pluto
let f = (x) -> x * 2;
print(f(10)); // 20

let square = (x) -> { x * x; };
print(square(5)); // 25
```

Anonymous functions can be used with array methods like `map`:

```pluto
let arr = [1, 2, 3];
let result = arr.map((x) -> x + 1);
print(result); // [2, 3, 4]
```

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

if !(5 < 2) {
    print("a");
} else {
    print("b");
}

print("abcde".char_at(1));

let first = input("first num: ");
let second = input("second num: ");
let sum = first.to_int() + second.to_float();
print(sum);
print(type(sum));
if sum > 10 {
    print("Grather than 10!");
} else {
    print("Less than 10!");
}

// User-defined function example
fn sum(a, b) {
    return a + b;
}
print(sum(3, 4));

x++;
print(x);
x += 2;
print(x);
x--;
print(x);
x *= 2;
print(x);
```

**Sample Output:**
```
10.5
5.5
Result: 
Result: 5.5
true false Bool
a
b
b
c
[first num prompt...]
[second num prompt...]
[sum result...]
[sum type...]
Grather than 10! or Less than 10!
7
6.5
8.5
7.5
15
```

---

## Comments

Pluto supports block comments using the `/* ... */` syntax. Everything between `/*` and `*/` will be ignored by the interpreter.

```pluto
let x = 5;
/*
  This is a block comment.
  It can span multiple lines.
*/
let y = x + 1; /* Inline block comment */
```

---

## While Loops

Pluto supports `while` loops with the condition enclosed in parentheses, similar to C-like languages:

```pluto
let i = 0;
while (i < 5) {
    print(i);
    i = i + 1;
}
```

---

## For Loops

Pluto now supports C-style `for` loops, with initialization, condition, and increment expressions, just like in C, C++, or JavaScript.

### Syntax

```pluto
for (let i = 0; i < 5; i = i + 1) {
    print(i);
}
```

- The initialization, condition, and increment are separated by semicolons and enclosed in parentheses.
- The loop body can be a block (`{ ... }`) or a single statement.

### Example

```pluto
for (let i = 0; i < 10; i = i + 2) {
    print(format("i = {}", i));
}
```

This will print:

```
i = 0
i = 2
i = 4
i = 6
i = 8
```

---

## Functions

Pluto supports user-defined functions using the `fn` keyword. Functions can take parameters and return values using `return`.

### Function Declaration

```pluto
fn greet(name) {
    print("Hello, " + name + "!");
}
greet("Pluto");
```

### Function with Return Value

```pluto
fn add(a, b) {
    return a + b;
}
let result = add(10, 20);
print(result); // 30
```

---

## Value Types

Pluto supports the following value types:

| Type    | Description                        | Example         |
|---------|------------------------------------|-----------------|
| Number  | 64-bit signed integer              | `let x = 42;`   |
| Float   | 64-bit floating point number       | `let y = 3.14;` |
| String  | UTF-8 string                       | `let s = "hi";` |
| Bool    | Boolean (`true` or `false`)        | `let b = true;` |
| Array   | Ordered collection of values       | `let arr = [1,2,3];` |
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

---

## Language Syntax

| Syntax                | Example                        | Description                     |
|-----------------------|--------------------------------|---------------------------------|
| Variable Declaration  | `let x = 10;`                  | Declare variable `x`            |
| Block Scope           | `{ let y = 5; print(y); }`     | Variables are scoped to blocks  |
| Arithmetic            | `let y = x * 2 + 3;`           | Expressions with `+ - * / %`    |
| Increment/Decrement   | `x++; x--;`                    | Postfix increment/decrement     |
| Assignment Operators  | `x += 1; x -= 2; x *= 3; x /= 4;` | Compound assignment operators |
| String Literal        | `let s = "hello";`             | String assignment               |
| String Concatenation  | `let t = s + " world";`        | Concatenate strings             |
| Boolean Literal       | `let b = true;`                | Boolean assignment              |
| Function Call         | `print(y);`                    | Call built-in or user function  |
| Function Declaration  | `fn add(a, b) { return a + b; }` | Define a function             |
| Return Statement      | `return x + 1;`                | Return from a function          |
| Float Support         | `let z = 3.14;`                | Floating point numbers          |
| If Statement          | `if x > 0 { print(x); }`       | Conditional execution           |
| Member Access         | `Math.pi`                      | Access module member            |
| Method Call           | `"abc".len()`                  | Call method on value            |
| Input                 | `let s = input("prompt: ");`   | Read user input                 |
| Array Literal         | `let arr = [1, 2, 3];`         | Array assignment                |
| Array Indexing        | `arr[0]`                       | Access array element            |
| Array Assignment      | `arr[0] = 42;`                 | Modify array element            |
| For Loop              | `for (let i = 0; i < 5; i = i + 1) { print(i); }` | C-style for loop               |

---

## Built-in Functions

| Function         | Description                                 | Example                        |
|------------------|---------------------------------------------|--------------------------------|
| print            | Prints arguments to stdout                  | `print(x);`                    |
| type             | Returns the type of the argument            | `print(type(x));`              |
| input            | Reads a line from stdin                     | `let s = input();`             |
| format           | Format string with arguments                | `format("Hello, {}", name)`    |
| exit             | Exit the interpreter                        | `exit(0);`                     |

---

## Built-in Modules

### Math

| Member         | Description                                 | Example                        |
|----------------|---------------------------------------------|--------------------------------|
| Math.pi        | Returns the value of π                      | `print(Math.pi);`              |
| Math.pow(a, b) | Raises `a` to the power of `b`              | `print(Math.pow(2, 3));`       |

---

## Built-in Methods by Type

### String Methods

| Method           | Description                        | Example                        |
|------------------|------------------------------------|--------------------------------|
| len()            | Returns string length              | `"abc".len()`                  |
| to_upper()       | Uppercase string                   | `"abc".to_upper()"`            |
| to_lower()       | Lowercase string                   | `"ABC".to_lower()"`            |
| char_at(i)       | Char at index `i` (as string)      | `"abc".char_at(1)`             |
| to_int()         | Convert string to integer          | `"42".to_int()"`               |
| to_float()       | Convert string to float            | `"3.14".to_float()"`           |

### Number Methods

| Method           | Description                        | Example                        |
|------------------|------------------------------------|--------------------------------|
| to_string()      | Convert number to string           | `x.to_string()`                |

### Float Methods

| Method           | Description                        | Example                        |
|------------------|------------------------------------|--------------------------------|
| to_string()      | Convert float to string            | `x.to_string()`                |

### Array Methods

| Method           | Description                        | Example                        |
|------------------|------------------------------------|--------------------------------|
| len()            | Returns array length               | `arr.len()`                    |
| push(val)        | Returns new array with `val` added | `arr.push(5)`                  |
| pop()            | Returns new array with last removed| `arr.pop()`                    |
| remove(i)        | Returns new array with element at index removed| `arr.remove(2)`               |
| sum()            | Returns sum of numeric elements    | `arr.sum()`                    |
| map(f)           | Returns new array with function applied to each element | `arr.map((x) -> x + 1)` |

### Module Methods

Modules can contain members (constants, functions) accessible via dot notation, e.g. `Math.pi`, `Math.pow(a, b)`.

---

## Notes on Scoping

Pluto uses **block scoping** for variables. This means that variables declared with `let` inside a block (`{ ... }`) are only accessible within that block and its inner blocks. Variables declared in an outer scope are accessible in inner scopes, but not vice versa.

Example:

```pluto
let x = 1;
if true {
    let y = 2;
    print(x); // 1
    print(y); // 2
}
print(x); // 1
print(y); // Error: y is not defined
```

---

## Contributing

Pull requests and suggestions are welcome! Please open an issue or submit a PR if you have ideas for new features, bug fixes, or improvements.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---
