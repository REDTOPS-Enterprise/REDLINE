# Test for list functionality in REDLINE v0.5

print("--- List Test ---")

# 1. Declaration and Initialization
var my_list: list[int] = [10, 20, 30]
print("Initial list:")
for i in 0..len(my_list):
    print(my_list[i])

# 2. Append
print("Appending 40...")
append(my_list, 40)

# 3. Index Access and Reassignment
print("Changing element at index 1 to 99...")
my_list[1] = 99

# 4. Print final list
print("Final list:")
for i in 0..len(my_list):
    print(my_list[i])

print("List test complete.")
