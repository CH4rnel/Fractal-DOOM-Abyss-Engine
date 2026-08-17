⛧-Doom-Slayer-⛧

# SCROLL 001 :: FRACTAL COMBAT SYSTEM

**Document Status:** Active  
**Phase:** VI — Fractal Combat  
**Last Updated:** 2026-08-18  
**Module:** `engine/src/gameplay/`

---

## OVERVIEW

The Fractal Combat System implements deterministic combat mechanics where **damage modifies geometry, not just HP**.

This document describes the architecture and API of the gameplay systems responsible for:
- Entity lifecycle management
- Player movement and orientation
- Weapon definitions and damage models
- Fractal-based collision detection
- Geometry deformation through combat
- Procedural enemy navigation

---

## ARCHITECTURE
gameplay/
├── entity.rs       # Base entity system (ID, position, health)
├── player.rs       # Player movement, rotation, jump
├── weapon.rs       # Weapon types, damage, fire rate
├── combat.rs       # Fractal hit zones, raycast
├── damage.rs       # Damage types, geometry deformation
├── navigation.rs   # Pathfinding, line of sight
└── mod.rs          # Module exports


---

## CORE SYSTEMS

### 1. Entity System (`entity.rs`)

Base abstraction for all combat participants (player, demons, projectiles).

#### Types

