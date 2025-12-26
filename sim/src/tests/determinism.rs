//! Determinism tests for the simulation

use crate::*;
use shared::*;

/// Test that the same seed produces the same RNG sequence
#[test]
fn test_rng_determinism() {
    let mut rng1 = SimRng::new(42);
    let mut rng2 = SimRng::new(42);

    let seq1: Vec<u64> = (0..1000).map(|_| rng1.next_u64()).collect();
    let seq2: Vec<u64> = (0..1000).map(|_| rng2.next_u64()).collect();

    assert_eq!(
        seq1, seq2,
        "RNG sequences should be identical with same seed"
    );
}

/// Test that different seeds produce different sequences
#[test]
fn test_rng_different_seeds() {
    let mut rng1 = SimRng::new(42);
    let mut rng2 = SimRng::new(43);

    let seq1: Vec<u64> = (0..100).map(|_| rng1.next_u64()).collect();
    let seq2: Vec<u64> = (0..100).map(|_| rng2.next_u64()).collect();

    assert_ne!(
        seq1, seq2,
        "Different seeds should produce different sequences"
    );
}

/// Test RNG reset functionality
#[test]
fn test_rng_reset() {
    let mut rng = SimRng::new(42);

    let first_run: Vec<u64> = (0..100).map(|_| rng.next_u64()).collect();
    rng.reset();
    let second_run: Vec<u64> = (0..100).map(|_| rng.next_u64()).collect();

    assert_eq!(
        first_run, second_run,
        "Reset RNG should produce same sequence"
    );
}

/// Test command ordering stability
#[test]
fn test_command_ordering() {
    let mut commands = vec![
        StampedCommand::new(
            10,
            PlayerId::PLAYER_2,
            0,
            GameCommand::Stop { entities: vec![] },
        ),
        StampedCommand::new(
            5,
            PlayerId::PLAYER_1,
            0,
            GameCommand::Stop { entities: vec![] },
        ),
        StampedCommand::new(
            10,
            PlayerId::PLAYER_1,
            0,
            GameCommand::Stop { entities: vec![] },
        ),
        StampedCommand::new(
            5,
            PlayerId::PLAYER_2,
            1,
            GameCommand::Stop { entities: vec![] },
        ),
        StampedCommand::new(
            5,
            PlayerId::PLAYER_2,
            0,
            GameCommand::Stop { entities: vec![] },
        ),
    ];

    commands.sort();

    // Expected order: tick 5 first, then tick 10
    // Within tick 5: player 1 first, then player 2
    // Within player 2: sequence 0 first, then sequence 1
    assert_eq!(commands[0].tick, 5);
    assert_eq!(commands[0].player_id, PlayerId::PLAYER_1);

    assert_eq!(commands[1].tick, 5);
    assert_eq!(commands[1].player_id, PlayerId::PLAYER_2);
    assert_eq!(commands[1].sequence, 0);

    assert_eq!(commands[2].tick, 5);
    assert_eq!(commands[2].player_id, PlayerId::PLAYER_2);
    assert_eq!(commands[2].sequence, 1);

    assert_eq!(commands[3].tick, 10);
    assert_eq!(commands[3].player_id, PlayerId::PLAYER_1);

    assert_eq!(commands[4].tick, 10);
    assert_eq!(commands[4].player_id, PlayerId::PLAYER_2);
}

/// Test command buffer drain ordering
#[test]
fn test_command_buffer_ordering() {
    let mut buffer = CommandBuffer::default();

    // Add commands out of order
    buffer.push_command(
        10,
        PlayerId::PLAYER_2,
        GameCommand::Stop { entities: vec![] },
    );
    buffer.push_command(
        5,
        PlayerId::PLAYER_1,
        GameCommand::Stop { entities: vec![] },
    );
    buffer.push_command(
        10,
        PlayerId::PLAYER_1,
        GameCommand::Stop { entities: vec![] },
    );
    buffer.push_command(
        5,
        PlayerId::PLAYER_2,
        GameCommand::Stop { entities: vec![] },
    );

    // Drain tick 5
    let tick5_commands = buffer.drain_for_tick(5);
    assert_eq!(tick5_commands.len(), 2);
    assert_eq!(tick5_commands[0].player_id, PlayerId::PLAYER_1);
    assert_eq!(tick5_commands[1].player_id, PlayerId::PLAYER_2);

    // Drain tick 10
    let tick10_commands = buffer.drain_for_tick(10);
    assert_eq!(tick10_commands.len(), 2);
    assert_eq!(tick10_commands[0].player_id, PlayerId::PLAYER_1);
    assert_eq!(tick10_commands[1].player_id, PlayerId::PLAYER_2);
}

