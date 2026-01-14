# 🗺️ REDLINE Roadmap

This document outlines the future direction of the REDLINE language and compiler.

## Near-Term Goals (v0.6 - v0.7)

These are features planned for the next few releases.

*   [ ] **Module System (`import`)**: Allow REDLINE files to be split into modules and imported into other files. This is crucial for code organization and reuse.
    *   `import "my_module.rl"`
*   [ ] **Public Visibility (`pub`)**: Add a `pub` keyword to make functions and constants accessible from other modules.
*   [ ] **Object-Oriented Programming (`class` / `struct`)**: Introduce basic data structures for grouping data and behavior.
*   [ ] **C++ Interoperability**: Allow C++ projects to include and call functions from compiled REDLINE code. This would likely involve generating a C++ header file (`.hpp`) for a REDLINE module.

## Mid-Term Goals (v0.8 - v1.0)

*   [ ] **Enhanced Standard Library**:
    *   **File I/O**: `read_file`, `write_file`.
    *   **String Manipulation**: `split`, `join`, `contains`.
    *   **More List Functions**: `sort`, `reverse`, `find`.
*   [ ] **Improved Error Handling**: Introduce a `try/catch` mechanism or an `Option`/`Result` type for more robust error management.
*   [- ] **Automated Testing Framework**: Create a `redline test` command that automatically discovers and runs test files.
*   [ ] **Package Manager**: A simple tool to fetch and manage third-party REDLINE libraries.

## Long-Term Vision (Post v1.0)

*   [ ] **Self-Hosted Compiler**: Rewrite the REDLINE compiler in REDLINE itself.
*   [ ] **Cross-Platform Support**: Officially support and test on Windows (MSVC) and macOS (Clang).
*   [ ] **Language Server Protocol (LSP)**: Implement an LSP for better IDE integration (e.g., autocompletion, go-to-definition in VS Code, etc.).
*   [ ] **Concurrency**: Add support for multi-threading (`spawn`, `mutex`).
