#!/usr/bin/env python3
import os
import subprocess
import sys
from pathlib import Path

# --- Constants ---
PROJECT_ROOT = Path(__file__).parent.resolve()
CORE_DIR = PROJECT_ROOT / "redline-core"
CORE_BIN = CORE_DIR / "target" / "release" / "redline-core"


def init_core():
    """Compiles the Rust core and streams its output."""
    print("---")
    print("🚀 Initializing REDLINE Core (Rust)...")
    print(f"   (Running 'cargo build --release' in {CORE_DIR})")
    print("---")
    try:
        # Use Popen to stream output in real-time for better user feedback
        process = subprocess.Popen(
            ["cargo", "build", "--release"],
            cwd=CORE_DIR,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            encoding='utf-8'
        )
        
        # Print cargo's output line by line as it comes in
        for line in iter(process.stdout.readline, ''):
            sys.stdout.write(f"   {line}")
        
        process.wait()  # Wait for the process to complete
        
        if process.returncode != 0:
            print("\n---")
            print(f"❌ Core initialization failed: Cargo returned non-zero exit status {process.returncode}.")
            print("---")
            return False

        print("\n---")
        print("✅ REDLINE Core initialized successfully!")
        print("---")
        return True
    except FileNotFoundError:
        print("❌ Error: 'cargo' command not found. Is Rust installed and in your PATH?")
        return False
    except Exception as e:
        print(f"\n❌ An unexpected error occurred during initialization: {e}")
        return False


def main():
    """Main entry point for the script."""
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python redline.py build <file.rl>   # Compile a REDLINE file")
        print("  python redline.py init            # Initialize/Build the compiler core")
        return

    command = sys.argv[1]

    # --- Init Command ---
    if command == "init":
        init_core()
        return

    # --- Build Command ---
    if command == "build":
        if len(sys.argv) < 3:
            print("❌ Error: Missing file path for 'build' command.")
            print("Usage: python redline.py build <file.rl>")
            return
            
        source_file_path = sys.argv[2]
        source_file = Path(source_file_path).resolve()

        if not source_file.exists() or source_file.suffix != ".rl":
            print(f"❌ Error: File not found or not a .rl file: {source_file}")
            return

        # Check for core binary and build if it's missing
        if not CORE_BIN.exists():
            print(f"⚠️ REDLINE Core binary not found. Running 'init' first...")
            if not init_core():
                print("\n❌ Aborting build due to core initialization failure.")
                return
            print("\n✅ Core initialized. Continuing with build...")

        cpp_output = PROJECT_ROOT / "output.cpp"
        exe_output = PROJECT_ROOT / source_file.stem

        try:
            # Step 1: Run the Rust Core (Source .rl -> C++ code)
            print(f"\n[1/2] 🧠 REDLINE Core parsing: {source_file.name}")
            with open(cpp_output, "w") as f:
                subprocess.run([str(CORE_BIN), str(source_file)], stdout=f, check=True, text=True)

            # Step 2: Compile with G++
            print(f"[2/2] 🛠️  G++ compiling: {exe_output.name}")
            subprocess.run(
                [
                    "g++",
                    "-std=c++11",
                    str(cpp_output),
                    "-o",
                    str(exe_output),
                    f"-I{PROJECT_ROOT}",  # This helps find the stdlib
                ],
                check=True,
            )

            print(f"\n🚀 Success! Compiled to: ./{exe_output.name}")

            # Optional: Clean up the temp .cpp file
            # cpp_output.unlink()

        except subprocess.CalledProcessError as e:
            print(f"\n❌ Build Failed: {e}")
        except Exception as e:
            print(f"\n❌ An unexpected error occurred: {e}")
        return

    # --- Unknown Command ---
    print(f"❌ Error: Unknown command '{command}'")
    print("Usage:")
    print("  python redline.py build <file.rl>   # Compile a REDLINE file")
    print("  python redline.py init            # Initialize/Build the compiler core")


if __name__ == "__main__":
    main()
