// ⛧-Doom-Slayer-⛧

mod core;

use core::random::RandomDomain;
use core::reality::RealityKernel;
use core::seed::Seed;

fn main() {
    println!("══════════════════════════════════════════════");
    println!("        FRACTAL DOOM :: ABYSS ENGINE");
    println!("══════════════════════════════════════════════");
    println!();

    let reality = RealityKernel::new(Seed::default());

    println!("Universe   : {}", reality.seed());
    println!("Depth      : {}", reality.depth());
    println!("Entropy    : {:.2}", reality.entropy());
    println!("Corruption : {:.2}", reality.corruption());
    println!("Stability  : {:.2}", reality.stability());

    println!();
    println!("═══════ DEMON RNG STREAM ═══════");

    let mut demons = reality.random_stream(RandomDomain::Demons);

    for index in 1..=3 {
        println!(
            "Demon Seed Fragment {:02}: 0x{:016X}",
            index,
            demons.next_u64()
        );
    }

    println!();
    println!("══════════════════════════════════════════════");
    println!(" Reality Stable.");
    println!(" Awaiting Descent...");
    println!("══════════════════════════════════════════════");
}
