/* The same again in C++, which answers identically on shadowing and adds the
 * one thing C lacks: a destructor, so a value really does die at its brace.
 *
 *   c++ -std=c++20 -Wall -Wshadow shadow.cpp -o /tmp/shadowpp && /tmp/shadowpp
 */
#include <iostream>
#include <string>

struct Noisy {
    std::string label;
    explicit Noisy(std::string l) : label(std::move(l)) {}
    ~Noisy() { std::cout << "  DROP " << label << "\n"; }
};

int main() {
    std::string s = "something important";
    const std::string& keep = s;   /* a reference — much closer to &s than C's alias */

    /* `int s = 5;` here is the same error C gives:
     *     error: redefinition of 's' with a different type: 'int' vs 'std::string'
     */
    {
        int s = 5;   /* warning: declaration shadows a local variable [-Wshadow] */
        std::cout << "s = " << s << ", but the string is still there: " << keep << "\n";
    }

    /* RAII really is Rust's Drop: reverse declaration order, at the brace. */
    {
        Noisy first("first  — declared first");
        Noisy second("second — declared second");
        std::cout << "  both alive: " << first.label << " / " << second.label << "\n";
    }
    std::cout << "      Same order Rust prints. What C++ does NOT have is a rule\n"
                 "      that `keep` cannot outlive what it refers to — see dangling.cpp.\n";
    return 0;
}
