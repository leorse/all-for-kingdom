# Proposal

## Why

L'enclume est dessinée une demi-case plus haut que sa collision, et elle est triée comme si elle s'arrêtait au bas de sa case. Du coup, un personnage dont les pieds sont déjà plus bas que le socle passe quand même derrière elle, et l'enclume paraît bien plus grande que lui. Son poste, au nord, oblige aussi le forgeron à se tenir loin du socle, et ses images de forge ont les jambes coupées : il semble coupé en deux.

## What Changes

- L'enclume occupe la moitié basse de sa case : son socle est dessiné sur cette moitié, et la moitié haute devient praticable.
- Le dessin, la collision et l'ordre d'affichage se calent sur le socle. Celui dont les pieds sont plus bas que le haut du socle passe devant l'enclume, les autres derrière.
- Le poste de forge passe dans la moitié libre de la case : le forgeron forge les pieds contre le socle. Le joueur qui se colle au socle est déjà à la bonne hauteur.
- Les images de forge retrouvent des jambes entières. Le pied de l'enclume cache ses bottes quand il est au poste.
- Les trajets restent calculés par case : la case de l'enclume reste un obstacle pour l'IA, qui n'y entre que pour rejoindre le poste. Un personnage qui se trouve dans cette moitié libre sait en repartir.
- **Hors périmètre :** les coffres, les étagères et les autres objets, qui seront traités par des changements ultérieurs. La taille de l'enclume et la hitbox des personnages ne changent pas.

## Capabilities

### New Capabilities

Aucune.

### Modified Capabilities

- `blacksmith` : l'enclume n'occupe plus que la moitié basse de sa case, le poste de forge se place contre le socle, et le forgeron est dessiné entier pendant la forge.

## Impact

- `crates/afk-core/src/map.rs` : la collision de l'enclume et le départ des trajets depuis sa case.
- `crates/afk-core/src/character.rs` et `game.rs` : le poste de forge, et les contrôles qui supposent qu'il est sur la case au nord.
- `crates/afk-game/src/render.rs` et `character_view.rs` : la position du dessin de l'enclume, l'ordre d'affichage et l'ombre.
- `crates/afk-game/assets/characters/forgeron.svg` et `forgeron.png` : les 6 images `travail`.
- `docs/art-direction.md` et `docs/gameplay/forgeron.md`.
