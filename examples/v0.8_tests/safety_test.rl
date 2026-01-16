# examples/v0.8_tests/safety_test.rl

var numbers: list[int] = [1, 2, 3]

print("Attempting to access invalid index...")

try:
    # This should throw an error, but currently it might Segfault (crash)
    print(numbers[100])
catch e:
    print("Caught index out of bounds error! Safe.")

print("Program finished.")
