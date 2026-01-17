# examples/v0.8_tests/file_test.rl

val filename: string = "test_output.txt"
val content: string = "This is a test. If you can read this, the experiment was a success. Or a horrible failure that resulted in sentience."

print("Attempting to write to " + filename + "...")
val success: bool = write_file(filename, content)

print("Write finished. Success value:")
print(success)

if success == true:
    print("Write successful. Now attempting to read the soul of the file...")
    val read_content: string = read_file(filename)
    print("File content:")
    print(read_content)

    if read_content == content:
        print("SUCCESS: The data survived the journey.")
    else:
        print("FAILURE: The data was corrupted. Burn the hard drive.")
else:
    print("FAILURE: Could not write to file. The machine spirit is displeased.")
