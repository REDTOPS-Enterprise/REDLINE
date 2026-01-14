#ifndef RL_STDLIB_HPP
#define RL_STDLIB_HPP

#include <vector>
#include <string>

namespace rl {
    // Returns the number of elements in a vector.
    template<typename T>
    int len(const std::vector<T>& vec) {
        return vec.size();
    }

    // Appends an element to a vector.
    template<typename T>
    void append(std::vector<T>& vec, const T& value) {
        vec.push_back(value);
    }
}

#endif // RL_STDLIB_HPP
