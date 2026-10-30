---
type: Module
title: Ressources, coffres et transport
description: Les sept ressources, les contenants (coffres) de capacité fixe, les transporteurs qui choisissent leurs voyages.
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

- **Ressources** (`Resource`) : Wheat, Wood, Iron, Copper, Gold, Stone, Tools. `Stock` est le total affiché en haut à droite de l'écran.
- **Contenants** (`Container`, `ContainerKind` : Farm, Mine(i), Lodge(i), Forge, Depot) : un coffre par lieu de travail, posé dehors (tuile `Crate`), de capacité `container_capacity` par ressource. Un contenant sait ce qu'il produit (`product`), ce qu'il consomme (`consumes`), s'il est l'entrepôt central (`is_depot`) et ce qu'il accepte (`accepts`). Coffre plein = le producteur s'arrête (voir [ressources et stocks](/regles/ressources-et-stocks.md)).
- **Transport** : les transporteurs choisissent seuls leur voyage (`best_trip`) et leur destination (`best_destination`). Une `Reservation` évite que plusieurs transporteurs visent la même place restante. Un transporteur ne part que pour une charge complète (`transport_load`) ou pour remplir exactement la place restante. Le fer des mines va d'abord à la forge, le reste à l'entrepôt central, qui rend du fer à la forge si elle en manque (voir [le transporteur](/metiers/transporteur.md)).
- Le mineur et le forgeron déposent directement dans le coffre de leur lieu ; le paysan et le bûcheron portent à la main (`farmer_carry`, `lumberjack_carry`).
