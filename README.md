# python3-win7-shim
This is a shim DLL to run unpatched Python3.8+ on Windows 7. This built specifically for Python and is not a fully featured, general purpose replacement for `api-ms-win-core-path-l1-1-0.dll`.

[Download compiled DLLs (x86, x64)](https://github.com/dbeinder/python3-win7-shim/releases/download/v0.1/python3-win7-shim.zip)

It passes through these functions, and only works with `PATHCCH_NONE` flags:<br>
`PathCchCanonicalizeEx() => PathCanonicalizeW()`<br>
`PathCchCombineEx() => PathCombineW()`
