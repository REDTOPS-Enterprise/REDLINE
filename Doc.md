# 🟥 REDLINE v0.9 Documentation

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
*   `void`: Represents the absence of a value (used for function return types).
*   `list[T]`: A dynamic array of elements of type `T`.

## 3. Functions

Functions in REDLINE use a modern "Arrow" syntax to show what they return.

Syntax:
```redline
def name(param: type) -> return_type:
    # Logic here
    return value
```

If a function does not return a value, the return type can be omitted (defaults to `void`).

```redline
def greet(name: string):
    print("Hello, " + name)
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

## 6. Error Handling

REDLINE uses `try` and `catch` blocks to handle runtime errors.

```redline
try:
    val content: string = read_file("missing.txt")
catch e:
    print("An error occurred!")
```

## 7. Lists

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
*   `len(list)`: Returns the number of elements.
*   `append(list, value)`: Adds an element to the end.
*   `sort(list)`: Sorts the list in-place.
*   `reverse(list)`: Reverses the list in-place.
*   `find(list, value)`: Returns the index of the value, or -1 if not found.

## 8. Classes & Objects

REDLINE supports Object-Oriented Programming (OOP) with classes.

### Defining a Class
Use the `class` keyword to define a new type.

```redline
class Person:
    var name: string = ""
    var age: int = 0

    # Constructor
    def init(n: string, a: int):
        this.name = n
        this.age = a

    # Method
    def greet():
        print("Hello, I am " + this.name)
```

### Using a Class
```redline
var p: Person = Person("Alice", 30)
p.greet()
```

*   **`this`**: Use `this` inside methods to access member variables and other methods.
*   **`init`**: A special method that acts as the constructor.

## 9. Modules

You can split your code into multiple files using modules.

### Importing a Module
Use the `import` keyword to include another `.rl` file.

```redline
import "math_utils.rl"

val result: int = add(10, 5)
```

### Public Visibility
By default, functions and variables are private to the module. Use the `pub` keyword to make them accessible to other modules.

```redline
# math_utils.rl
pub def add(a: int, b: int) -> int:
    return a + b
```

## 10. C++ Interoperability

REDLINE is designed to work seamlessly with C++. You can compile REDLINE code into a library and use it in your C++ projects.

### Building a Library
```bash
python redline.py lib my_library.rl
```
This generates `my_library.hpp` and `my_library.o`.

### Using in C++
```cpp
#include "my_library.hpp"

int main() {
    int result = rl::add(10, 20); // Call REDLINE function
    return 0;
}
```

## 11. Standard Library

### I/O (`rl_io.hpp`)
- `print(value)`: Print to stdout.
- `input(prompt)`: Read a string from stdin.

### File I/O (`rl_file.hpp`)
- `read_file(path) -> string`: Reads the entire content of a file. Throws on error.
- `write_file(path, content) -> bool`: Writes content to a file. Throws on error.

### String Manipulation (`rl_string.hpp`)
- `split(string, delimiter) -> list[string]`: Splits a string into a list.
- `join(list[string], delimiter) -> string`: Joins a list of strings.
- `contains(string, substring) -> bool`: Checks if a string contains a substring.

### Math (`rl_math.hpp`)
- Common math functions (`sqrt`, `pow`, `sin`, etc.) and constants (`PI`, `E`).

### Stdlib (`rl_stdlib.hpp`)
- `len(list)`
- `append(list, value)`
- `sort(list)`
- `reverse(list)`
- `find(list, value)`
- `to_int(value)`
- `to_float(value)`
- `to_string(value)`

This documentation is a work in progress. If you find any errors or want to improve it, please feel free to open an issue or pull request!
