#!/usr/bin/env python3
import os
import subprocess
import sys
import json
import re
from pathlib import Path

# --- Constants ---
VERSION = "0.9.0"
PROJECT_ROOT = Path(__file__).parent.resolve()
CORE_DIR = PROJECT_ROOT / "redline-core"
CORE_BIN = CORE_DIR / "target" / "release" / "redline-core"
BUILD_DIR = PROJECT_ROOT / "build"

ASCII_ART = r"""
██████╗ ███████╗██████╗ ██╗     ██╗███╗   ██╗███████╗
██╔══██╗██╔════╝██╔══██╗██║     ██║████╗  ██║██╔════╝
██████╔╝█████╗  ██║  ██║██║     ██║██╔██╗ ██║█████╗
██╔══██╗██╔══╝  ██║  ██║██║     ██║██║╚██╗██║██╔══╝
██║  ██║███████╗██████╔╝███████╗██║██║ ╚████║███████╗
╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝
"""

def print_usage():
    """Prints the main help message with ASCII art and commands."""
    print(ASCII_ART)
    print(f"REDLINE Compiler v{VERSION}")
    print("---------------------------------")
    print("A high-performance, transpiled systems language.")
    print("\nUsage:")
    print("  python redline.py <command> [arguments]")
    print("\nCommands:")
    print("  build <file>    Compile a REDLINE (.rl) or C++ (.cpp) file.")
    print("  parse <file.rl> Generate C++ code from a REDLINE file without compiling.")
    print("  lib <file.rl>   Compile a REDLINE file into a static library (.o).")
    print("  test            Run all tests in a local 'tests/' directory.")
    print("  init            Initialize and build the REDLINE compiler core.")
    print("  help            Show this help message.")

class Module:
    """Represents a single REDLINE module (a .rl file)."""
    def __init__(self, source_path, ast):
        self.source_path = source_path
        self.ast = ast
        self.name = source_path.stem
        self.cpp_path = BUILD_DIR / f"{self.name}.cpp"
        self.hpp_path = BUILD_DIR / f"{self.name}.hpp"
        self.obj_path = BUILD_DIR / f"{self.name}.o"

    def get_imports(self):
        """Extracts import paths from the module's AST."""
        imports = []
        for statement in self.ast.get('statements', []):
            if 'Import' in statement:
                imports.append(statement['Import'])
        return imports

class Compiler:
    """Orchestrates the compilation of a REDLINE project."""

    def __init__(self, core_bin_path):
        self.core_bin_path = core_bin_path
        self.modules = {} # Cache for compiled modules: path -> Module

    def get_ast(self, source_file):
        """Runs the core parser and returns the AST as a JSON object."""
        try:
            result = subprocess.run(
                [str(self.core_bin_path), str(source_file), "--json-ast"],
                capture_output=True, text=True, check=True,
            )
            return json.loads(result.stdout)
        except (subprocess.CalledProcessError, json.JSONDecodeError) as e:
            print(f"Error: Failed to parse {source_file.name}.")
            if hasattr(e, 'stderr'): print(e.stderr, file=sys.stderr)
            return None

    def compile_module_recursive(self, source_path):
        """Recursively compiles a module and its dependencies."""
        if source_path in self.modules:
            return self.modules[source_path]

        print(f"  -> Analyzing module: {source_path.name}")
        ast = self.get_ast(source_path)
        if not ast:
            return None

        module = Module(source_path, ast)
        self.modules[source_path] = module

        for import_path_str in module.get_imports():
            import_path = source_path.parent / import_path_str
            if not self.compile_module_recursive(import_path):
                return None
        
        return module

    def generate_code(self, module, mode):
        """Generates .cpp or .hpp file for a single module."""
        output_path = module.cpp_path if mode == "cpp" else module.hpp_path
        try:
            result = subprocess.run(
                [str(self.core_bin_path), str(module.source_path), "--gen", mode],
                capture_output=True, text=True, check=True,
            )
            output_path.write_text(result.stdout)
            return True
        except subprocess.CalledProcessError as e:
            print(f"Error: Failed to generate {mode.upper()} for {module.name}.")
            print(e.stderr, file=sys.stderr)
            return False

def init_core():
    """Initializes the REDLINE compiler core."""
    print("Initializing REDLINE Core...")
    try:
        subprocess.run(
            ["cargo", "build", "--release"],
            cwd=CORE_DIR, check=True, capture_output=True, text=True
        )
        print("REDLINE Core initialized successfully.")
        return True
    except (subprocess.CalledProcessError, FileNotFoundError) as e:
        print(f"Error: Core initialization failed.")
        if hasattr(e, 'stderr'): print(e.stderr, file=sys.stderr)
        return False

def get_source_file(command_name, args):
    if len(args) < 3:
        print(f"Error: Missing file path for '{command_name}' command.")
        print(f"Usage: python redline.py {command_name} <file>")
        return None
    
    file_path = Path(args[2])
    if not file_path.is_absolute():
        file_path = Path.cwd() / file_path
    
    if not file_path.exists():
        print(f"Error: File not found: {file_path}")
        return None
    return file_path

