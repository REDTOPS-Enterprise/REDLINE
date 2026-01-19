# The REDLINE Programming Language: High-Speed Science (v1.0)

---

## Introduction: Welcome to the REDLINE Program

Welcome to the official guide for the REDLINE Programming Language. You're here because you're not satisfied with the status quo. You want the simplicity of modern scripting languages and the raw power of compiled, low-level code. You want to have your cake and eat it too. We're here to tell you that you can.

### What is REDLINE?

REDLINE is a high-performance, transpiled systems programming language. Our motto is **"C++ but Simplified."** We've engineered a language that combines the clean, readable, indentation-based syntax of Python with the near-bare-metal speed of C++.

How? We don't try to reinvent sixty years of compiler technology. Instead, REDLINE is a "transpiled" language. Your REDLINE code is converted into highly optimized, human-readable C++17 code, which is then compiled by a standard C++ compiler (like G++) into a native executable. This gives you the best of both worlds: a beautiful, modern language on the frontend, and a battle-tested, performance-obsessed toolchain on the backend.

### Who is this book for?

This book is for any developer who has ever said, "I love how easy Python is, but I wish it were faster," or, "I love how fast C++ is, but I wish it were simpler." Whether you're a student building your first project, a hobbyist making a game, or a professional engineering a high-performance application, REDLINE is your new favorite tool.

### The REDLINE Philosophy: Why Transpilation?

Our philosophy is simple: **focus on results, not on reinventing the wheel.**

1.  **Leverage the Best:** By compiling to C++, we instantly inherit decades of compiler optimization research. We get access to the massive C++ ecosystem of libraries and tools for free.
2.  **Focus on the Developer:** Since we don't have to write a new code optimizer from scratch, we can focus on what matters most: the developer experience. We've poured our resources into creating a clean syntax, a powerful standard library, and intuitive tooling.
3.  **Transparency:** You can always look at the generated C++ code in the `temp_build` directory. There's no magic here—just efficient, high-speed science.

### How to Install REDLINE

Installation is a simple process of cloning the repository and initializing the compiler. Please follow the official instructions in the [README.md](README.md) file.

---

## Chapter 1: The Basics - Your First REDLINE Program

This chapter covers the fundamental building blocks of any REDLINE program.

### "Hello, World!" with `print()`

The traditional first step in any language is to make it talk. In REDLINE, this is a single, clean line.

```redline
print("Hello, World!")
```
This calls the built-in `print` function, which outputs the given string to the console, followed by a newline character.

### Comments: Leaving Notes for Your Future Self

Use the `#` symbol for single-line comments. The compiler will ignore them completely. Good comments explain the *why*, not the *what*. If your code is so complex you have to explain what it does, you probably wrote it wrong.

```redline
# Calculate the velocity based on a fixed acceleration.
val velocity: float = initial_velocity + (acceleration * time) # a = 9.8
```

### Variables and Constants: `var` vs. `val`

REDLINE forces you to be intentional about your data. This is a core feature for writing safe and predictable code.

*   `val` (Value): Creates an **immutable constant**. Once you assign a value to it, it can never be changed. This is the default you should always reach for. It signals to other developers (and the compiler) that this piece of data is a fixed point.
*   `var` (Variable): Creates a **mutable variable**. You can update its value later. Use this only when you know the data *must* change during its lifetime.

```redline
val LAUNCH_CODE: string = "Project-A-113" # A constant. Cannot be changed.
var reactor_temp: int = 900               # A variable that will fluctuate.

reactor_temp = 950                        # This is valid.
# LAUNCH_CODE = "New-Code"                # This would cause a compiler error.
```
Preferring `val` makes your code easier to reason about and helps prevent a whole class of bugs caused by accidental modification of data.

### Core Data Types

REDLINE is a strictly typed language. You must declare the type of your data. This eliminates runtime errors where you might, for example, try to do math on a string.

| REDLINE Type | C++ Equivalent | Description |
|---|---|---|
| `int` | `int` | Whole numbers, positive or negative. |
| `float` | `double` | 64-bit floating-point (decimal) numbers. |
| `string` | `std::string` | A sequence of characters. |
| `bool` | `bool` | Logical values: `true` or `false`. |
| `void` | `void` | Represents the absence of a value. |

---

## Chapter 2: Control Flow - Making Decisions and Looping

This is where your programs stop being a straight line and start adapting to changing conditions.

### If/Else

Use `if` and `else` to execute code based on a condition. The colon (`:`) and indentation are mandatory and define the code blocks.

```redline
val shield_integrity: float = 0.45

if shield_integrity < 0.5:
    print("Warning: Shields are below 50%.")
else:
    print("Shields are holding strong.")
```

### While Loops

A `while` loop runs as long as its condition evaluates to `true`. Make sure something inside the loop eventually causes the condition to become false, or you've just invented an infinite loop.

```redline
var countdown: int = 3
while countdown > 0:
    print(f"T-minus {countdown}...")
    countdown = countdown - 1
print("Liftoff!")
```

### For Loops

