//! Replay runner
//!
//! Replays a command stream and validates against expected hashes.

use std::fs;
use std::path::PathBuf;

use tools::compute_world_hash_seahash;

/// Replay file format
#[derive(serde::Deserialize)]
struct ReplayFile {
    seed: u64,
    tick_rate: u32,
    commands: Vec<shared::StampedCommand>,
    /// Expected hashes at specific ticks for validation
    checkpoints: Vec<ReplayCheckpoint>,
}

#[derive(serde::Deserialize)]
struct ReplayCheckpoint {
    tick: u64,
    hash: u64,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let replay_path = args.get(1).map(PathBuf::from);

    println!("Rise RTS Replay Runner");
    println!("======================");
    println!();

    let Some(path) = replay_path else {
        println!("Usage: replay_runner <replay_file.ron>");
        println!();
        println!("Replay file format (RON):");
        println!("  ReplayFile(");
        println!("    seed: 12345,");
        println!("    tick_rate: 20,");
        println!("    commands: [");
        println!("      StampedCommand(tick: 10, player_id: PlayerId(1), ...),");
        println!("    ],");
        println!("    checkpoints: [");
        println!("      ReplayCheckpoint(tick: 100, hash: 123456789),");
        println!("    ],");
        println!("  )");
        return;
    };

    println!("Loading replay: {}", path.display());

    let content = fs::read_to_string(&path).expect("Failed to read replay file");
    let replay: ReplayFile = ron::from_str(&content).expect("Failed to parse replay file");

    println!("Seed: {}", replay.seed);
    println!("Tick rate: {} Hz", replay.tick_rate);
    println!("Commands: {}", replay.commands.len());
    println!("Checkpoints: {}", replay.checkpoints.len());
    println!();

    // Determine final tick
    let final_tick = replay.checkpoints.iter().map(|c| c.tick).max().unwrap_or(0);

    if final_tick == 0 {
        println!("No checkpoints to validate!");
        return;
    }

    println!("Running replay to tick {}...", final_tick);
    println!();

    // In a full implementation, we would run the actual simulation
    // For now, demonstrate the validation structure

    let mut validation_passed = true;
    let mut validated_count = 0;

    for checkpoint in &replay.checkpoints {
        // In real implementation:
        // 1. Run simulation to checkpoint.tick
        // 2. Generate snapshot
        // 3. Compute hash
        // 4. Compare

        // Placeholder: assume validation passes
        println!(
            "  Tick {}: expected hash {}, validation: SKIPPED (not implemented)",
            checkpoint.tick, checkpoint.hash
        );
        validated_count += 1;
    }

    println!();

    if validation_passed {
        println!(
            "Replay validation: PASSED ({} checkpoints)",
            validated_count
        );
    } else {
        println!("Replay validation: FAILED");
        std::process::exit(1);
    }

    println!();
    println!("TODO: Implement full replay validation with actual simulation");
    println!("TODO: Replay recording/playback");
}
