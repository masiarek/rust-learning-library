/* The same shape as the Rust lesson, in C — and the two places it will not go.
 *
 *   cc -std=c17 -Wall -Wshadow shadow.c -o /tmp/shadow && /tmp/shadow
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *s = strdup("something important");
    char *keep = s;   /* an alias. NOT a borrow: nothing relates their lifetimes. */

    /* The line the Rust program writes here is:
     *
     *     let s = 5;
     *
     * In C, in this same block, that is not a shadow but a redefinition:
     *
     *     int s = 5;
     *     error: redefinition of 's' with a different type: 'int' vs 'char *'
     *     note: previous definition is here
     *
     * And it is not about the type change. Same type, same verdict:
     *
     *     int x = 1; int x = 2;
     *     error: redefinition of 'x'
     *
     * A name belongs to its block, once. To shadow you must open a new block —
     * and then the compiler treats it as a suspected mistake, which is the
     * opposite of how Rust reads the same line.
     */
    {
        int s = 5;   /* warning: declaration shadows a local variable [-Wshadow] */
        printf("s = %d, but the string is still there: %s\n", s, keep);
    }

    printf("outer s is back: %s\n", s);   /* the inner binding is gone, not hidden */

    free(s);   /* the free is yours to remember, and yours to get wrong */
    return 0;
}
