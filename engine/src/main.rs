// ⛧-Doom-Slayer-⛧

mod core;

use core::reality::RealityKernel;
use core::seed::Seed;

fn main() {
    let mut reality = RealityKernel::new(Seed::default());

    reality.set_depth(0.0);
    reality.set_entropy(0.0);
    reality.set_corruption(0.0);

    println!("FRACTAL DOOM :: ABYSS ENGINE");
    println!("Universe: {}", reality.seed());
    println!("Depth: {}", reality.depth());
    println!("Entropy: {:.2}", reality.entropy());
    println!("Corruption: {:.2}", reality.corruption());
    println!("Stability: {:.2}", reality.stability());
}
