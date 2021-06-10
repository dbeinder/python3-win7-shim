@echo off
REM   Make sure to run rustup install stable-i686-pc-windows-msvc
REM                    rustup install stable-x86_64-pc-windows-msvc

rmdir 64bit /s /q
rmdir 32bit /s /q
mkdir 64bit
mkdir 32bit
rustup default stable-i686-pc-windows-msvc
rustc --target i686-pc-windows-msvc -C panic=abort -C target-feature=+crt-static -o 32bit/api-ms-win-core-path-l1-1-0.dll lib.rs
rustup default stable-x86_64-pc-windows-msvc
rustc --target x86_64-pc-windows-msvc -C panic=abort -C target-feature=+crt-static -o 64bit/api-ms-win-core-path-l1-1-0.dll lib.rs