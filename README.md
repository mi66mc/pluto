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
| <b>Uninitialized Variables</b>| Variables can be declared without initialization |
| <b>Block Scoping</b>          | Variables are scoped to their block              |
| <b>Arithmetic Expressions</b> | Supports `+`, `-`, `*`, `/`, `%`, `++`, `--`, `+=`, `-=`, `*=`, `/=` |
| <b>Floating Point Support</b> | Both integers and floats supported               |
| <b>String Support</b>         | String literals and concatenation                |
| <b>String Methods</b>         | Methods: `len()`, `to_upper()`, `to_lower()`, `char_at(i)`, `to_int()`, `to_float()` |
| <b>Boolean Support</b>        | `true` and `false` literals, type detection      |
| <b>Boolean Logic</b>          | Supports `&&`, `||`, `!`                         |
| <b>Type Detection</b>         | `type(x)` returns the type as a string           |
| <b>Function Calls</b>         | Built-in and user-defined functions              |
| <b>Anonymous Functions</b>    | Lambda/arrow functions with `->` syntax          |
| <b>Method Calls</b>           | Call methods on strings, numbers, arrays, etc.   |
| <b>Member Access</b>          | Access module members (e.g. `Math.pi`)           |
| <b>If Statements</b>          | Conditional execution with `if` and `else`       |
| <b>While Loops</b>            | `while` loops with condition in parentheses      |
| <b>For Loops</b>              | C-style `for` loops with init, condition, increment |
| <b>Arrays</b>                 | Array literals, indexing, assignment, methods    |
| <b>Array Methods</b>          | Methods: `push`, `pop`, `remove`, `sum`, `len`, `map` |
| <b>HashMap Support</b>        | HashMap literals, methods: `set`, `get`, `len`; access by `hashmap["key"]` |
| <b>Const Declarations</b>     | `const` keyword for constants                   |
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

## Language Syntax

| Syntax                | Example                        | Description                     |
|-----------------------|--------------------------------|---------------------------------|
| Variable Declaration  | `let x = 10;`                  | Declare variable `x`            |
| Uninitialized Variable| `let n;`                       | Declare without initialization  |
| Block Scope           | `{ let y = 5; print(y); }`     | Variables are scoped to blocks  |
| Arithmetic            | `let y = x * 2 + 3;`           | Expressions with `+ - * / %`    |
| Increment/Decrement   | `x++; x--;`                    | Postfix increment/decrement     |
| Assignment Operators  | `x += 1; x -= 2; x *= 3; x /= 4;` | Compound assignment operators |
| String Literal        | `let s = "hello";`             | String assignment               |
| String Concatenation  | `let t = s + " world";`        | Concatenate strings             |
| String Methods        | `"abc".len(); "abc".to_upper(); "abc".char_at(1);` | String methods                  |
| Boolean Literal       | `let b = true;`                | Boolean assignment              |
| Boolean Logic         | `let b = true && false;`       | Boolean logic                   |
| Type Detection        | `type(x)`                      | Returns type as string          |
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
| Array Methods         | `arr.push(4); arr.pop(); arr.remove(1); arr.sum(); arr.len();` | Array methods |
| HashMap Literal       | `let map = {"a": 1, "b": 2};`  | HashMap assignment              |
| HashMap Methods       | `map.set("c", 3); map.get("b"); map.len();` | HashMap methods         |
| HashMap Indexing      | `map["a"]`                     | Access value by key             |
| For Loop              | `for (let i = 0; i < 5; i++) { print(i); }` | C-style for loop               |
| Const Declaration     | `const PI = 3.14;`             | Constant value                  |
| Comments              | `/* ... */`                    | Block comments                  |

---

## Comprehensive Feature Example

