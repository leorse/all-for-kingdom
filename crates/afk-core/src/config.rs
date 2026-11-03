//! Réglages de la partie. Tout ce qui est "à équilibrer" vit ici, au même endroit.
//!
//! Les durées de fatigue, de sommeil et de pousse sont exprimées en **heures de jeu** : si on
//! raccourcit la journée, le rythme de vie et les cultures restent cohérents avec le cycle jour/nuit.
//! Les vitesses de déplacement et de travail, elles, restent en secondes réelles.

#[derive(Clone, Debug)]
pub struct Config {
    /// Durée d'une journée complète de 24 h de jeu, en secondes réelles (600 = 10 minutes).
    pub day_length_secs: f32,
    /// Heure de jeu au démarrage de la partie.
    pub start_hour: f32,
    /// Heure du lever du jour.
    pub dawn_hour: f32,
    /// Heure du coucher du soleil.
    pub dusk_hour: f32,

    /// Fatigue gagnée par heure de jeu en marchant ou en travaillant.
    pub fatigue_active_per_hour: f32,
    /// Fatigue gagnée par heure de jeu à l'arrêt (éveillé).
    pub fatigue_idle_per_hour: f32,
    /// Fatigue perdue par heure de jeu en dormant.
    pub sleep_recovery_per_hour: f32,
    /// Au-dessus de ce niveau, l'IA va se coucher même en plein jour.
    pub exhausted_fatigue: f32,
    /// En journée, on se réveille quand la fatigue passe sous ce niveau.
    pub wake_fatigue: f32,

    /// Distance (en tuiles) à laquelle on peut se coucher dans son lit.
    pub bed_reach: f32,

    // --- travail
    /// Facteur de vitesse de travail du joueur par rapport à l'IA (2.0 = deux fois plus vite).
    pub player_work_speed: f32,
    /// Durée (secondes réelles, pour l'IA en pleine forme) pour labourer une parcelle.
    pub till_secs: f32,
    /// Durée pour planter une parcelle.
    pub plant_secs: f32,
    /// Durée pour récolter une parcelle.
    pub harvest_secs: f32,
    /// Temps de pousse d'une culture, en heures de jeu. Personne ne peut l'accélérer.
    pub crop_growth_hours: f32,
    /// Blé obtenu par parcelle récoltée.
    pub wheat_per_harvest: u32,

    // --- forge
    /// Fer dans le coffre de la forge au début de la partie.
    pub starting_iron: u32,
    /// Fer consommé pour forger un outil.
    pub iron_per_tool: u32,
    /// Outils obtenus à chaque coup de forge réussi.
    pub tools_per_forge: u32,
    /// Durée (secondes réelles, pour l'IA en pleine forme) pour forger un outil.
    pub forge_secs: f32,

    // --- mine
    /// Coups de pioche par unité de minerai, pour une roche pure, moyenne et impure.
    pub strikes_per_quality: [u32; 3],
    /// Durée d'un coup de pioche (secondes réelles, pour l'IA en pleine forme).
    pub strike_secs: f32,

    // --- forêt
    /// Durée de référence pour abattre un arbre (secondes réelles, un bûcheron IA en pleine forme
    /// seul ; à plusieurs, les coups s'additionnent).
    pub chop_secs: f32,
    /// Bûches obtenues quand un arbre tombe.
    pub logs_per_tree: u32,
    /// Temps de repousse d'un arbre abattu, en heures de jeu.
    pub tree_regrow_hours: f32,

    // --- contenants et transport
    /// Quantité maximale de chaque ressource dans un contenant.
    pub container_capacity: u32,
    /// Ressources qu'un transporteur porte par voyage.
    pub transport_load: u32,
    /// Vitesse des transporteurs par rapport aux autres personnages (ils courent).
    pub transporter_speed_factor: f32,
    /// Vitesse du roi quand le joueur l'incarne, par rapport aux autres : il traverse la carte pour placer ses plans.
    pub king_speed_factor: f32,
    /// Bûches qu'un bûcheron peut porter à la main.
    pub lumberjack_carry: u32,
    /// Blé qu'un paysan peut porter à la main.
    pub farmer_carry: u32,

    // --- le départ de partie (le camp)
    /// Capacité (par ressource) du coffre du camp de base.
    pub camp_capacity: u32,
    /// Le pécule de départ, dans le coffre du camp.
    pub starting_wood: u32,
    pub starting_stone: u32,
    pub starting_seeds: u32,