A `for` loop is used to iterate over a range of numbers. The range `start..end` includes `start` but is **exclusive** of `end`.

```redline
# This will print numbers 0, 1, 2, 3, 4
for i in 0..5:
    print(f"Processing item {i}")
```

### Loop Control: `break` and `continue`

Sometimes you need to change a loop's behavior mid-stride.
*   `continue`: Immediately stops the current iteration and jumps to the beginning of the next one. Useful for skipping certain items.
*   `break`: Immediately terminates the loop entirely. Useful for stopping once you've found what you're looking for.

```redline
print("Searching for the first test subject with a passing grade...")
val grades: list[int] = [45, 59, 92, 81, 30]
for i in 0..len(grades):
    val grade: int = grades[i]
    if grade < 60:
        continue # Skip failing grades.
    
    print(f"Found a passing grade: {grade} at index {i}. Stopping search.")
    break
```

---

## Chapter 3: Data Structures - Organizing Your Experimental Data

Single variables are fine for simple calculations, but real-world problems require handling data in bulk.

### Lists (`list[T]`)

A `list` is a dynamic, ordered collection of elements of the same type. It can grow and shrink as needed.

```redline
# A list of experiment names
var experiments: list[string] = ["Experiment-A", "Experiment-B"]

# Access elements by index (starts at 0)
val first_exp: string = experiments[0] # "Experiment-A"

# Modify an element
experiments[1] = "Experiment-C" # The list is now ["Experiment-A", "Experiment-C"]

# Add an element to the end
append(experiments, "Experiment-D")
```

### Dictionaries (`dict[K, V]`)

A `dict` is a collection of key-value pairs. It provides an incredibly fast way to look up a value if you know its key. While searching a list takes time proportional to its size (`O(n)`), looking up a value in a dictionary is nearly instantaneous (`O(log n)`).

The "key" must be unique within the dictionary.

```redline
# A dictionary mapping subject IDs to their status
var subject_status: dict[string, string] = {
    "Subject-001": "Stable",
    "Subject-002": "Unstable",
    "Subject-003": "Terminated"
}

# Access a value by its key
val status: string = subject_status["Subject-002"] # "Unstable"

# Add or modify an entry
subject_status["Subject-002"] = "Contained"
subject_status["Subject-004"] = "Active"
```

---

## Chapter 4: Functions - Creating Reusable Apertures

Don't repeat yourself. If you find yourself writing the same block of code more than once, it's time to bundle it into a reusable function.

### Defining Functions

Functions are defined with `def`, followed by the function name, parameters (with their types), and an optional return type using the `->` arrow syntax. If a function doesn't return a value, its return type is `void` and can be omitted.

```redline
# A function that takes two floats and returns their average
def calculate_average(a: float, b: float) -> float:
    return (a + b) / 2.0

# A function that performs an action but returns nothing
def log_message(message: string):
    print(f"[LOG] {message}")
```

### Function Overloading

REDLINE allows you to define multiple functions with the same name, as long as their parameter signatures (the number or types of parameters) are different. This allows you to create a single, intuitive API for operations that can apply to different kinds of data.

```redline
# Serializes an integer to a string
def serialize(data: int) -> string:
    return f"i:{data}"

# Serializes a list of strings to a single, comma-separated string
def serialize(data: list[string]) -> string:
    return join(data, ",")

val int_data: string = serialize(123)          # Calls the first function
val list_data: string = serialize(["a", "b"])  # Calls the second function
```

---

## Chapter 5: Object-Oriented Programming - The REDLINE Way

Classes are blueprints for creating objects. They allow you to bundle data (fields) and the functions that operate on that data (methods) into a single, logical unit.

### Defining a Class

```redline
class Reactor:
    # Fields: data that belongs to each Reactor object
    var temperature: float = 300.0
    val max_temp: float = 5000.0

    # Constructor: a special method named 'init' that sets up a new object
    def init(start_temp: float):
        this.temperature = start_temp

    # Method: a function that belongs to the object
    def check_status():
        if this.temperature > this.max_temp:
            print("CRITICAL: CORE MELTDOWN IMMINENT!")
        else:
            print(f"Core temperature stable at {this.temperature}K.")
```

### Automatic Memory Management with `new`

In many low-level languages, you are responsible for both allocating and freeing memory. This is tedious and a common source of catastrophic bugs. REDLINE handles this for you.

When you want to create a complex object, you use the `new` keyword. This allocates memory for the object on the **heap** (a large pool of memory for long-lived data) and gives you a "smart pointer" to it. This smart pointer automatically keeps track of how many parts of your code are using the object. When the last reference to the object is gone, its memory is automatically and safely deallocated.

```redline
# Create a new Reactor object. This calls the 'init' constructor.
var main_reactor: Reactor = new Reactor(450.0)
main_reactor.check_status()
```

**Safety Warning: Circular References.** Our automatic cleanup system is robust, but it has one weakness: a "deadly embrace." If object A holds a reference to object B, and object B holds a reference back to object A, they will keep each other alive forever, even if the rest of your program can no longer reach them. This creates a memory leak. We're engineering a solution for this, but for now, design your object relationships carefully to avoid these circular dependencies.

