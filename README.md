# 🟥 REDLINE v1.0.0

**C++ but Simplified.**

REDLINE is a high-performance, transpiled systems programming language designed to combine the **readability of Python** with the **raw speed of C++**. It compiles your REDLINE code (`.rl`) into optimized C++, which is then compiled into a native executable.

## 📦 Installation

Installation is simple. You just need to clone the repository and initialize the compiler core.

### Prerequisites
*   **Rust & Cargo**: To build the REDLINE compiler core.
*   **G++ (C++17 compatible)**: To compile the generated C++ code.
*   **Python 3**: To run the build script.
*   **Git**: To clone the repository.

### Steps
1.  **Clone the Repository:**
    Find a good place on your computer and clone the REDLINE repository.
    ```bash
    git clone https://github.com/REDTOPS-Enterprise/REDLINE.git
    ```

2.  **Navigate into the Directory:**
    ```bash
    cd REDLINE
    ```

3.  **Initialize the Compiler:**
    Run the `init` command. This will use `cargo` to build the Rust-based compiler core.
    ```bash
    python redline.py init
    ```

Once these steps are complete, you are ready to use the compiler.

## 🛠️ Usage

All commands are run using the `redline.py` script from within the repository folder.

### Building a Project
If your project has a `RedConfig.toml` file, you can build it by running the `build` command from your project's root directory.
```bash
# Assuming you are inside a project like 'examples/v1.0_tests/MyTestProject'
python ../../../redline.py build
```

### Compiling a Single File
You can also compile a single file directly by providing its path:
```bash
python redline.py build path/to/my_file.rl
```

### (Optional) Making `redline` a Global Command
If you want the convenience of running `redline` from anywhere, you can add the REDLINE repository directory to your system's `PATH` environment variable.

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) for the full list of planned features and future goals.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
