# examples/v0.8_tests/list_test_v2.rl

var numbers: list[int] = [5, 2, 9, 1, 5, 6]

print("Original list:")
for i in 0..len(numbers):
    print(numbers[i])

# Test sort
print("Sorting...")
sort(numbers)
print("Sorted list:")
for i in 0..len(numbers):
    print(numbers[i])

# Test reverse
print("Reversing...")
reverse(numbers)
print("Reversed list:")
for i in 0..len(numbers):
    print(numbers[i])

# Test find
val index_of_9: int = find(numbers, 9)
print("Index of 9: " + to_string(index_of_9))

val index_of_99: int = find(numbers, 99)
print("Index of 99: " + to_string(index_of_99))