```pluto
let x = 42;
let y = 3.14;
let s = "hello";
let b = true;
let arr = [1, 2, 3];
let map = {"a": 1, "b": 2};
let n; /* uninitialized */

print("x:", x, "y:", y, "s:", s, "b:", b, "arr:", arr, "map:", map, "n:", n);

x++;
x--;
x += 10;
x -= 2;
x *= 2;
x /= 5;
print("x after ops:", x);

y += 2.86;
y *= 2;
print("y after ops:", y);

let z = x + y * 2 - 1 / 2 % 2;
print("z:", z);

let t = s + " world";
print("t:", t);
print("t.len():", t.len());
print("t.to_upper():", t.to_upper());
print("t.to_lower():", t.to_lower());
print("\"abcde\".char_at(2):", "abcde".char_at(2));
print("\"123\".to_int():", "123".to_int());
print("\"3.14\".to_float():", "3.14".to_float());

let bool1 = true && false;
let bool2 = true || false;
let bool3 = !false;
print("bool1:", bool1, "bool2:", bool2, "bool3:", bool3);

print("type(x):", type(x));
print("type(y):", type(y));
print("type(s):", type(s));
print("type(arr):", type(arr));
print("type(map):", type(map));
print("type(b):", type(b));

let arr2 = arr.push(4);
print("arr2 (push):", arr2);
let arr3 = arr2.pop();
print("arr3 (pop):", arr3);
let arr4 = arr3.remove(1);
print("arr4 (remove):", arr4);
print("arr4.sum():", arr4.sum());
print("arr4.len():", arr4.len());
print("arr4[0]:", arr4[0]);

let map2 = map.set("c", 3);
print("map2:", map2);
print("map2.len():", map2.len());
print("map2.get(\"b\"):", map2.get("b"));

if x > 40 {
    print("x is greater than 40");
} else {
    print("x is not greater than 40");
}

let i = 0;
while (i < 3) {
    print("while i:", i);
    i++;
}

for (let j = 0; j < 3; j++) {
    print("for j:", j);
}

fn add(a, b) {
    return a + b;
}
print("add(2, 3):", add(2, 3));

fn no_return(a) {
    a * 2;
}
print("no_return(5):", no_return(5)); /* Should print 10 */

let square = (x) -> x * x;
print("square(4):", square(4));

let inc = (x) -> { return x + 1; };
print("inc(7):", inc(7));

const PI = 3.14159;
print("PI:", PI);
const double = (x) -> x * 2;
print("double(6):", double(6));

let nums = [1, 2, 3, 4];
let squares = nums.map(square);
print("squares:", squares);

print("Math.pi:", Math.pi);
print("Math.pow(2, 8):", Math.pow(2, 8));
print("Time.now():", Time.now());

/*
  This is a block comment.
  It should be ignored by the interpreter.
*/

print("All features tested!");
```

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

### HashMap Methods

| Method           | Description                        | Example                        |
|------------------|------------------------------------|--------------------------------|
| set(key, val)    | Returns new map with key set       | `map.set("c", 3)`              |
| get(key)         | Gets value for key                 | `map.get("b")`                 |
| len()            | Returns number of keys             | `map.len()`                    |
| Indexing         | Access value by key                | `map["a"]`                     |

---

## Type Detection

You can check the type of any value using the `type` built-in function:

```pluto
let x = 5.5;
print(type(x)); // Output: Float
let b = true;
print(type(b)); // Output: Bool
let s = "hello";
print(type(s)); // Output: String
let arr = [1,2,3];
print(type(arr)); // Output: Array
let map = {"a": 1};
print(type(map)); // Output: HashMap
```

---

## Boolean Logic

Pluto supports boolean logic with `&&`, `||`, and `!`:

```pluto
let bool1 = true && false;
let bool2 = true || false;
let bool3 = !false;
print(bool1, bool2, bool3); // false true true
```

---

## Method Chaining

You can chain methods and member accesses:

```pluto
print("abc".to_upper().len());
print([1,2,3].push(4).len());
```

---

## Const Declarations

Use `const` to declare constants:

```pluto
const PI = 3.14159;
print(PI);
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
| HashMap | Key-value pairs                    | `let map = {"a": 1, "b": 2};` |
| Null    | Represents no value                 | `let n = null;` |

---

## Getting Started

### Prerequisites

- Rust (edition 2021 or later)

---

## Contributing

Pull requests and suggestions are welcome! Please open an issue or submit a PR if you have ideas for new features, bug fixes, or improvements.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---
