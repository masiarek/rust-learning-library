/* The half that has no Rust equivalent, because rustc rejects it.
 *
 * THIS PROGRAM HAS UNDEFINED BEHAVIOUR ON PURPOSE. Its output is not an answer
 * key and is not checked by CI -- it cannot be, because "what a read from a
 * released frame prints" is not a property the language defines. That is the
 * whole point: it may print 7, it may print garbage, it may print 7 today and
 * garbage after an unrelated edit.
 *
 *   cc -std=c17 -Wall use_after_return.c -o /tmp/uar && /tmp/uar
 *
 * The compiler does warn here -- clang calls it -Wreturn-stack-address, gcc
 * calls it -Wreturn-local-addr. It still produces a binary, and the warning
 * only sees the `return &b` it can point at: move that line behind a helper
 * and both compilers go quiet while the bug stays exactly the same.
 */
#include <stdio.h>

struct Ballot { unsigned precinct; unsigned score; };

/* Returns a pointer into a frame that is released as this returns. */
struct Ballot *cast(unsigned precinct, unsigned score) {
    struct Ballot b = { precinct, score };
    return &b;                      /* the frame ends on the next line */
}

/* An ordinary call that reissues the same region for its own locals. */
unsigned reuse(void) {
    unsigned scratch[8];
    for (unsigned i = 0; i < 8; i++) scratch[i] = 0xDEADBEEF;
    return scratch[3];
}

int main(void) {
    struct Ballot *kept = cast(7, 42);

    printf("before the region is reissued: precinct=%u score=%u\n",
           kept->precinct, kept->score);

    reuse();                        /* ...same region, different contents */

    printf("after:                        precinct=%u score=%u\n",
           kept->precinct, kept->score);

    return 0;
}
