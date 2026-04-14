# Weapons

## Types d'armes

- **Projectile** : Tire des projectiles en ligne (droite ou multi-directions)
- **AoE** : Zone d'effet instantanée ou persistante
- **Orbital** : Objets tournant autour du joueur avec trajectoires non-linéaires

## Structure Rust

```rust
#[derive(Deserialize, Clone)]
struct WeaponConfig {
    name: String,
    levels: Vec<WeaponLevel>,
}

#[derive(Deserialize, Clone)]
struct WeaponLevel {
    level: u8,
    stats: WeaponStats,
    attack: WeaponAttack,
}

#[derive(Deserialize, Clone)]
struct WeaponStats {
    damage: f32,
    energy_cost: f32,
    fire_rate: f32,
}

#[derive(Deserialize, Clone)]
enum WeaponAttack {
    Projectile {
        sprite: String,
        speed: f32,
        lifetime: f32,
        trajectories: Vec<Trajectory>,
        on_hit: Option<OnHitEffect>,
    },
    Aoe {
        sprite: String,
        radius: f32,
        duration: f32,
    },
    Orbital {
        sprite: String,
        count: u32,
        damage: f32,
        orbit_pattern: OrbitPattern,
    },
}

#[derive(Deserialize, Clone)]
struct Trajectory {
    angle_offset: f32,
}

#[derive(Deserialize, Clone)]
enum OnHitEffect {
    Explosion { radius: f32, damage: f32 },
    Pierce { max_targets: u32 },
    Slow { duration: f32, percentage: f32 },
}

#[derive(Deserialize, Clone)]
enum OrbitPattern {
    Circle {
        radius: f32,
        rotation_speed: f32,
        clockwise: bool,
    },
    Spiral {
        min_radius: f32,
        max_radius: f32,
        rotation_speed: f32,
        expansion_speed: f32,
    },
    FigureEight {
        width: f32,
        height: f32,
        period: f32,
    },
    Ellipse {
        width: f32,
        height: f32,
        rotation_speed: f32,
    },
}
```

## Évolution par niveaux

Chaque niveau peut modifier :
- Stats (damage, energy_cost, fire_rate)
- Type d'attaque complet (Projectile → AoE, etc.)
- Nombre de trajectoires/projectiles
- Pattern orbital
- Effets on_hit

## Notes

- Armes en tir automatique (fire_rate > 0)
- Orbitaux permanents (fire_rate = 0, coût énergie par seconde)
- 2 slots d'armes maximum
- Slots définitifs pendant la run
