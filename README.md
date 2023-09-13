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

Where the search query may be of the form `[<function name> ::] <function signature>`

## Examples
### Example output
You can, for example, search the libc headers for a function returning an `int` and taking no parameters in the following way:

    $ coogler /usr/include/stdlib.h 'int()'

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

Like coogle, this tool works great for searching the [Raylib](https://github.com/raysan5/raylib) headers:

    $ coogler raylib/include/raylib.h 'contrast :: Color(Color, float)'

    raylib/include/raylib.h:1339:13: ColorContrast :: Color(Color color, float contrast)
    raylib/include/raylib.h:1340:13: ColorAlpha :: Color(Color color, float alpha)
    raylib/include/raylib.h:1331:13: Fade :: Color(Color color, float alpha)
    raylib/include/raylib.h:1337:13: ColorTint :: Color(Color color, Color tint)
    raylib/include/raylib.h:1338:13: ColorBrightness :: Color(Color color, float factor)
    raylib/include/raylib.h:1433:12: DrawGrid :: void(int slices, float spacing)
    raylib/include/raylib.h:1536:12: SetSoundPan :: void(Sound sound, float pan)
    raylib/include/raylib.h:1336:13: ColorFromHSV :: Color(float hue, float saturation, float value)
    raylib/include/raylib.h:1133:12: SetMouseScale :: void(float scaleX, float scaleY)
    raylib/include/raylib.h:1332:11: ColorToInt :: int(Color color)
    raylib/include/raylib.h:1335:15: ColorToHSV :: Vector3(Color color)
    raylib/include/raylib.h:1357:12: IsFontReady :: _Bool(Font font)
    raylib/include/raylib.h:1432:12: DrawRay :: void(Ray ray, Color color)
    raylib/include/raylib.h:1535:12: SetSoundPitch :: void(Sound sound, float pitch)
    raylib/include/raylib.h:1557:12: SetMusicPan :: void(Music music, float pan)
    raylib/include/raylib.h:1342:13: GetColor :: Color(unsigned int hexValue)
    raylib/include/raylib.h:1353:12: LoadFont :: Font(const char * fileName)
    raylib/include/raylib.h:1365:12: DrawFPS :: void(int posX, int posY)
    raylib/include/raylib.h:1467:12: GenMeshPoly :: Mesh(int sides, float radius)
    raylib/include/raylib.h:1534:12: SetSoundVolume :: void(Sound sound, float volume)