def run_tests():
    """Discovers and runs all tests."""
    print("Running tests...")
    test_dir = Path.cwd() / "tests"
    if not test_dir.exists():
        print("No 'tests' directory found.")
        return

    test_files = list(test_dir.rglob("*.rl"))
    
    if not test_files:
        print("No tests found in 'tests/' directory.")
        return

    passed = 0
    failed = 0

    for test_file in test_files:
        print(f"--- Running: {test_file.relative_to(Path.cwd())} ---")
        
        compiler = Compiler(CORE_BIN)
        BUILD_DIR.mkdir(exist_ok=True)

        main_module = compiler.compile_module_recursive(test_file)
        if not main_module:
            print("🔴 FAILED (Module Analysis)")
            failed += 1
            continue

        all_modules = list(compiler.modules.values())
        
        for module in all_modules:
            if not compiler.generate_code(module, "hpp") or not compiler.generate_code(module, "cpp"):
                print("🔴 FAILED (Code Generation)")
                failed += 1
                continue
        
        exe_output = BUILD_DIR / test_file.stem
        cpp_files = [m.cpp_path for m in all_modules]

        try:
            cmd = ["g++", "-std=c++11", *cpp_files, "-o", str(exe_output), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"]
            subprocess.run(cmd, check=True, capture_output=True, text=True)
            
            run_result = subprocess.run([str(exe_output)], check=True, capture_output=True, text=True)
            print(run_result.stdout.strip())
            print("🟢 PASSED")
            passed += 1

        except subprocess.CalledProcessError as e:
            print("🔴 FAILED (Compilation or Runtime)")
            print(e.stderr, file=sys.stderr)
            failed += 1
        
        print("-" * (len(str(test_file.relative_to(Path.cwd()))) + 14))

    print("\n--- Test Summary ---")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print("--------------------")
    
    if failed > 0:
        sys.exit(1)

def main():
    if len(sys.argv) < 2 or sys.argv[1] == 'help':
        print_usage()
        return

    command = sys.argv[1]

    if command == "init":
        init_core()
        return
        
    if command == "test":
        run_tests()
        return

    if command not in ["build", "parse", "lib"]:
        print(f"Error: Unknown command '{command}'")
        print_usage()
        return

    if not CORE_BIN.exists():
        print("REDLINE Core binary not found. Running 'init' first...")
        if not init_core():
            print("Aborting due to core initialization failure.")
            return
        print("Core initialized. Continuing...")

    source_file = get_source_file(command, sys.argv)
    if not source_file: return

    compiler = Compiler(CORE_BIN)
    BUILD_DIR.mkdir(exist_ok=True)

    if command == "build" and source_file.suffix == ".cpp":
        print(f"Starting C++ Interop build for: {source_file.name}")
        
        content = source_file.read_text()
        includes = re.findall(r'#include\s+"([^"]+)\.hpp"', content)
        
        for inc in includes:
            rl_path = source_file.parent / f"{inc}.rl"
            if rl_path.exists():
                print(f"  -> Detected REDLINE dependency: {rl_path.name}")
                if not compiler.compile_module_recursive(rl_path):
                    print(f"Failed to compile dependency: {rl_path.name}")
                    return

        all_modules = list(compiler.modules.values())
        if all_modules:
            print("Compiling REDLINE dependencies...")
            for module in all_modules:
                if not compiler.generate_code(module, "hpp") or not compiler.generate_code(module, "cpp"):
                    return
                
                print(f"  -> Compiling {module.name}.o")
                try:
                    subprocess.run(
                        ["g++", "-std=c++11", "-c", str(module.cpp_path), "-o", str(module.obj_path), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"],
                        check=True
                    )
                except subprocess.CalledProcessError as e:
                    print(f"Compilation failed for {module.name}: {e}")
                    return

        exe_output = PROJECT_ROOT / source_file.stem
        obj_files = [str(m.obj_path) for m in all_modules]
        
        print("Compiling and linking C++ application...")
        try:
            cmd = ["g++", "-std=c++11", str(source_file), *obj_files, "-o", str(exe_output), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"]
            subprocess.run(cmd, check=True)
            print(f"Build successful. Executable created at: ./{exe_output.name}")
        except subprocess.CalledProcessError as e:
            print(f"G++ compilation failed: {e}")
        return

    if source_file.suffix != ".rl":
        print(f"Error: Expected a .rl file for this command, but got: {source_file.suffix}")
        return

    print(f"Starting build for entry point: {source_file.name}")
    main_module = compiler.compile_module_recursive(source_file)

    if not main_module:
        print("Build failed during module analysis.")
        return

    all_modules = list(compiler.modules.values())

    print("Generating C++ code...")
    for module in all_modules:
        if not compiler.generate_code(module, "hpp") or not compiler.generate_code(module, "cpp"):
            return

    if command == "parse":
        print(f"C++ output generated in: {BUILD_DIR}")
        return

    if command == "lib":
        print("Compiling object files...")
        for module in all_modules:
            print(f"  -> Compiling {module.name}.o")
            try:
                subprocess.run(
                    ["g++", "-std=c++11", "-c", str(module.cpp_path), "-o", str(module.obj_path), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"],
                    check=True
                )
            except subprocess.CalledProcessError as e:
                print(f"Compilation failed for {module.name}: {e}")
                return
        print(f"Library object files generated in: {BUILD_DIR}")
        return

    if command == "build":
        exe_output = PROJECT_ROOT / source_file.stem
        cpp_files = [m.cpp_path for m in all_modules]

        print("Compiling and linking...")
        try:
            cmd = ["g++", "-std=c++11", *cpp_files, "-o", str(exe_output), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"]
            subprocess.run(cmd, check=True)
            print(f"Build successful. Executable created at: ./{exe_output.name}")
        except subprocess.CalledProcessError as e:
            print(f"G++ compilation failed: {e}")
        finally:
            pass

if __name__ == "__main__":
    main()
