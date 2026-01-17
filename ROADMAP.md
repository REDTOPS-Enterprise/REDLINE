# 🗺️ REDLINE Roadmap

This document outlines the future direction of the REDLINE language and compiler.

## v0.9 - Memory, Overloads, & Polish

*   [x] **Automatic Memory Management**: Implemented `new` keyword and `std::shared_ptr` for all class objects to prevent memory leaks.
*   [x] **Function Overloading**: The compiler now supports defining multiple functions with the same name but different parameters.
*   [x] **Critical Safety Fixes**: Hardened the language by fixing out-of-bounds crashes and other memory-related bugs.
*   [x] **CLI Overhaul**: Rewrote the `redline.py` build script for a more professional and robust user experience.

## v1.0 - The Final Stretch

*   [ ] **Automated Testing Framework**: Create a `redline test` command that automatically discovers and runs test files.
*   [ ] **Package Manager**: A simple tool to fetch and manage third-party REDLINE libraries.
*   [ ] **Self-Hosted Compiler**: Rewrite the REDLINE compiler in REDLINE itself.


## Long-Term Vision (Post v1.0 and ahead)

*   [ ] **Cross-Platform Support**: Officially support and test on Windows (MSVC) and macOS (Clang).
*   [ ] **Language Server Protocol (LSP)**: Implement an LSP for better IDE integration (e.g., autocompletion, go-to-definition in VS Code, etc.).
*   [ ] **Concurrency**: Add support for multi-threading (`spawn`, `mutex`).