    // --- construction : un réglage par type de bâtiment, dans l'ordre de `PlanKind::ALL` (maison, entrepôt, taverne, forge)
    /// Bois et pierre par tuile de surface (murs compris).
    pub plan_wood_per_tile: [f32; 4],
    pub plan_stone_per_tile: [f32; 4],
    /// Ce que coûtent les assets de base, compris dans le prix et posés automatiquement.
    pub plan_asset_wood: [u32; 4],
    pub plan_asset_stone: [u32; 4],
    /// Durée de construction par tuile, pour un artisan en pleine forme (secondes réelles).
    pub plan_build_secs_per_tile: [f32; 4],
    /// Durée supplémentaire pour poser les assets de base.
    pub plan_asset_secs: [f32; 4],
    /// Un lit de plus dans une maison : bois et durée.
    pub extra_bed_wood: u32,
    pub extra_bed_secs: f32,
    /// Capacité (par ressource) d'un entrepôt bâti par le roi.
    pub warehouse_capacity: u32,

    // --- parler, postes
    /// Distance (en tuiles) à laquelle le roi peut s'adresser à quelqu'un.
    pub talk_range: f32,
    /// Distance à laquelle un vagabond ralentit en présence du roi, et son facteur de vitesse alors.
    pub slow_range: f32,
    pub vagabond_slow_factor: f32,
    /// Postes de transporteur d'un entrepôt.
    pub warehouse_posts: u32,

    // --- les vagabonds
    /// Heures de jeu avant le premier vagabond, puis entre deux arrivées.
    pub vagabond_first_hours: f32,
    pub vagabond_every_hours: f32,
    /// Combien en même temps, au plus.
    pub vagabond_max: usize,
    /// Heures de jeu qu'un vagabond reste dans les parages avant de repartir (quatre fois plus s'il y a une taverne).
    pub vagabond_stay_hours: f32,

    // --- déplacements
    /// Surcoût de l'herbe par rapport à la route pour les trajets ordinaires (la route coûte 1) :
    /// plus il est haut, plus les habitants s'accrochent aux routes.
    pub grass_cost: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            day_length_secs: 600.0,
            start_hour: 7.0,
            dawn_hour: 6.0,
            dusk_hour: 20.0,
            fatigue_active_per_hour: 5.0,
            fatigue_idle_per_hour: 2.5,
            sleep_recovery_per_hour: 12.0,
            exhausted_fatigue: 85.0,
            wake_fatigue: 15.0,
            bed_reach: 1.5,
            player_work_speed: 2.0,
            till_secs: 4.0,
            plant_secs: 3.0,
            harvest_secs: 4.0,
            crop_growth_hours: 12.0,
            wheat_per_harvest: 4,
            starting_iron: 60,
            iron_per_tool: 2,
            tools_per_forge: 1,
            forge_secs: 8.0,
            strikes_per_quality: [3, 5, 8],
            strike_secs: 1.0,
            chop_secs: 6.0,
            logs_per_tree: 3,
            tree_regrow_hours: 24.0,
            container_capacity: 200,
            transport_load: 10,
            transporter_speed_factor: 1.5,
            king_speed_factor: 1.6,
            lumberjack_carry: 9,
            farmer_carry: 20,
            camp_capacity: 100,
            starting_wood: 90,
            starting_stone: 60,
            starting_seeds: 12,
            plan_wood_per_tile: [0.5, 0.4, 0.6, 0.5],
            plan_stone_per_tile: [0.2, 0.5, 0.3, 0.6],
            plan_asset_wood: [8, 8, 14, 10],
            plan_asset_stone: [2, 0, 3, 8],
            plan_build_secs_per_tile: [1.6, 1.2, 2.0, 1.8],
            plan_asset_secs: [8.0, 6.0, 12.0, 14.0],
            extra_bed_wood: 4,
            extra_bed_secs: 5.0,
            warehouse_capacity: 1000,
            talk_range: 2.0,
            slow_range: 3.0,
            vagabond_slow_factor: 0.4,
            warehouse_posts: 2,
            vagabond_first_hours: 2.0,
            vagabond_every_hours: 5.0,
            vagabond_max: 3,
            vagabond_stay_hours: 10.0,
            grass_cost: crate::map::DEFAULT_GRASS_COST,
        }
    }
}
