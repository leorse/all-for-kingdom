#!/bin/sh
# Régénère blacksmith.svg (source) et blacksmith.png (atlas chargé par le jeu, rendu à 2x).
# Nécessite python3, node, playwright et Chromium (CHROMIUM_PATH si besoin).
set -e
cd "$(dirname "$0")"
python3 gen_blacksmith.py blacksmith.svg
CHROMIUM_PATH="${CHROMIUM_PATH:-/opt/pw-browsers/chromium-1194/chrome-linux/chrome}" node render_svg.js blacksmith.svg blacksmith.png 2
