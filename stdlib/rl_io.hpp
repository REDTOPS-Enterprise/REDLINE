// stdlib/rl_io.hpp
#ifndef RL_IO_H
#define RL_IO_H

#include <iostream>
#include <string>
#include <vector>

namespace rl {
    // Overload for printing std::string
    inline void print(const std::string& msg) {
        std::cout << msg << std::endl;
    }

    // Overload for printing string literals to prevent implicit bool conversion
    inline void print(const char* msg) {
        std::cout << msg << std::endl;
    }

    // Overload for printing integers
    inline void print(int val) {
        std::cout << val << std::endl;
    }

    // Overload for printing floating-point numbers
    inline void print(double val) {
        std::cout << val << std::endl;
    }

    // Overload for printing booleans
    inline void print(bool val) {
        std::cout << (val ? "true" : "false") << std::endl;
    }

    // Function to read a line of input from the user
    inline std::string input(const std::string& prompt = "") {
        if (!prompt.empty()) {
            std::cout << prompt;
        }
        std::string line;
        std::getline(std::cin, line);
        return line;
    }
}
#endif
