# Clients

The last thing outside your program, and the least cooperative: a service you do not own, on a network that fails differently from a disk, returning JSON somebody else designed. This section is where [errors](../02_Errors/README.md), [data](../06_Data/README.md) and testing all have to hold at once.

**These pages are stubs** — outlines waiting for a runnable example. See the [Errors](../02_Errors/README.md) section for what that means and how a page graduates. These are also the pages furthest from this library's promise that every claim is printed by a program CI runs: an example may not reach the network, so the finished pages will have to earn their output some other way — which is the subject of two of them.

| Lesson | Level | What it will teach |
|---|---|---|
| [An HTTP request](http_with_reqwest/README.md) | 201 | The smallest real `GET`, and the blocking-or-async decision you make before writing it |
| [Deserializing a response](deserializing_a_response/README.md) | 201 | JSON you did not design: a struct per level, or one pointer straight to the field you wanted |
| [Injecting the base URL](injecting_the_base_url/README.md) | 201 → 301 | The constant that makes a client untestable, and the parameter that does not |
| [Mocking a server](mocking_a_server/README.md) | 301 | What a fake server proves, what it cannot, and the test that passes against an API that no longer exists |
| [Units are types](units_are_types/README.md) | 201 | A number whose meaning lives in a comment, versus one the compiler will not let you add to a different kind |
