#ifndef RL_FILE_H
#define RL_FILE_H

#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <stdexcept> // For std::runtime_error

namespace rl {

    // Rips the soul out of a file and returns it as a string.
    // Throws an exception if the file refuses to yield its secrets.
    inline std::string read_file(const std::string& path) {
        std::ifstream file(path);
        if (!file.is_open()) {
            throw std::runtime_error("Could not open file: " + path);
        }
        std::stringstream buffer;
        buffer << file.rdbuf();
        return buffer.str();
    }

    // Shoves a string into a file. Overwrites everything. No mercy.
    // Throws an exception if the hard drive rejects our offering.
    inline bool write_file(const std::string& path, const std::string& content) {
        std::ofstream file(path);
        if (!file.is_open()) {
            throw std::runtime_error("Could not write to file: " + path);
        }
        file << content;
        return true;
    }
}

#endif