---

## Chapter 6: The Standard Library - Your Toolkit for Scientific Breakthroughs

A language is only as good as its tools. We've included a standard library with the essentials you need to get real work done.

### System (`rl_stdlib.hpp`)
*   `args: list[string]`: A global list containing the command-line arguments passed to your program.
*   `len(list) -> int`: Returns the number of elements in a list.
*   `append(list, value)`: Adds an element to the end of a list.
*   `sort(list)` / `reverse(list)` / `find(list, value)`
*   `to_string(value) -> string`, `to_int(string) -> int`, `to_float(string) -> float`

### I/O (`rl_io.hpp`)
*   `print(value)`: Prints a value to the console, followed by a newline.
*   `input(prompt: string) -> string`: Displays a prompt and reads a line of text from the user.

### File System (`rl_file.hpp`)
*   `read_file(path: string) -> string`: Reads an entire file into a single string. Throws an error if the file can't be opened.
*   `write_file(path: string, content: string)`: Writes a string to a file, overwriting it if it exists.
*   `exists(path: string) -> bool`: Returns `true` if a file or directory exists.
*   `mkdir(path: string)`: Creates a new directory.
*   `remove(path: string)`: Deletes a file or an empty directory.
*   `list_dir(path: string) -> list[string]`: Returns a list of filenames in a directory.

### Time (`rl_time.hpp`)
*   `time() -> float`: Returns the current system time as a Unix timestamp (seconds since 1970).
*   `sleep(seconds: float)`: Pauses the program's execution for the given number of seconds.

### Random (`rl_random.hpp`)
*   `random_int(min: int, max: int) -> int`: Returns a random integer between `min` and `max` (inclusive).
*   `random_float() -> float`: Returns a random float between 0.0 and 1.0.

### Strings & F-Strings
For complex string formatting, f-strings are the best tool for the job. Any valid REDLINE expression can be placed inside the curly braces.
```redline
val name: string = "Redline"
print(f"The current time is {time()}.")
print(f"A random number: {random_int(1, 100)}.")
```

---

## Chapter 7: Building & Projects - From Blueprint to Reality

### Single-File Compilation
For simple scripts, you can compile a single file directly.
```bash
python redline.py build my_script.rl
```

### Multi-File Projects with `RedConfig.toml`
For any real project, you'll want a `RedConfig.toml` file. This is the master blueprint for your project. It tells the compiler everything it needs to know.
```toml
[project]
name = "MyAwesomeProject"
version = "1.0.0"
entry_point = "src/main.rl"
output_dir = "bin"
```
With this file in your project's root directory, the build script will automatically use it.
```bash
# Run from your project's root directory
python path/to/redline.py build
```

---

## Chapter 8: The Philosophy - How It All Works

So, how does the magic happen? It's not magic, it's efficient science.

### The Transpilation Pipeline: `RL -> C++ -> Executable`
When you run `redline build`, a three-stage process kicks off:
1.  **Parsing (Rust):** Our high-speed Rust core reads your `.rl` files, validates the syntax, and builds an Abstract Syntax Tree (AST)—an internal representation of your code.
2.  **Code Generation (Rust):** The core then walks the AST and generates clean, human-readable C++17 code.
3.  **Compilation (C++):** Finally, we hand that generated code off to your system's G++ compiler, which creates a highly-optimized, native executable.

### Why C++ is the Target
Simple: performance and ecosystem. C++ is one of the fastest languages on the planet, and by compiling to it, we get that speed for free. We also get to tap into decades of C++ libraries and tools.

### Why Rust is the Core
The compiler itself needs to be fast, safe, and reliable. Rust's focus on memory safety and performance makes it the perfect tool for building a rock-solid compiler that won't crash while it's processing your genius ideas. This means fewer bugs in our tools, so you can focus on the bugs in your own brilliant, world-changing experiments.

---

## Appendix A: Keyword Reference

| Keyword | Description |
|---|---|
| `val` / `var` | Declare a constant / variable. |
| `def` | Defines a function. |
| `class` | Defines a class. |
| `if` / `else` | Conditional branching. |
| `while` / `for` | Looping constructs. |
| `break` / `continue` | Loop control. |
| `return` | Return a value from a function. |
| `print` | Print a value to the console. |
| `import` | Import another REDLINE module. |
| `pub` | Make a function or class public. |
| `new` | Allocate a new object instance. |
| `this` | Reference the current object instance. |
| `try` / `catch` | Handle runtime errors. |
| `true` / `false` | Boolean literals. |

---

## Appendix B: Standard Library Quick Reference

*   **System:** `args`, `len`, `append`, `sort`, `reverse`, `find`, `to_string`, `to_int`, `to_float`
*   **I/O:** `print`, `input`
*   **File System:** `read_file`, `write_file`, `exists`, `mkdir`, `remove`, `list_dir`
*   **Time:** `time`, `sleep`
*   **Random:** `random_int`, `random_float`
