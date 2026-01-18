#!/usr/bin/env python3
import os
import subprocess
import sys
import json
import re
import shutil
from pathlib import Path
try:
    import tomllib
except ImportError:
    # For Python versions before 3.11, use the 'toml' library
    try:
        import toml as tomllib
    except ImportError:
        print("Error: 'toml' library not found. Please run 'pip install toml'")
        sys.exit(1)

# --- Constants ---
VERSION = "1.0.1"
# Use os.path.realpath to correctly resolve the script's true location, even when symlinked
PROJECT_ROOT = Path(os.path.dirname(os.path.realpath(__file__)))
CORE_DIR = PROJECT_ROOT / "redline-core"
CORE_BIN = CORE_DIR / "target" / "release" / "redline-core"
BUILD_DIR = PROJECT_ROOT / "temp_build"

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
    print("  build [file]    Compile a REDLINE project or a single file.")
    print("  parse <file.rl> Generate C++ code from a REDLINE file without compiling.")
    print("  lib <file.rl>   Compile a REDLINE file into a static library (.o).")
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
            if hasattr(e, 'stderr') and e.stderr: print(e.stderr, file=sys.stderr)
            return None

    def compile_module_recursive(self, source_path):
        """Recursively compiles a module and its dependencies."""
        if source_path in self.modules:
            return self.modules[source_path]

        print(f"  -> Analyzing module: {source_path.name}")
        ast = self.get_ast(source_file)
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
            if hasattr(e, 'stderr') and e.stderr: print(e.stderr, file=sys.stderr)
            return False

def init_core():
    """Initializes the REDLINE compiler core."""
    print("Initializing REDLINE Core...")
    
    # Ensure we are running from the project root for cargo
    os.chdir(PROJECT_ROOT)

    try:
        result = subprocess.run(
            ["cargo", "build", "--release"],
            cwd=CORE_DIR,
            check=True,
            capture_output=True,
            text=True
        )
        print(result.stdout)
        if result.stderr:
            print(result.stderr, file=sys.stderr)

        print("REDLINE Core initialized successfully.")
        return True
    except FileNotFoundError:
        print("Error: 'cargo' command not found. Please install Rust from https://rustup.rs/.")
        return False
    except subprocess.CalledProcessError as e:
        print("\nError: Cargo build failed.")
        print(e.stdout)
        print(e.stderr, file=sys.stderr)
        return False
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
        return False

def find_config(start_path):
    """Searches upward from a path for a RedConfig.toml file."""
    current_path = start_path.resolve()
    while current_path != current_path.parent:
        config_file = current_path / "RedConfig.toml"
        if config_file.exists():
            return config_file
        current_path = current_path.parent
    return None

def main():
    # Ensure all paths are relative to the project root, even if called from elsewhere
    os.chdir(PROJECT_ROOT)

    if len(sys.argv) < 2 or sys.argv[1] == 'help':
        print_usage()
        return

    command = sys.argv[1]

    if command == "init":
        if not init_core():
            sys.exit(1)
        return

    if not CORE_BIN.exists():
        print("REDLINE Core binary not found. Running 'init' first...")
        if not init_core():
            print("Aborting due to core initialization failure.")
            sys.exit(1)
        print("Core initialized. Continuing...")

    source_file = None
    project_name = None
    output_dir = None
    
    # Determine build mode (file vs. config)
    if command == "build" and len(sys.argv) < 3:
        # Config-based build from current directory
        config_path = Path.cwd() / "RedConfig.toml"
        if not config_path.exists():
            print("Error: No input file specified and no RedConfig.toml found in current directory.")
            print_usage()
            return
    else:
        # File-based build, potentially with config lookup
        file_arg = sys.argv[2] if len(sys.argv) >= 3 else None
        if not file_arg:
            print(f"Error: Missing file path for '{command}' command.")
            print_usage()
            return
        
        source_file = Path(file_arg)
        if not source_file.is_absolute():
            source_file = Path.cwd() / source_file
        
        if not source_file.exists():
            print(f"Error: File not found: {source_file}")
            return
            
        config_path = find_config(source_file.parent)

    if config_path:
        print(f"Found RedConfig.toml at: {config_path}")
        with open(config_path, "rb") as f:
            config = tomllib.load(f)
        
        project_config = config.get("project", {})
        project_name = project_config.get("name", "a.out")
        entry_point_str = project_config.get("entry_point")
        output_dir_str = project_config.get("output_dir", ".")
        
        if not entry_point_str:
            print("Error: RedConfig.toml is missing 'entry_point'.")
            return
        
        project_root = config_path.parent
        source_file = (project_root / entry_point_str).resolve()
        output_dir = (project_root / output_dir_str).resolve()
        output_dir.mkdir(exist_ok=True)
    else:
        if not source_file:
            print("Error: No source file specified and no RedConfig.toml found.")
            return
        project_name = source_file.stem
        output_dir = source_file.parent

    compiler = Compiler(CORE_BIN)
    BUILD_DIR.mkdir(exist_ok=True)

    try:
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
                        ["g++", "-std=c++17", "-c", str(module.cpp_path), "-o", str(module.obj_path), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"],
                        check=True
                    )
                except subprocess.CalledProcessError as e:
                    print(f"Compilation failed for {module.name}: {e}")
                    return
            print(f"Library object files generated in: {BUILD_DIR}")
            return

        if command == "build":
            exe_output = output_dir / project_name
            cpp_files = [m.cpp_path for m in all_modules]

            print("Compiling and linking...")
            try:
                cmd = ["g++", "-std=c++17", *cpp_files, "-o", str(exe_output), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"]
                subprocess.run(cmd, check=True)
                print(f"Build successful. Executable created at: {exe_output}")
            except subprocess.CalledProcessError as e:
                print(f"G++ compilation failed: {e}")
    finally:
        if BUILD_DIR.exists():
            shutil.rmtree(BUILD_DIR)
            print("Temporary build directory cleaned up.")

if __name__ == "__main__":
    main()
