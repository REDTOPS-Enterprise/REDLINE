# 🟥 REDLINE v0.6.1 Documentation

REDLINE is a high-performance, transpiled systems language designed to be as readable as Python but as fast as C++.

## 1. Variables & Constants

REDLINE distinguishes between data that changes and data that stays the same. This allows the compiler to optimize memory.

| Keyword | Meaning | C++ Translation |
|---|---|---|
| `val` | Immutable (Constant). Cannot be changed once set. | `const` |
| `var` | Mutable (Variable). Can be updated later. | (Standard variable) |

Syntax:
```redline
val name: string = "Ace"
var health: int = 100
var pi: float = 3.14
var is_active: bool = true
```

## 2. Data Types

REDLINE is strictly typed, meaning the compiler ensures you don't accidentally treat a number like a word.

*   `int`: Whole numbers (e.g., `10`, `-5`).
*   `float`: Decimal numbers (e.g., `10.5`, `3.14`).
*   `string`: Text wrapped in double quotes (e.g., `"Redline"`).
*   `bool`: Logical values (`true` or `false`).
*   `list[T]`: A dynamic array of elements of type `T`.

## 3. Functions

Functions in REDLINE use a modern "Arrow" syntax to show what they return.

Syntax:
```redline
def name(param: type) -> return_type:
    # Logic here
    return value
```

## 4. Control Flow (Decision Making)

REDLINE uses `if` and `else` for logic. It uses C-style comparison operators but Python-style structure.

Comparison Operators: `==`, `!=`, `>`, `<`, `>=`, `<=`

## 5. Loops

REDLINE supports `while` and `for` loops for repeating actions.

### While Loops
```redline
var i: int = 0
while i < 5:
    print(i)
    i = i + 1
```

### For Loops
```redline
for i in 0..5:
    print(i)
```

## 6. Lists

REDLINE has a built-in `list` type, which is a dynamic array.

### Declaration
```redline
var my_list: list[int] = [10, 20, 30]
```

### Indexing
Access and assign elements using square brackets.
```redline
val first_element: int = my_list[0]
my_list[1] = 99
```

### Built-in Functions
*   `len(list)`: Returns the number of elements in the list.
*   `append(list, value)`: Adds a new element to the end of the list.

Example:
```redline
var numbers: list[int] = []
append(numbers, 1)
append(numbers, 2)
print(len(numbers)) # Prints 2
```

## 7. Input & Output

The built-in `print` and `input` commands handle communication with the console.

## 8. Type Conversion

REDLINE provides built-in functions to convert values between different types.

*   `to_int(value)`
*   `to_float(value)`
*   `to_string(value)`

## 9. Examples

### Fibonacci Sequence
```redline
def fib(n: int) -> int:
    if n <= 1:
        return n
    else:
        return fib(n - 1) + fib(n - 2)
```

### Guessing Game
```redline
val secret_number: int = 42
var running: bool = true
while running:
    val guess_str: string = input("Take a guess: ")
    if guess_str == "quit":
        running = false
    else:
        val guess_num: int = to_int(guess_str)
        if guess_num < secret_number:
            print("Too low!")
        else:
            if guess_num > secret_number:
                print("Too high!")
            else:
                print("You got it!")
                running = false
```

## 10. The Compiler Pipeline

1.  **.rl File**: You write your logic here.
2.  **Lexer (`lexer.rs`)**: Breaks your code into "tokens".
3.  **Parser (`parser.rs`)**: Builds a structured representation (AST).
4.  **Code Generator (`codegen.rs`)**: Translates the AST into C++ code.
5.  **G++ Compiler**: Turns the C++ into a runnable executable.

## 11. Standard Library

### rl_io.hpp
- `print()`: Print values to stdout.
- `input()`: Read a string from stdin.

### rl_math.hpp
- Common math functions (`sqrt`, `pow`, `sin`, etc.) and constants (`PI`, `E`).

### rl_stdlib.hpp
- `len()`: Returns the size of a list.
- `append()`: Appends an element to a list.

This documentation is a work in progress. If you find any errors or want to improve it, please feel free to open an issue or pull request!
