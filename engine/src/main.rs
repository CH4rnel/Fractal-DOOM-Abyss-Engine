//! ⛧-Doom-Slayer-⛧
//! Fractal DOOM: Abyss Engine - Execution Harness

mod core;
mod demon;
mod fractal;
mod gameplay;
mod world;

use core::abyss::MutationRecord;
use core::random::RandomDomain;
use core::reality::RealityKernel;
use core::seed::Seed;
use demon::{DemonIdentity, DemonSeed, ThreatLevel};
use fractal::{sdf_sphere, Scene, Vec3};
use gameplay::{
    find_path, has_line_of_sight, move_towards, raycast, DamageEvent, DamageResult, DamageType,
    EntityId, EntityState, FractalHitZone, GeometryDeformation, NavigationPath, Player,
    RaycastHit, Weapon, WeaponType,
};
use world::{
    Biome, Chunk, ChunkCoord, ChunkEvaluation, ChunkState, StreamUpdate, WorldGenerator,
    WorldStreamer, CHUNK_SIZE,
};

fn main() {
    println!("══════════════════════════════════════════════");
    println!("        FRACTAL DOOM :: ABYSS ENGINE");
    println!("══════════════════════════════════════════════");

    let mut reality = RealityKernel::new(Seed::default());

    println!("[ INIT ] Seed: {}", reality.seed());
    println!("[ INIT ] Tick: {}", reality.tick());

    // --- Phase II: State Machine ---
    reality.descend();
    reality.corrupt(0.4);
    reality.fracture();

    println!("[ KERNEL ] State: {}", reality.state().as_str());
    println!("[ KERNEL ] Depth: {}", reality.depth());
    println!("[ KERNEL ] Entropy: {:.2}", reality.entropy());
    println!("[ KERNEL ] Corruption: {:.2}", reality.corruption());
    println!("[ KERNEL ] Stability: {:.2}", reality.stability());

    // --- Phase III: Fractal Raymarching ---
    println!();
    println!("═══════ FRACTAL RAYMARCHING TEST ═══════");

    let ray_origin = Vec3::new(-3.0, 0.0, 0.0);
    let ray_dir = Vec3::new(1.0, 0.0, 0.0);
    let mut t = 0.0;
    let max_steps = 64;
    let mut hit = false;

    for _ in 0..max_steps {
        let pos = ray_origin + ray_dir * t;
        let d = Scene::evaluate(pos);
        if d < 0.001 {
            hit = true;
            break;
        }
        t += d;
        if t > 10.0 {
            break;
        }
    }

    if hit {
        println!("[ RAY ] HIT Mandelbulb surface at distance {:.4}", t);
    } else {
        println!("[ RAY ] MISSED. Escaped to infinity.");
    }

    let sphere_d = sdf_sphere(Vec3::new(5.0, 0.0, 0.0), Vec3::zero(), 1.0);
    println!("[ SDF ] Sphere distance: {:.4}", sphere_d);

    // --- Phase IV: World Generator ---
    println!();
    println!("═══════ ABYSS WORLD GENERATOR ═══════");
    println!("[ WORLD ] Chunk Size: {:.1} units", CHUNK_SIZE);

    let generator = WorldGenerator::new(&reality);
    let biome: Biome = generator.biome();
    println!("[ WORLD ] Current Biome: {}", biome.as_str());
    println!(
        "[ WORLD ] Fractal Iterations: {}",
        biome.fractal_iterations()
    );
    println!("[ WORLD ] Fractal Power: {:.1}", biome.fractal_power());
    println!(
        "[ WORLD ] Corruption Intensity: {:.2}",
        biome.corruption_intensity()
    );

    // Chunk lifecycle
    println!();
    println!("═══════ CHUNK LIFECYCLE ═══════");

    let coord = ChunkCoord::new(0, 0, 0);
    let mut chunk = Chunk::new(coord, reality.seed());

    println!(
        "[ CHUNK ] Created at ({},{},{})",
        chunk.coord.x, chunk.coord.y, chunk.coord.z
    );
    println!("[ CHUNK ] Seed: {}", chunk.seed);
    println!("[ CHUNK ] Initial state: {:?}", chunk.state);

    chunk.state = ChunkState::Loading;
    println!("[ CHUNK ] State: {:?}", chunk.state);

    chunk.activate();
    println!("[ CHUNK ] State after activate(): {:?}", chunk.state);

    chunk.corrupt();
    println!("[ CHUNK ] State after corrupt(): {:?}", chunk.state);

    chunk.state = ChunkState::Unloading;
    println!("[ CHUNK ] State: {:?}", chunk.state);

    let is_present = matches!(chunk.state, ChunkState::Active | ChunkState::Corrupted);
    println!("[ CHUNK ] Present in world: {}", is_present);

    // Chunk evaluation
    println!();
    println!("═══════ CHUNK EVALUATION ═══════");

    let eval_coord = ChunkCoord::new(1, 0, 0);
    let mut eval_chunk = Chunk::new(eval_coord, reality.seed());
    eval_chunk.activate();

    let evaluation: ChunkEvaluation = generator.evaluate_chunk(&eval_chunk);
    println!(
        "[ EVAL ] Chunk ({},{},{})",
        evaluation.coord.x, evaluation.coord.y, evaluation.coord.z
    );
    println!("[ EVAL ] Biome: {}", evaluation.biome.as_str());
    println!("[ EVAL ] Center SDF: {:.4}", evaluation.center_sdf);
    println!("[ EVAL ] Center Density: {:.2}", evaluation.center_density);
    println!("[ EVAL ] Mostly Solid: {}", evaluation.is_mostly_solid);

    // Free point evaluation
    println!();
    println!("═══════ FREE POINT EVALUATION ═══════");

    let test_points = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(5.0, 5.0, 5.0),
        Vec3::new(10.0, 0.0, 0.0),
    ];

    for (i, pos) in test_points.iter().enumerate() {
        let sdf = generator.evaluate_sdf(*pos);
        let density = generator.density(*pos);
        let solid = if generator.is_solid(*pos) { "SOLID" } else { "VOID" };
        println!(
            "[ WORLD ] Point {}: ({:.1}, {:.1}, {:.1}) -> SDF: {:.4}, Density: {:.2}, {}",
            i, pos.x, pos.y, pos.z, sdf, density, solid
        );
    }

    // Chunk streaming
    println!();
    println!("═══════ CHUNK STREAMING ═══════");

    let mut streamer = WorldStreamer::new(reality.seed(), 1);
    let player_pos = Vec3::new(0.0, 0.0, 0.0);
    let update: StreamUpdate = streamer.update(player_pos);

    println!(
        "[ STREAM ] Player at: ({:.1}, {:.1}, {:.1})",
        player_pos.x, player_pos.y, player_pos.z
    );
    println!("[ STREAM ] Chunks loaded: {}", update.loaded);
    println!("[ STREAM ] Chunks unloaded: {}", update.unloaded);
    println!(
        "[ STREAM ] Total active chunks: {}",
        streamer.loaded_count()
    );

    let target_coord = ChunkCoord::new(0, 0, 0);
    if let Some(retrieved_chunk) = streamer.get_chunk(&target_coord) {
        println!(
            "[ STREAM ] Retrieved chunk ({},{},{}) state: {:?}",
            retrieved_chunk.coord.x,
            retrieved_chunk.coord.y,
            retrieved_chunk.coord.z,
            retrieved_chunk.state
        );
    }

    let query_pos = Vec3::new(8.0, 8.0, 8.0);
    let sdf_at_query = streamer.evaluate_at(query_pos, &reality);
    println!(
        "[ STREAM ] World SDF at ({:.1}, {:.1}, {:.1}): {:.4}",
        query_pos.x, query_pos.y, query_pos.z, sdf_at_query
    );

    // --- Phase V: Demon Seed Generator ---
    println!();
    println!("══════════════════════════════════════════════");
    println!("       ⛧ DEMON SEED GENERATOR ⛧");
    println!("══════════════════════════════════════════════");

    let demon_coord = ChunkCoord::new(3, 5, 2);
    let demon_tick = reality.tick();
    let demon_seed = DemonSeed::new(reality.seed(), demon_coord, demon_tick);

    println!("[ DEMON ] Identity Derivation:");
    println!("  Universe:    {}", demon_seed.universe());
    println!(
        "  Coordinate:  ({}, {}, {})",
        demon_seed.coord().x,
        demon_seed.coord().y,
        demon_seed.coord().z
    );
    println!("  Spawn Tick:  {}", demon_seed.spawn_tick());
    println!("  Derived ID:  {}", demon_seed.derive_identity());

    let identity = DemonIdentity::reconstruct(demon_seed);

    println!();
    println!("[ DEMON ] Anatomy:");
    println!("  Body Scale:  {:.2}", identity.anatomy.body_scale);
    println!("  Limb Count:  {}", identity.anatomy.limb_count);
    println!("  Limb Length: {:.2}", identity.anatomy.limb_length);
    println!("  Torso:       {}", identity.anatomy.torso_shape_str());
    println!("  Symmetry:    {}", identity.anatomy.symmetry_str());

    println!();
    println!("[ DEMON ] Behavior:");
    println!("  Movement:    {}", identity.behavior.movement_str());
    println!("  Aggression:  {:.2}", identity.behavior.aggression);
    println!("  Speed:       {:.2}", identity.behavior.speed);
    println!("  Perception:  {:.1}", identity.behavior.perception_range);
    println!("  Attack:      {}", identity.behavior.attack_str());

    println!();
    println!("[ DEMON ] Mutation:");
    println!("  Level:       {}", identity.mutation.mutation_level);
    println!("  Type:        {}", identity.mutation.mutation_type_str());
    println!("  Material:    {}", identity.mutation.material_str());
    println!("  Audio:       {}", identity.mutation.audio_str());
    println!("  Color Hue:   {:.1}", identity.mutation.color_hue);

    println!();
    println!("[ DEMON ] Genealogy:");
    println!("  Recursion:   {}", identity.genealogy.recursion_depth);
    println!("  Lineage:     {}", identity.genealogy.lineage_str());

    let threat: ThreatLevel = identity.overall_threat();
    println!("[ DEMON ] Threat:      {}", threat.as_str());

    let identity_again = DemonIdentity::reconstruct(demon_seed);
    let deterministic = identity == identity_again;
    println!();
    println!("[ DEMON ] Determinism check: {}", deterministic);

    let demon_seed_2 = DemonSeed::new(
        reality.seed(),
        ChunkCoord::new(7, 12, 4),
        demon_tick + 100,
    );
    let identity_2 = DemonIdentity::reconstruct(demon_seed_2);
    println!(
        "[ DEMON ] Second demon threat: {}",
        identity_2.overall_threat().as_str()
    );

    // --- Phase VI: Fractal Combat ---
    println!();
    println!("══════════════════════════════════════════════");
    println!("       ⛧ FRACTAL COMBAT SYSTEM ⛧");
    println!("══════════════════════════════════════════════");

    // Player
    let mut player = Player::new(Vec3::new(0.0, 0.0, 0.0));
    println!(
        "[ PLAYER ] Spawned at ({:.1}, {:.1}, {:.1})",
        player.entity.position.x, player.entity.position.y, player.entity.position.z
    );
    println!(
        "[ PLAYER ] Health: {:.0}/{:.0} ({:.2})",
        player.entity.health,
        player.entity.max_health,
        player.entity.health_fraction()
    );

    player.rotate(0.5, 0.1);
    println!(
        "[ PLAYER ] Rotated: yaw={:.2}, pitch={:.2}",
        player.yaw, player.pitch
    );

    let forward = player.forward_direction();
    println!(
        "[ PLAYER ] Forward: ({:.2}, {:.2}, {:.2})",
        forward.x, forward.y, forward.z
    );

    player.move_direction(Vec3::new(1.0, 0.0, 0.0), 0.5);
    println!(
        "[ PLAYER ] Moved to: ({:.1}, {:.1}, {:.1})",
        player.entity.position.x, player.entity.position.y, player.entity.position.z
    );

    player.jump();
    println!("[ PLAYER ] Jump velocity: {:.1}", player.entity.velocity.y);

    // Weapons
    println!();
    println!("[ WEAPON ] Arsenal:");
    for w in [
        Weapon::shotgun(),
        Weapon::chainsaw(),
        Weapon::fractal_rifle(),
        Weapon::abyss_grenade(),
    ] {
        println!(
            "  {} | {} | DMG:{:.0} | DPS:{:.1}",
            w.name,
            w.weapon_type_str(),
            w.damage,
            w.dps()
        );
    }
    let fractal_type: WeaponType = WeaponType::Fractal;
    println!(
        "[ WEAPON ] Rifle matches fractal type: {}",
        Weapon::fractal_rifle().weapon_type == fractal_type
    );

    // Combat entities
    println!();
    println!("[ COMBAT ] Entity simulation:");
    let mut demon = EntityState::new(EntityId::new(666), Vec3::new(5.0, 0.0, 5.0), 100.0);
    println!("  Demon HP: {:.0}/{:.0}", demon.health, demon.max_health);

    let dmg_event = DamageEvent::new(
        EntityId::new(0),
        EntityId::new(666),
        35.0,
        DamageType::Fractal,
        Vec3::new(5.0, 0.0, 5.0),
    );
    let dmg_result: DamageResult = gameplay::damage::apply_damage(&mut demon, &dmg_event);
    println!("  Applied {:.1} fractal damage", dmg_result.damage_applied);
    println!(
        "  Geometry deformation: {:.3}",
        dmg_result.geometry_deformation
    );
    println!("  Target died: {}", dmg_result.target_died);
    println!(
        "  Demon HP: {:.0}/{:.0} ({:.2}), alive: {}",
        demon.health,
        demon.max_health,
        demon.health_fraction(),
        demon.is_alive
    );

    // Demonstrate all damage types and their modifiers
    println!();
    println!("[ DAMAGE ] Damage type modifiers:");
    for dtype in [
        DamageType::Physical,
        DamageType::Fractal,
        DamageType::Corruption,
        DamageType::Void,
    ] {
        let mut dummy = EntityState::new(EntityId::new(100), Vec3::new(0.0, 0.0, 0.0), 1000.0);
        let ev = DamageEvent::new(
            EntityId::new(0),
            EntityId::new(100),
            10.0,
            dtype,
            Vec3::new(0.0, 0.0, 0.0),
        );
        let res: DamageResult = gameplay::damage::apply_damage(&mut dummy, &ev);
        println!("  {:?}: base 10.0 -> applied {:.1}", dtype, res.damage_applied);
    }

    // Geometry deformation
    println!();
    println!("[ GEOMETRY ] Deformation:");
    let deform = GeometryDeformation::new(Vec3::new(5.0, 0.0, 5.0), 100.0);
    println!(
        "  At center: {:.3}",
        deform.evaluate_at(Vec3::new(5.0, 0.0, 5.0))
    );
    println!(
        "  At edge: {:.3}",
        deform.evaluate_at(Vec3::new(10.0, 0.0, 5.0))
    );

    // Fractal hit zones
    println!();
    println!("[ COLLISION ] Fractal hit zones:");
    let zone = FractalHitZone::new(Vec3::new(5.0, 0.0, 5.0), 3.0);
    println!(
        "  (5.5,0,5) inside: {}",
        zone.contains_point(Vec3::new(5.5, 0.0, 5.0))
    );
    println!(
        "  (10,0,10) inside: {}",
        zone.contains_point(Vec3::new(10.0, 0.0, 10.0))
    );

    // Raycast
    println!();
    println!("[ COLLISION ] Raycast:");
    let ray: Option<RaycastHit> = raycast(
        Vec3::new(-5.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        20.0,
        &generator,
    );
    match ray {
        Some(r) => {
            println!("  HIT at distance {:.2}", r.distance);
            println!(
                "  Position: ({:.2}, {:.2}, {:.2})",
                r.position.x, r.position.y, r.position.z
            );
            println!(
                "  Normal: ({:.2}, {:.2}, {:.2})",
                r.surface_normal.x, r.surface_normal.y, r.surface_normal.z
            );
        }
        None => println!("  NO HIT within range"),
    }

    // Navigation
    println!();
    println!("[ NAVIGATION ] Pathfinding:");
    let path: NavigationPath = find_path(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        &generator,
        50,
    );
    println!(
        "  Waypoints: {}, Length: {:.2}, Empty: {}",
        path.waypoints.len(),
        path.total_length,
        path.is_empty()
    );

    let mut mover = EntityState::new(EntityId::new(999), Vec3::new(0.0, 0.0, 0.0), 50.0);
    move_towards(&mut mover, Vec3::new(10.0, 0.0, 0.0), 5.0, 1.0);
    println!(
        "  Mover at: ({:.1}, {:.1}, {:.1})",
        mover.position.x, mover.position.y, mover.position.z
    );

    let los = has_line_of_sight(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(5.0, 0.0, 0.0),
        &generator,
        20.0,
    );
    println!("  Line of sight: {}", los);

    // --- Mutation Log ---
    println!();
    println!("═══════ MUTATION LOG ═══════");
    let history = reality.log().history();
    println!(
        "Total events: {} (Empty: {})",
        reality.log().len(),
        reality.log().is_empty()
    );

    for record in history {
        let r: &MutationRecord = record;
        println!(
            "TICK {:04} | [{}] {} -> {} | ΔE: {:.2} ΔC: {:.2}",
            r.tick,
            r.event.as_str(),
            r.previous_state.as_str(),
            r.new_state.as_str(),
            r.entropy_delta,
            r.corruption_delta
        );
    }

    // --- Domain Manifest ---
    println!();
    println!("═══════ DOMAIN MANIFEST ═══════");
    for domain in RandomDomain::iter_all() {
        println!("  -> {}", domain.as_str());
    }

    // --- Demon RNG Stream ---
    println!();
    println!("═══════ DEMON RNG STREAM ═══════");
    let mut demons = reality.random_stream(RandomDomain::Demons);
    println!("Demon Seed Fragment 01: 0x{:016X}", demons.next_u64());

    println!();
    println!("══════════════════════════════════════════════");
    println!(" Engine Foundation Verified.");
    println!(" Abyss World Generator Online.");
    println!(" Demon Seed Generator Operational.");
    println!(" Fractal Combat System Active.");
    println!(" Awaiting Fractal Boss Integration...");
    println!("══════════════════════════════════════════════");
}