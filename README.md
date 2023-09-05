# coogler
[coogle](https://www.youtube.com/playlist?list=PLpM-Dvs8t0VYhYLxY-i7OcvBbDsG4izam), but for all us plebs who do not have access to the Jai beta.
The name comes from **coogle** and **R**ust.

It's basically a [Hoogle](https://hoogle.haskell.org/)-like search engine for C functions by signature.
You give it a C header or C source file and a desired function signature and it gives you a list of all the functions with that specific signature.

## Compiling and Running

    cargo run --release <SOURCE_FILE> <SEARCH_QUERY>

Or:

    cargo build --release
    target/release/coogler <SOURCE_FILE> <SEARCH_QUERY>

Examples:

    coogler /usr/include/stdlib.h 'int()'

Will yield something close to:

    /usr/include/stdlib.h:573:12: rand :: int()
    /usr/include/stdlib.h:803:12: clearenv :: int()
    /usr/include/stdlib.h:521:17: random :: long()
    /usr/include/stdlib.h:591:17: lrand48 :: long()
    /usr/include/stdlib.h:596:17: mrand48 :: long()
    /usr/include/stdlib.h:980:12: abs :: int(int __x)
    /usr/include/stdlib.h:98:15: __ctype_get_mb_cur_max :: size_t()
    /usr/include/stdlib.h:730:13: abort :: void()
    /usr/include/stdlib.h:587:15: drand48 :: double()
    /usr/include/stdlib.h:786:12: putenv :: int(char * __string)
    /usr/include/stdlib.h:827:12: mkstemp :: int(char * __template)
    /usr/include/stdlib.h:657:19: arc4random :: __uint32_t()
    /usr/include/stdlib.h:756:13: exit :: void(int __status)
    /usr/include/stdlib.h:762:13: quick_exit :: void(int __status)
    /usr/include/stdlib.h:768:13: _Exit :: void(int __status)
    /usr/include/stdlib.h:981:17: labs :: long(long __x)
    /usr/include/stdlib.h:601:13: srand48 :: void(long __seedval)
    /usr/include/stdlib.h:505:14: l64a :: char *(long __n)
    /usr/include/stdlib.h:687:13: free :: void(void * __ptr)
    /usr/include/stdlib.h:672:14: malloc :: void *(size_t __size)
