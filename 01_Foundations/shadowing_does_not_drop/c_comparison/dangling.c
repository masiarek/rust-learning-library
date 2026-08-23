/* The half that has no Rust equivalent, because rustc rejects it.
 *
 * THIS PROGRAM HAS UNDEFINED BEHAVIOUR ON PURPOSE. Its output is not an answer
 * key and is not checked by CI — it cannot be, because "what a use-after-free
 * prints" is not a property the language defines. That is the entire point.
 *
 *   cc -std=c17 -Wall dangling.c -o /tmp/dangling && /tmp/dangling
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *s = strdup("something important");
    char *keep = s;

    free(s);                        /* the owner is gone... */
    printf("keep = %s\n", keep);    /* ...and C reads it anyway. No diagnostic. */

    return 0;
}
