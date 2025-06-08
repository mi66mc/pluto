# Pluto Programming Language

<p align="center">
  <img src="./public/logos/Pluto_DARK TEXT BACKGROUND.png" alt="Pluto Logo" width="120" style="background: #fff; border-radius: 16px; box-shadow: 0 2px 8px #0002; background-color: #fff;"/>
</p>

<div align="center">
  <h1 style="color:#222; font-size:2.8em; font-weight:800; letter-spacing:-2px; margin-bottom:0.2em;">
    Pluto
  </h1>
  <p style="color:#444; font-size:1.2em;">
    A modern, expressive programming language implemented in Rust<br>
    <i>Simple syntax. Dynamic typing. Extensible. Fun.</i>
  </p>
</div>

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat&logo=license&logoColor=white)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org)

</div>

---

## 🚀 Quick Start

```bash
# Install via Cargo
cargo install pluto-lang

# Run the REPL
pluto-lang

# Run a script
pluto-lang script.po
```

Your first Pluto program:
```pluto
/* Hello World in Pluto */
print("Hello, World!");

/* Quick example of Pluto's features */
let greet = (name="World") -> {
    ? name == "World" ->
        print("Hello, stranger!")
        : print("Welcome back, " + name + "!");
};

greet();        /* Hello, stranger! */
greet("Alice"); /* Welcome back, Alice! */
```

