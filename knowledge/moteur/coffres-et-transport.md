---
type: Module
title: Ressources, coffres et transport
description: Les huit ressources, les contenants (coffres, coffre du camp, entrepôt, tas de chantier), les transporteurs qui choisissent leurs voyages.
tags: [moteur, ressources, coffres, transport]
sources:
  - resource: crates/afk-core/src/resources.rs
    title: Resource et Stock
  - resource: crates/afk-core/src/container.rs
    title: Container et ContainerKind
  - resource: crates/afk-core/src/transport.rs
    title: best_trip, best_destination, Reservation
generated: { by: claude-code/claude-sonnet-5-5, at: 2026-10-30T12:00:00Z }
---

# Ressources, coffres et transport

- **Ressources** (`Resource`) : Wheat, Seeds, Wood, Iron, Copper, Gold, Stone, Tools (les graines n'ont pas encore de consommateur). `Stock` est le total affiché en haut à droite de l'écran.
- **Contenants** (`Container`, `ContainerKind` : Farm, Mine(i), Lodge(i), Forge, Depot, Camp, Site(i)) : un coffre par lieu de travail, posé dehors (tuile `Crate`), de capacité `container_capacity` par ressource. Un contenant sait ce qu'il produit (`product`), ce qu'il consomme (`consumes`), s'il est l'entrepôt central (`is_depot`) et ce qu'il accepte (`accepts`). Coffre plein = le producteur s'arrête (voir [ressources et stocks](/regles/ressources-et-stocks.md)).
- **Transport** : les transporteurs choisissent seuls leur voyage (`best_trip`) et leur destination (`best_destination`). Une `Reservation` évite que plusieurs transporteurs visent la même place restante. Un transporteur ne part que pour une charge complète (`transport_load`) ou pour remplir exactement la place restante. Le fer des mines va d'abord à la forge, le reste à l'entrepôt central, qui rend du fer à la forge si elle en manque (voir [le transporteur](/metiers/transporteur.md)).
- Le mineur et le forgeron déposent directement dans le coffre de leur lieu ; le paysan et le bûcheron portent à la main (`farmer_carry`, `lumberjack_carry`).
- **Le coffre du camp** (`Camp`) et **l'entrepôt bâti par le roi** (`Depot`) reçoivent tout ce que personne ne consomme (`is_depot`) ; leur capacité est `camp_capacity` (100) et `warehouse_capacity` (grande). Le coffre du camp reçoit le pécule de départ.
- **Le tas d'un chantier** (`Site(i)`, `Container::new_site`) ne reçoit que ce qu'il lui faut (`needs`, `room`) : `wants(resource)` est vrai tant qu'il manque cette ressource, et le planificateur de transport le traite comme un consommateur. `is_complete` dit que tout est livré. Un tas fini est `retire` (il reste dans la liste, les indices ne bougent jamais) ou devient le coffre d'un entrepôt (`become_depot`). Voir [construire](/regles/construction.md).
