@echo off
rem Convertit les dessins SVG des personnages (crates\afk-game\assets\characters) en planches PNG pour le jeu.
rem   sprites.bat           regenere les PNG et un apercu dans target\sprites-preview
rem   sprites.bat --check   verifie seulement que les PNG correspondent a leur SVG
cargo +stable-x86_64-pc-windows-gnu run -p afk-game --features spritegen --bin spritegen -- %*
