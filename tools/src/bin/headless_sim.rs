//! Headless simulation runner
//!
//! Runs the simulation without graphics for testing and validation.

use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let ticks = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let seed = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(12345u64);

    let commands_file = args.get(3).map(PathBuf::from);

    println!("Rise RTS Headless Simulation");
    println!("============================");
    println!("Ticks: {}", ticks);
    println!("Seed: {}", seed);
    println!();

    // Load commands if provided
    let commands: Vec<shared::StampedCommand> = if let Some(path) = commands_file {
        println!("Loading commands from: {}", path.display());
        let content = fs::read_to_string(&path).expect("Failed to read commands file");
        ron::from_str(&content).expect("Failed to parse commands")
    } else {
        println!("No commands file provided, running empty simulation");
        Vec::new()
    };

    println!("Loaded {} commands", commands.len());
    println!();

    // Run simulation
    // Note: In a real implementation, we would run the sim without Bevy's windowing
    // For now, we just demonstrate the structure

    println!("Running simulation for {} ticks...", ticks);

    // Create a minimal simulation state
    let mut rng = sim::SimRng::new(seed);
    let mut tick = 0u64;

    // Simulate ticks
    for _ in 0..ticks {
        tick += 1;

        // In a full implementation, we would:
        // 1. Process commands for this tick
        // 2. Run all simulation systems
        // 3. Generate snapshot

        // For demonstration, just advance
        if tick % 100 == 0 {
            println!("  Tick {}", tick);
        }
    }

    println!();
    println!("Simulation complete!");
    println!("Final tick: {}", tick);
    println!("RNG state check: {}", rng.next_u64());

    // In a full implementation, we would output the final snapshot hash
    println!();
    println!("To validate determinism, run twice with same seed and commands.");
    println!("The final RNG state and snapshot hash should match.");
}

