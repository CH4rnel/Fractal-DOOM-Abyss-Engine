//! ⛧-Doom-Slayer-⛧
//! Fractal DOOM: Abyss Engine - Execution Harness

mod core;
mod fractal;
mod world;

use core::abyss::MutationRecord;
use core::random::RandomDomain;
use core::reality::RealityKernel;
use core::seed::Seed;
use fractal::{Scene, Vec3, sdf_sphere};
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

    // --- Phase II: State Machine ---
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
        println!("[ RAY ] HIT Mandelbulb surface at distance {:.4}", t);
    } else {
        println!("[ RAY ] MISSED. Escaped to infinity.");
    }

    let sphere_d = sdf_sphere(Vec3::new(5.0, 0.0, 0.0), Vec3::zero(), 1.0);
    println!("[ SDF ] Sphere distance: {:.4}", sphere_d);

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

    // Demonstrate chunk construction, lifecycle, and evaluation.
    let coord = ChunkCoord::new(0, 0, 0);
    let mut chunk = Chunk::new(coord, reality.seed());
    chunk.activate();
    println!(
        "[ WORLD ] Chunk ({},{},{}) state: {:?}",
        chunk.coord.x, chunk.coord.y, chunk.coord.z, chunk.state
    );

    // Verify lifecycle state through the ChunkState enum.
    let is_active = matches!(chunk.state, ChunkState::Active | ChunkState::Corrupted);
    println!("[ WORLD ] Chunk present: {}", is_active);

    let evaluation: ChunkEvaluation = generator.evaluate_chunk(&chunk);
    println!(
        "[ WORLD ] Chunk eval: biome={}, density={:.2}, solid={}",
        evaluation.biome.as_str(),
        evaluation.center_density,
        evaluation.is_mostly_solid
    );

    // Evaluate world at a few free points.
    let test_points = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(5.0, 5.0, 5.0),
        Vec3::new(10.0, 0.0, 0.0),
    ];

    for (i, pos) in test_points.iter().enumerate() {
        let sdf = generator.evaluate_sdf(*pos);
        let density = generator.density(*pos);
        let solid = if generator.is_solid(*pos) {
            "SOLID"
        } else {
            "VOID"
        };
        println!(
            "[ WORLD ] Point {}: ({:.1}, {:.1}, {:.1}) -> SDF: {:.4}, Density: {:.2}, {}",
            i, pos.x, pos.y, pos.z, sdf, density, solid
        );
    }

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

    println!();
    println!("═══════ DOMAIN MANIFEST ═══════");
    for domain in RandomDomain::iter_all() {
        println!("  -> {}", domain.as_str());
    }

    println!();
    println!("═══════ DEMON RNG STREAM ═══════");
    let mut demons = reality.random_stream(RandomDomain::Demons);
    println!("Demon Seed Fragment 01: 0x{:016X}", demons.next_u64());

    println!();
    println!("══════════════════════════════════════════════");
    println!(" Engine Foundation Verified.");
    println!(" Abyss World Generator Online.");
    println!(" Awaiting Renderer Integration...");
    println!("══════════════════════════════════════════════");
}
