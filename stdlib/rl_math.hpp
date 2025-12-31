#ifndef RL_MATH_HPP
#define RL_MATH_HPP

#include <cmath>

namespace rl {
    // Basic math functions
    inline double abs(double x) { return std::abs(x); }
    inline double sqrt(double x) { return std::sqrt(x); }
    inline double pow(double base, double exp) { return std::pow(base, exp); }
    inline double sin(double x) { return std::sin(x); }
    inline double cos(double x) { return std::cos(x); }
    inline double tan(double x) { return std::tan(x); }
    inline double log(double x) { return std::log(x); }
    inline double log10(double x) { return std::log10(x); }
    inline double exp(double x) { return std::exp(x); }
    inline double floor(double x) { return std::floor(x); }
    inline double ceil(double x) { return std::ceil(x); }
    inline double round(double x) { return std::round(x); }

    // Min and max
    inline double min(double a, double b) { return a < b ? a : b; }
    inline double max(double a, double b) { return a > b ? a : b; }

    // Constants
    constexpr double PI = 3.14159265358979323846;
    constexpr double E = 2.71828182845904523536;
}

#endif // RL_MATH_HPP
