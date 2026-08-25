/* The C++ version, with a real reference rather than a raw pointer — and the
 * same non-answer.
 *
 * THIS PROGRAM HAS UNDEFINED BEHAVIOUR ON PURPOSE. See dangling.c.
 *
 *   c++ -std=c++20 -Wall dangling.cpp -o /tmp/danglingpp && /tmp/danglingpp
 */
#include <iostream>
#include <string>

int main() {
    const std::string* keep;
    {
        std::string s = "something important";
        keep = &s;                 /* the reference outlives the value */
    }                              /* ~string() runs here */

    std::cout << "keep = " << *keep << "\n";   /* reads freed memory */
    return 0;
}
