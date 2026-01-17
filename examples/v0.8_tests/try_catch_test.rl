# examples/v0.8_tests/try_catch_test.rl

print("Attempting to read a non-existent file...")

try:
    val content: string = read_file("ghost_file.txt")
    print("This should not be printed.")
catch e:
    print("Caught an error!")
    # In a real implementation, we would access 'e.what()', but for now we just catch it.
    print("The ghost file remains elusive.")

print("Program continued execution after catch block.")
