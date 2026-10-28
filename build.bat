@echo off
rem Build avec la toolchain GNU (pas besoin de Visual Studio / Windows SDK).
rem Arguments supplementaires transmis a cargo, ex: build.bat --release
cargo +stable-x86_64-pc-windows-gnu build %*
