# examples/v0.9_tests/overload_test.rl

# Overloaded 'add' function
def add(a: int, b: int) -> int:
    return a + b

def add(a: float, b: float) -> float:
    return a + b

def add(a: string, b: string) -> string:
    return a + b

# --- Main execution starts here ---

val r1: int = add(10, 20)
print("10 + 20 = " + to_string(r1))

val r2: float = add(3.14, 2.71)
print("3.14 + 2.71 = " + to_string(r2))

val r3: string = add("Hello, ", "World!")
print("'Hello, ' + 'World!' = " + r3)