```rust
pub struct EntityId(pub u64);

pub struct EntityState {
    pub id: EntityId,
    pub position: Vec3,
    pub velocity: Vec3,
    pub health: f64,
    pub max_health: f64,
    pub is_alive: bool,
}

Key Methods

    new(id, position, max_health) — Creates entity with full health
    apply_damage(amount) -> bool — Applies damage, returns true if entity died
    health_fraction() -> f64 — Returns health as 0.0..1.0
    update_movement(delta_time) — Updates position based on velocity

Determinism
Entity state is fully deterministic given:

    Initial position
    Velocity history
    Damage events

    2. Player System (player.rs)
Player entity with FPS-style movement and orientation.
Types

rust
1
2
3
4
5
6
7

Key Methods

    new(position) — Creates player at given position
    forward_direction() -> Vec3 — Returns forward vector based on yaw/pitch
    move_direction(direction, delta_time) — Moves player in direction
    rotate(yaw_delta, pitch_delta) — Rotates player (pitch clamped to ±1.5 rad)
    jump() — Sets vertical velocity

Movement Model
Yaw is measured from the +X axis:

    yaw = 0 → forward = (1, 0, 0)
    yaw = π/2 → forward = (0, 0, 1)

    3. Weapon System (weapon.rs)
Weapon definitions with damage parameters.
Types
pub enum WeaponType {
    Melee,
    Ranged,
    Explosive,
    Fractal,
}

pub struct Weapon {
    pub id: u32,
    pub name: &'static str,
    pub damage: f64,
    pub fire_rate: f64,
    pub range: f64,
    pub weapon_type: WeaponType,
}

Presets

    Weapon::shotgun() — ABYSSAL SHOTGUN (40 dmg, 1.5 rate, 25 range)
    Weapon::chainsaw() — FRACTAL CHAINSAW (15 dmg, 8.0 rate, 3 range)
    Weapon::fractal_rifle() — REALITY FRACTURE RIFLE (25 dmg, 4.0 rate, 50 range)
    Weapon::abyss_grenade() — ABYSS GRENADE (80 dmg, 0.8 rate, 15 range)

Key Methods

    dps() -> f64 — Damage per second
    weapon_type_str() -> &'static str — Stable identifier

    4. Combat System (combat.rs)
Fractal-based collision detection using SDF evaluation.
Types
pub struct FractalHitZone {
    pub center: Vec3,
    pub radius: f64,
}

pub struct RaycastHit {
    pub position: Vec3,
    pub distance: f64,
    pub surface_normal: Vec3,
}

Key Functions

    FractalHitZone::contains_point(point) -> bool — Sphere-based hit test
    raycast(origin, direction, max_distance, world_gen) -> Option<RaycastHit> — Raycast through fractal geometry

Raycast Algorithm

    Step along ray with fixed step size (0.1 units)
    Evaluate SDF at each point
    If SDF ≤ 0 (solid), return hit with computed normal
    Normal computed via SDF gradient (finite differences)

5. Damage System (damage.rs)
Core principle: Damage modifies geometry, not just HP.
Types
pub enum DamageType {
    Physical,    // Modifier: 1.0
    Fractal,     // Modifier: 1.2 + geometry deformation
    Corruption,  // Modifier: 1.5
    Void,        // Modifier: 2.0
}

pub struct DamageEvent {
    pub source: EntityId,
    pub target: EntityId,
    pub amount: f64,
    pub damage_type: DamageType,
    pub position: Vec3,
}

pub struct DamageResult {
    pub target_died: bool,
    pub damage_applied: f64,
    pub geometry_deformation: f64,
}

pub struct GeometryDeformation {
    pub center: Vec3,
    pub radius: f64,
    pub depth: f64,
}

Key Functions

    apply_damage(entity, event) -> DamageResult — Applies damage with type modifiers
    GeometryDeformation::new(impact, damage) — Creates deformation from impact
    GeometryDeformation::evaluate_at(point) -> f64 — Returns SDF modification (negative = carve)

Geometry Deformation Model
Deformation uses smooth falloff:

    Maximum effect at impact center
    Zero effect at radius boundary
    Smooth hermite interpolation: f(t) = t²(3 - 2t)

This deformation is subtracted from the world SDF, effectively carving into fractal geometry.

6. Navigation System (navigation.rs)
Procedural enemy navigation through fractal terrain.
Types
pub struct NavigationPath {
    pub waypoints: Vec<Vec3>,
    pub total_length: f64,
}

Key Functions

    find_path(start, goal, world_gen, max_steps) -> NavigationPath — Finds path avoiding solid geometry
    move_towards(entity, target, speed, delta_time) — Moves entity towards target
    has_line_of_sight(from, to, world_gen, max_distance) -> bool — Checks if path is clear

Pathfinding Algorithm

    Move towards goal in fixed steps (0.5 units)
    If next position is solid (SDF ≤ 0), try perpendicular directions
    If both perpendiculars blocked, stop pathfinding
    Accumulate waypoints and total path length

    INTEGRATION WITH WORLD GENERATOR
All combat systems integrate with WorldGenerator from engine/src/world/:


let kernel = RealityKernel::new(Seed::new(666));
let world_gen = WorldGenerator::new(&kernel);

// Raycast uses world SDF
let hit = raycast(origin, direction, max_distance, &world_gen);

// Navigation checks world solidity
let path = find_path(start, goal, &world_gen, 100);

// Line of sight queries world SDF
let los = has_line_of_sight(from, to, &world_gen, 50.0);

DETERMINISM GUARANTEES
All gameplay systems are fully deterministic:

    Entity state — Deterministic given initial conditions and event history
    Player movement — Deterministic given input sequence
    Weapon damage — Deterministic given weapon type and base damage
    Raycast — Deterministic given world state and ray parameters
    Navigation — Deterministic given world geometry and start/goal

This enables:

    Replay systems
    Multiplayer synchronization
    Anti-cheat validation
    Deterministic bug reproduction

    TESTING
All systems include unit tests verifying:

    Entity lifecycle (spawn, damage, death)
    Player movement and rotation bounds
    Weapon DPS calculations
    Damage type modifiers
    Geometry deformation falloff
    Pathfinding correctness
    Line of sight accuracy

    Run tests: cargo test gameplay

QUALITY GATES
Every combat subsystem passes:
cargo fmt
cargo check
cargo test
cargo clippy -- -D warnings

Zero warnings. Zero errors. Zero technical debt.

FUTURE EXTENSIONS
Planned additions to the combat system:

    Fractal projectiles — Recursive projectile splitting
    Environmental combat — Destructible fractal structures
    Enemy mutation — Dynamic behavior changes based on damage
    Boss encounters — Multi-phase fractal boss fights
    Weapon upgrades — Procedural weapon modification

    RELATED DOCUMENTS

    docs/architecture/000-ABYSS-ARCHITECTURE.md — Overall engine architecture
    docs/roadmap/ — Development roadmap and phase planning
    Fractal-DOOM-Abyss-Engine_PROJECT_MASTER_ROADMAP.md — Master roadmap

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║  COMBAT SYSTEM STATUS: OPERATIONAL                                           ║
║  GEOMETRY MODIFICATION: ENABLED                                              ║
║  DETERMINISM: VERIFIED                                                       ║
║  REALITY STABILITY: 87%                                                      ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