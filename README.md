# 🟥 REDLINE

**C++ but Simplified.**

REDLINE is a high-performance, transpiled systems programming language designed to combine the **readability of Python** with the **raw speed of C++**. It compiles your REDLINE code (`.rl`) into optimized C++, which is then compiled into a native executable.

## 🚀 Key Features

*   **Python-like Syntax**: Clean, indentation-based structure. No semicolons or curly braces required.
*   **C++ Performance**: Transpiles directly to C++, leveraging the full power of the G++ compiler.
*   **Rich Type System**: Supports `int`, `float`, `string`, `bool`, and `list[T]`.
*   **Modern Tooling**: Built-in Lexer, Parser, and Code Generator written in Rust for speed and safety.
*   **Standard Library**: Includes fast I/O, math, and list manipulation functions out of the box.

## 📦 Installation

### Prerequisites
*   **Rust**: To build the REDLINE compiler core.
*   **G++**: To compile the generated C++ code.
*   **Python 3**: To run the build script wrapper.

### Building the Compiler
1.  Clone the repository:
    ```bash
    git clone https://github.com/yourusername/REDLINE.git
    cd REDLINE
    ```
2.  Build the core compiler (Rust):
    ```bash
    cd redline-core
    cargo build --release
    cd ..
    ```

## 🛠️ Usage

### Compiling and Running a File
The easiest way to compile and run a REDLINE file is using the `redline.py` wrapper script.

```bash
python redline.py <path_to_file.rl>
```

**Example:**
```bash
python redline.py examples/v0.5_tests/list_test.rl
```

## 📝 Example Code

**List Manipulation:**
```redline
# Declare a list of integers
var my_list: list[int] = [10, 20, 30]

# Add an element
append(my_list, 40)

# Change an element
my_list[1] = 99

# Print the list
for i in 0..len(my_list):
    print(my_list[i])
```

**Fibonacci Sequence:**
```redline
def fib(n: int) -> int:
    if n <= 1:
        return n
    else:
        return fib(n - 1) + fib(n - 2)

for i in 0..10:
    print(fib(i))
```

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) for the full list of planned features and future goals.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
