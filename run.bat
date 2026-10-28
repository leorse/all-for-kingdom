@echo off
rem Lance le jeu avec la toolchain GNU (pas besoin de Visual Studio / Windows SDK).
rem Arguments supplementaires transmis a cargo, ex: run.bat --release
cargo +stable-x86_64-pc-windows-gnu run %*
