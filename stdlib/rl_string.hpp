#ifndef RL_STRING_H
#define RL_STRING_H

#include <string>
#include <vector>
#include <sstream>

namespace rl {

    // Checks if a string contains a substring.
    // Returns true if the needle is found in the haystack.
    inline bool contains(const std::string& haystack, const std::string& needle) {
        return haystack.find(needle) != std::string::npos;
    }

    // Splits a string into pieces based on a delimiter.
    // It's like taking a hammer to a vase, but for text.
    inline std::vector<std::string> split(const std::string& s, const std::string& delimiter) {
        std::vector<std::string> tokens;
        size_t start = 0;
        size_t end = s.find(delimiter);
        while (end != std::string::npos) {
            tokens.push_back(s.substr(start, end - start));
            start = end + delimiter.length();
            end = s.find(delimiter, start);
        }
        tokens.push_back(s.substr(start));
        return tokens;
    }

    // Joins a list of strings into a single string, separated by a delimiter.
    // It's the duct tape that puts the vase back together.
    inline std::string join(const std::vector<std::string>& list, const std::string& delimiter) {
        std::string result;
        for (size_t i = 0; i < list.size(); ++i) {
            result += list[i];
            if (i < list.size() - 1) {
                result += delimiter;
            }
        }
        return result;
    }
}

#endif
