# examples/v0.9_tests/memory_test.rl

class Person:
    var name: string = ""
    var age: int = 0

    def init(n: string, a: int):
        this.name = n
        this.age = a

    def greet():
        print("Hello, I am " + this.name)

# --- Main execution starts here ---

var people: list[Person] = []

append(people, new Person("Alice", 30))
append(people, new Person("Bob", 40))

print("Created " + to_string(len(people)) + " people.")

for i in 0..len(people):
    people[i].greet()
