# examples/v0.8_tests/string_test.rl

val raw_data: string = "apple,banana,cherry,date"
print("Original string: " + raw_data)

# Test split
val fruits: list[string] = split(raw_data, ",")
print("Split into " + to_string(len(fruits)) + " fruits.")

for i in 0..len(fruits):
    print("- " + fruits[i])

# Test contains
val has_banana: bool = contains(raw_data, "banana")
if has_banana:
    print("Yes, we have no bananas. Wait, we do.")
else:
    print("No bananas found.")

val has_pizza: bool = contains(raw_data, "pizza")
if has_pizza:
    print("Pizza found!")
else:
    print("No pizza found.")

# Test join
val new_string: string = join(fruits, " | ")
print("Joined with pipe: " + new_string)
