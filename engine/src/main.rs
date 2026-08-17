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
use fractal::{Scene, Vec3, sdf_sphere};
use gameplay::{
    DamageEvent, DamageResult, DamageType, EntityId, EntityState, FractalHitZone,
    NavigationPath, Player, Weapon, WeaponType, find_path, has_line_of_sight,
    move_towards, raycast, GeometryDeformation,
};
use world::{
    Biome, CHUNK_SIZE, Chunk, ChunkCoord, ChunkEvaluation, ChunkState, StreamUpdate,
    WorldGenerator, WorldStreamer,
};

fn main() {
    println!("══════════════════════════════════════════════");
    println!("        FRACTAL DOOM :: ABYSS ENGINE");
    println!("══════════════════════════════════════════════");

    let mut reality = RealityKernel::new(Seed::default());

    println!("[ INIT ] Seed: {}", reality.seed());
    println!("[ INIT ] Tick: {}", reality.tick());

    reality.descend();
    reality.corrupt(0.4);
    reality.fracture();

    println!("[ KERNEL ] State: {}", reality.state().as_str());
    println!("[ KERNEL ] Depth: {}", reality.depth());
    println!("[ KERNEL ] Entropy: {:.2}", reality.entropy());
    println!("[ KERNEL ] Corruption: {:.2}", reality.corruption());
    println!("[ KERNEL ] Stability: {:.2}", reality.stability());

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
        println!("[ RAY ] HIT Mandelbulb at distance {:.4}", t);
    } else {
        println!("[ RAY ] MISSED.");
    }

    let sphere_d = sdf_sphere(Vec3::new(5.0, 0.0, 0.0), Vec3::zero(), 1.0);
    println!("[ SDF ] Sphere distance: {:.4}", sphere_d);

    println!();
    println!("═══════ ABYSS WORLD GENERATOR ═══════");
    println!("[ WORLD ] Chunk Size: {:.1}", CHUNK_SIZE);

    let generator = WorldGenerator::new(&reality);
    let biome: Biome = generator.biome();
    println!("[ WORLD ] Biome: {}", biome.as_str());
    println!("[ WORLD ] Iterations: {}", biome.fractal_iterations());
    println!("[ WORLD ] Power: {:.1}", biome.fractal_power());
    println!("[ WORLD ] Corruption: {:.2}", biome.corruption_intensity());

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
    chunk.state = ChunkState::Loading;
    println!("[ CHUNK ] State: {:?}", chunk.state);
    chunk.activate();
    println!("[ CHUNK ] Activated: {:?}", chunk.state);
    chunk.corrupt();
    println!("[ CHUNK ] Corrupted: {:?}", chunk.state);

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
    println!("[ EVAL ] Density: {:.2}", evaluation.center_density);

    // Streaming
    println!();
    println!("═══════ CHUNK STREAMING ═══════");
    let mut streamer = WorldStreamer::new(reality.seed(), 1);
    let update: StreamUpdate = streamer.update(Vec3::new(0.0, 0.0, 0.0));
    println!(
        "[ STREAM ] Loaded: {}, Unloaded: {}",
        update.loaded, update.unloaded
    );
    println!("[ STREAM ] Active chunks: {}", streamer.loaded_count());

    // Demon Seed
    println!();
    println!("═══════ DEMON SEED GENERATOR ═══════");
    let demon_seed = DemonSeed::new(reality.seed(), ChunkCoord::new(3, 5, 2), reality.tick());
    println!("[ DEMON ] Derived ID: {}", demon_seed.derive_identity());

    let identity = DemonIdentity::reconstruct(demon_seed);
    println!(
        "[ DEMON ] Anatomy: scale={:.2}, limbs={}",
        identity.anatomy.body_scale, identity.anatomy.limb_count
    );
    println!(
        "[ DEMON ] Behavior: {}, aggression={:.2}",
        identity.behavior.movement_str(),
        identity.behavior.aggression
    );
    println!(
        "[ DEMON ] Mutation: level={}, type={}",
        identity.mutation.mutation_level,
        identity.mutation.mutation_type_str()
    );
    println!(
        "[ DEMON ] Genealogy: depth={}, lineage={}",
        identity.genealogy.recursion_depth,
        identity.genealogy.lineage_str()
    );
    println!("[ DEMON ] Threat: {}", identity.overall_threat().as_str());

    let deterministic = identity == DemonIdentity::reconstruct(demon_seed);
    println!("[ DEMON ] Determinism: {}", deterministic);

    // Phase VI: Fractal Combat
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
        "[ PLAYER ] Health: {:.0}/{:.0}",
        player.entity.health, player.entity.max_health
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

    // Weapons
    println!();
    println!("[ WEAPON ] Arsenal:");
    for w in [
        Weapon::shotgun(),
        Weapon::chainsaw(),
        Weapon::fractal_rifle(),
    ] {
        println!(
            "  {} | {} | DMG:{:.0} | DPS:{:.1}",
            w.name,
            w.weapon_type_str(),
            w.damage,
            w.dps()
        );
    }

    // Combat
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
    let dmg_result = gameplay::damage::apply_damage(&mut demon, &dmg_event);
    println!("  Applied {:.1} fractal damage", dmg_result.damage_applied);
    println!(
        "  Geometry deformation: {:.3}",
        dmg_result.geometry_deformation
    );
    println!(
        "  Demon HP: {:.0}/{:.0}, alive: {}",
        demon.health, demon.max_health, demon.is_alive
    );

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

    // Hit zones
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
    let ray = raycast(
        Vec3::new(-5.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        20.0,
        &generator,
    );
    match ray {
        Some(r) => println!("  HIT at {:.2}", r.distance),
        None => println!("  NO HIT"),
    }

    // Navigation
    println!();
    println!("[ NAVIGATION ] Pathfinding:");
    let path = find_path(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        &generator,
        50,
    );
    println!(
        "  Waypoints: {}, Length: {:.2}",
        path.waypoints.len(),
        path.total_length
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

    // Mutation Log
    println!();
    println!("═══════ MUTATION LOG ═══════");
    let history = reality.log().history();
    println!(
        "Total events: {} (Empty: {})",
        reality.log().len(),
        reality.log().is_empty()
    );
    for r in history {
        let r: &MutationRecord = r;
        println!(
            "  TICK {:04} | [{}] {} -> {} | ΔE:{:.2} ΔC:{:.2}",
            r.tick,
            r.event.as_str(),
            r.previous_state.as_str(),
            r.new_state.as_str(),
            r.entropy_delta,
            r.corruption_delta
        );
    }

    // Domain Manifest
    println!();
    println!("═══════ DOMAIN MANIFEST ═══════");
    for domain in RandomDomain::iter_all() {
        println!("  -> {}", domain.as_str());
    }

    // RNG
    println!();
    println!("═══════ DEMON RNG STREAM ═══════");
    let mut demons = reality.random_stream(RandomDomain::Demons);
    println!("Demon Seed Fragment 01: 0x{:016X}", demons.next_u64());

    println!();
    println!("══════════════════════════════════════════════");
    println!(" Phase VI: Fractal Combat System Active.");
    println!(" Awaiting Fractal Bosses...");
    println!("══════════════════════════════════════════════");
}
