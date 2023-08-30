# coogler
[coogle](), but for all us plebs who do not have access to the Jai beta.
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