## 📖 Table of Contents
- [Features](#-features)
- [Installation](#-installation)
- [Language Guide](#-language-guide)
- [Built-in Types](#-built-in-types)
- [Standard Library](#-standard-library)
- [Examples](#-examples)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

<div align="center">
  <table>
    <tr>
      <td width="33%">
        <div align="center">
          <h3>🔄 Dynamic Typing</h3>
          <p>Type inference and flexible typing</p>
        </div>
        <pre><code>let number = 42;
let text = "Hello";
let flag = true;</code></pre>
      </td>
      <td width="33%">
        <div align="center">
          <h3>🎯 First-class Functions</h3>
          <p>Arrow syntax and closures</p>
        </div>
        <pre><code>let greet = (name) -> {
    print("Hello, " + name);
}</code></pre>
      </td>
      <td width="33%">
        <div align="center">
          <h3>🛡️ Immutable by Default</h3>
          <p>Safe and predictable code</p>
        </div>
        <pre><code>let arr = [1, 2, 3];
let new_arr = arr.push(4);</code></pre>
      </td>
    </tr>
  </table>
</div>

### Core Features
- 🔄 Dynamic typing with type inference
- 🎯 First-class functions with anonymous function support
- 🎨 Named arguments and default parameters
- 🛡️ Immutable by default
- 📚 Rich standard library
- 🎲 Modern random number generation (Xoshiro256**)
- 💻 Interactive REPL environment
- 🔄 Iterative programming model

### Key Highlights
```pluto
/* Variables and functions */
let x = 42;
let greet = (name="World") -> {
    print("Hello, " + name + "!");
};

/* Array operations (immutable) */
let numbers = [1, 2, 3];
let doubled = numbers.map((x) -> x * 2);  /* [2, 4, 6] */

/* Random number generation */
let dice = Random.int(1, 6);
let probability = Random.float();
```

## 📥 Installation

### Prerequisites
- Rust (edition 2021 or later)
- Cargo package manager

### Option 1: Install via Cargo
```bash
cargo install pluto-lang
```

### Option 2: Build from Source
```bash
git clone https://github.com/mi66mc/pluto
cd pluto
cargo install --path .
```

## 📚 Language Guide

### Comments
Pluto uses block comments:
```pluto
/* Single line comment */

/* Multi-line
   block comment */

let x = 42; /* Inline comment */
```

### Variables and Types
```pluto
/* Basic types */
let number = 42;          /* Integer */
let float = 3.14;        /* Float */
let text = "Hello";      /* String */
let flag = true;         /* Boolean */
let empty = null;        /* Null */

/* Type conversion */
let str_num = "123".to_int();
let str_float = "3.14".to_float();
let num_str = 42.to_string();
```

### Functions
```pluto
/* Arrow function with default parameters */
let greet = (name="World", greeting="Hello") -> {
    print(greeting + ", " + name + "!");
};

/* Usage */
greet();                          /* "Hello, World!" */
greet("Alice");                   /* "Hello, Alice!" */
greet(greeting="Hi", name="Bob"); /* "Hi, Bob!" */

/* Immediate function invocation */
((x) -> x * x)(5);               /* 25 */
((name) -> print("Hello, " + name))("World");  /* "Hello, World!" */

/* Function return values */
fn square(x) x*x;     /* Returns x*x (expression) */
fn cube(x) {x*x*x;}   /* Returns null (block with no return) */
fn quad(x) {          /* Returns x^4 (explicit return) */
    return x*x*x*x;
}
```

### Data Structures

#### Arrays (Immutable)
```pluto
/* Creating arrays */
let numbers = [1, 2, 3, 4, 5];

/* Array operations (all return new arrays) */
let longer = numbers.push(6);      /* [1, 2, 3, 4, 5, 6] */
let shorter = numbers.pop();       /* [1, 2, 3, 4] */
let filtered = numbers.remove(2);  /* [1, 2, 4, 5] */
let doubled = numbers.map((x) -> x * 2);  /* [2, 4, 6, 8, 10] */

/* Array properties */
print(numbers.len());  /* 5 */
print(numbers.sum());  /* 15 */
```

#### HashMaps
```pluto
/* Creating hashmaps */
let user = {
    "name": "John",
    "age": 30
};

/* HashMap operations */
let updated = user.set("city", "New York");
print(user.get("name"));  /* "John" */
print(user.len());        /* 2 */
```

### Control Flow

#### Conditional Statements
```pluto
if x > 0 {
    print("Positive");
} else if x < 0 {
    print("Negative");
} else {
    print("Zero");
}
```

#### Ternary Operator
```pluto
/* Ternary syntax: ? condition -> then_expr : else_expr */
let status = ? age >= 18 -> "adult" : "minor";

/* Can also be used with function calls */
? score > 100 -> print("High score!") : print("Keep trying!");

/* Nested ternary expressions */
let message = ? is_logged_in -> 
    ? is_admin -> "Welcome, admin!" : "Welcome, user!" 
    : "Please log in";
```

#### Match Expressions

Match expressions provide a powerful pattern matching mechanism in Pluto. They allow you to compare a value against a series of patterns and execute code based on which pattern matches.

##### Basic Syntax
```pluto
match value {
    pattern1 -> expression1,
    pattern2 -> expression2,
    _ -> default_expression
}
```

##### Features

1. **Literal Patterns**
```pluto
let num = 2;
match num {
    1 -> "one",
    2 -> "two",
    _ -> "other"
}
```

2. **Block Expressions with Return**
```pluto
match greeting {
    "hi" -> {
        let msg = "Hi there!";
        return msg;
    },
    _ -> "default greeting"
}
```

3. **Multiple Types**
```pluto
match value {
    42 -> "number 42",
    "42" -> "string 42",
    true -> "boolean true",
    null -> "null value",
    _ -> "something else"
}
```

4. **Expression Matching**
```pluto
let x = 10;
let y = 5;
match x + y {
    15 -> "equals 15",
    _ -> "not 15"
}
```

##### Notes
- The `_` pattern acts as a catch-all default case
- Patterns are checked in order from top to bottom
- Different types can be mixed in the same match expression
- Block expressions can use `return` statements
- Commas are required between arms, but optional before the closing brace
- Type mismatches (like comparing string with number) evaluate to false

#### Loops
```pluto
/* For loop */
for (let i = 0; i < 5; i++) {
    print(i);
}

/* While loop */
let i = 0;
while (i < 5) {
    print(i);
    i++;
}
```

## 🛠️ Built-in Modules

### Random Module
```pluto
/* Random number generation */
let num = Random.int(1, 100);    /* Integer in range [1, 100] */
let prob = Random.float();       /* Float in range [0, 1) */
let coin = Random.bool(0.5);     /* Boolean with 50% probability */
let shuffled = Random.shuffle(arr);  /* Shuffle array */
```

### Math Module
```pluto
/* Mathematical operations */
print(Math.pi);              /* π constant */
print(Math.pow(2, 8));      /* Power function (256) */
```

### Time Module
```pluto
/* Time operations */
print(Time.now());          /* Unix timestamp (seconds) */
print(Time.now_ms());       /* Unix timestamp (milliseconds) */
Time.sleep(1000);          /* Sleep for 1 second */
```

## 📝 Examples

### Monte Carlo Pi Estimation
```pluto
/* Estimate Pi using Monte Carlo method */
let points = 1000000;
let inside = 0;

for (let i = 0; i < points; i++) {
    let x = Random.float();
    let y = Random.float();
    if (x * x + y * y <= 1.0) {
        inside++;
    }
}

let pi = 4.0 * inside / points;
print("Estimated Pi:", pi);
```

### Card Deck Shuffling
```pluto
/* Create a deck of cards */
let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
let ranks = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
let deck = [];

/* Build the deck */
for (let s = 0; s < suits.len(); s++) {
    for (let r = 0; r < ranks.len(); r++) {
        deck = deck.push(ranks[r] + " of " + suits[s]);
    }
}

/* Shuffle and display */
let shuffled_deck = Random.shuffle(deck);
print("First 5 cards:");
for (let i = 0; i < 5; i++) {
    print(shuffled_deck[i]);
}
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 📚 Language Reference

### Built-in Types and Methods

#### String Methods
| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `len()` | none | Number | Returns string length |
| `to_int()` | none | Number | Converts string to integer if possible |
| `to_float()` | none | Float | Converts string to float if possible |
| `to_upper()` | none | String | Converts string to uppercase |
| `to_lower()` | none | String | Converts string to lowercase |
| `char_at(index)` | Number | String | Returns character at index as string |
| `concat(str1, str2, ...)` | String, String... | String | Concatenates strings |
| `is_number()` | none | Boolean | Checks if string is a number |
| `is_float()` | none | Boolean | Checks if string is a float |

#### Number Methods
| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `to_string()` | none | String | Converts number to string |
| `to_float()` | none | Float | Converts number to float |

#### Float Methods
| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `to_string()` | none | String | Converts float to string |
| `to_int()` | none | Number | Converts float to integer |

#### Array Methods
| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `len()` | none | Number | Returns array length |
| `push(element)` | Any | Array | Returns new array with element added |
| `pop()` | none | Array | Returns new array with last element removed |
| `remove(index)` | Number | Array | Returns new array with element at index removed |
| `sum()` | none | Float | Returns sum of numeric elements |
| `map(function)` | Function | Array | Returns new array with function applied to each element |

#### HashMap Methods
| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `len()` | none | Number | Returns number of key-value pairs |
| `get(key)` | String | Any | Returns value associated with key |
| `set(key, value)` | String, Any | HashMap | Returns new hashmap with key-value pair added/updated |

### Built-in Modules

#### Math Module
| Function/Constant | Parameters | Returns | Description |
|------------------|------------|---------|-------------|
| `Math.pi` | none | Float | Mathematical constant π |
| `Math.pow(base, exp)` | Number/Float, Number/Float | Float | Returns base raised to exp power |
| `Math.sqrt(value)` | Number/Float | Number/Float | Returns the square root of value |

#### Time Module
| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `Time.now()` | none | Number | Current Unix timestamp in seconds |
| `Time.now_ms()` | none | Number | Current Unix timestamp in milliseconds |
| `Time.sleep(ms)` | Number | Number | Pauses execution for specified milliseconds |

#### Random Module
| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `Random.int(min, max)` | Number, Number | Number | Random integer in range [min, max], defaults to [0, 100] |
| `Random.float()` | none | Float | Random float in range [0, 1) |
| `Random.bool(probability)` | Float/Number | Boolean | Random boolean with given probability (default 0.5) |
| `Random.choice(array)` | Array | Any | Random element from array |
| `Random.shuffle(array)` | Array | Array | Returns new shuffled array |

### Global Functions
| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `print(...args, end="\n")` | Any..., String | null | Prints arguments with space separator. Optional 'end' parameter specifies ending (default "\n") |
| `print_raw(...args, end="\n")` | Any..., String | null | Prints arguments with space separator. Optional 'end' parameter specifies ending (default "\n") |
| `type(value)` | Any | String | Returns type name of value |
| `input(prompt?)` | String? | String | Reads line from stdin with optional prompt |
| `exit(code?)` | Number? | never | Exits program with optional code (default 0) |
| `format(template, ...args)` | String, Any... | String | Formats string replacing {} with arguments |

### Operators
| Operator | Types | Description |
|----------|-------|-------------|
| `+` | Number/Float/String | Addition or concatenation |
| `-` | Number/Float | Subtraction |
| `*` | Number/Float | Multiplication |
| `/` | Number/Float | Division |
| `%` | Number/Float | Modulo |
| `==` | Any | Equality comparison |
| `!=` | Any | Inequality comparison |
| `<` | Number/Float | Less than |
| `>` | Number/Float | Greater than |
| `<=` | Number/Float | Less than or equal |
| `>=` | Number/Float | Greater than or equal |
| `++` | Number/Float | Postfix increment |
| `--` | Number/Float | Postfix decrement |
| `&&` | Boolean | Logical AND |
| `||` | Boolean | Logical OR |
| `!` | Boolean | Logical NOT |

### Control Flow
```pluto
/* Variable declaration */
let name = value;

/* If statement */
if (condition) {
    /* code */
} else if (another_condition) {
    /* code */
} else {
    /* code */
}

/* While loop */
while (condition) {
    /* code */
}

/* For loop */
for (let i = 0; i < 10; i++) {
    /* code */
}

/* Break and continue */
break;
continue;
```

Note: All data structures in Pluto are immutable by default. Operations that appear to modify data structures actually return new copies with the modifications applied.

---

### REPL Environment
Pluto comes with an interactive REPL (Read-Eval-Print Loop) environment:

```bash
$ pluto-lang

=== Pluto Programming Language REPL ===
Type :help for available commands
Type :exit to quit

>> let x = 42;
42
>> x + 8;
50
>> fn square(x) x*x;
null
>> square(5);
25
>> ((x) -> x*x)(6);
36
>> (x) -> x*x;
<function: params=[x], body=BinaryExpression(Identifier("x"), "*", Identifier("x")), env_size=1 >
>> :help

Available Commands:
  :help, :h     - Show this help message
  :clear, :c    - Clear the screen
  :exit, :q     - Exit the REPL
  :reset        - Reset the environment
```

<div align="center">
  <p>
    Working on it... <br>
    <a href="https://github.com/mi66mc/pluto">GitHub</a> |
    <a href="https://github.com/mi66mc/pluto/issues">Report Bug</a> |
    <a href="https://github.com/mi66mc/pluto/issues">Request Feature</a>
  </p>
</div>