/// Test snapshot hash consistency
#[test]
fn test_snapshot_hash_consistency() {
    let snapshot1 = WorldSnapshot {
        tick: 100,
        entities: vec![EntitySnapshot {
            id: EntityId::new(1),
            entity_type: EntityType::Unit(UnitType::Villager),
            position: [1.0, 0.0, 2.0],
            owner: Some(PlayerId::PLAYER_1),
            health: Some((25, 25)),
            selected_by: vec![],
            gatherer_state: None,
            production_queue: None,
            resource_remaining: None,
        }],
        players: vec![PlayerSnapshot {
            id: PlayerId::PLAYER_1,
            resources: ResourceBundle::new(200, 200, 0, 0),
            population: 1,
            population_cap: 5,
            current_age: AgeId::new("dark_age"),
            researched_techs: vec![],
        }],
    };

    let snapshot2 = snapshot1.clone();

    let hash1 = snapshot1.compute_hash();
    let hash2 = snapshot2.compute_hash();

    assert_eq!(
        hash1, hash2,
        "Identical snapshots should have identical hashes"
    );
}

/// Test that different snapshots produce different hashes
#[test]
fn test_snapshot_hash_different() {
    let snapshot1 = WorldSnapshot {
        tick: 100,
        entities: vec![],
        players: vec![PlayerSnapshot {
            id: PlayerId::PLAYER_1,
            resources: ResourceBundle::new(200, 200, 0, 0),
            population: 1,
            population_cap: 5,
            current_age: AgeId::new("dark_age"),
            researched_techs: vec![],
        }],
    };

    let mut snapshot2 = snapshot1.clone();
    snapshot2.players[0].resources.food = 201;

    let hash1 = snapshot1.compute_hash();
    let hash2 = snapshot2.compute_hash();

    assert_ne!(
        hash1, hash2,
        "Different snapshots should have different hashes"
    );
}

/// Test resource bundle operations
#[test]
fn test_resource_bundle() {
    let mut resources = ResourceBundle::new(100, 100, 50, 25);

    // Test can_afford
    assert!(resources.can_afford(&ResourceBundle::new(50, 50, 25, 0)));
    assert!(!resources.can_afford(&ResourceBundle::new(150, 0, 0, 0)));

    // Test subtract
    assert!(resources.subtract(&ResourceBundle::new(50, 50, 0, 0)));
    assert_eq!(resources.food, 50);
    assert_eq!(resources.wood, 50);

    // Test add
    resources.add(&ResourceBundle::new(10, 20, 30, 40));
    assert_eq!(resources.food, 60);
    assert_eq!(resources.wood, 70);
    assert_eq!(resources.gold, 80);
    assert_eq!(resources.stone, 65);
}

/// Test tick scheduler
#[test]
fn test_tick_scheduler() {
    let mut scheduler = TickScheduler::new(20);

    assert_eq!(scheduler.tick(), 0);

    scheduler.advance();
    assert_eq!(scheduler.tick(), 1);

    for _ in 0..99 {
        scheduler.advance();
    }
    assert_eq!(scheduler.tick(), 100);

    // Test time conversion
    assert_eq!(scheduler.tick_duration, 0.05); // 1/20
    assert_eq!(scheduler.ticks_to_seconds(20), 1.0);
    assert_eq!(scheduler.seconds_to_ticks(1.0), 20);
}
