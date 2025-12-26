//! Command ordering tests

use crate::*;
use shared::*;

/// Test that commands are processed in correct order
#[test]
fn test_command_ordering_stability() {
    // Create many commands with various orderings
    let mut commands = Vec::new();

    for tick in (0..10).rev() {
        for player in [PlayerId::PLAYER_2, PlayerId::PLAYER_1] {
            for seq in (0..3).rev() {
                commands.push(StampedCommand::new(
                    tick,
                    player,
                    seq,
                    GameCommand::Stop { entities: vec![] },
                ));
            }
        }
    }

    // Shuffle using a deterministic pattern
    let mut shuffled = commands.clone();
    for i in 0..shuffled.len() {
        let j = (i * 7 + 3) % shuffled.len();
        shuffled.swap(i, j);
    }

    // Sort
    shuffled.sort();

    // Verify order
    let mut prev_tick = 0u64;
    let mut prev_player = 0u8;
    let mut prev_seq = 0u64;

    for (i, cmd) in shuffled.iter().enumerate() {
        if i > 0 {
            // Tick should be >= previous
            assert!(
                cmd.tick >= prev_tick,
                "Tick should be non-decreasing: {} < {}",
                cmd.tick,
                prev_tick
            );

            if cmd.tick == prev_tick {
                // Player should be >= previous within same tick
                assert!(
                    cmd.player_id.0 >= prev_player,
                    "Player should be non-decreasing within tick: {} < {}",
                    cmd.player_id.0,
                    prev_player
                );

                if cmd.player_id.0 == prev_player {
                    // Sequence should be > previous within same tick/player
                    assert!(
                        cmd.sequence > prev_seq,
                        "Sequence should be increasing within tick/player: {} <= {}",
                        cmd.sequence,
                        prev_seq
                    );
                }
            }
        }

        prev_tick = cmd.tick;
        prev_player = cmd.player_id.0;
        prev_seq = cmd.sequence;
    }
}

/// Test command buffer handles multiple ticks correctly
#[test]
fn test_command_buffer_multi_tick() {
    let mut buffer = CommandBuffer::default();

    // Add commands for ticks 1, 2, 3
    buffer.push_command(
        1,
        PlayerId::PLAYER_1,
        GameCommand::Stop { entities: vec![] },
    );
    buffer.push_command(
        2,
        PlayerId::PLAYER_1,
        GameCommand::Stop { entities: vec![] },
    );
    buffer.push_command(
        3,
        PlayerId::PLAYER_1,
        GameCommand::Stop { entities: vec![] },
    );
    buffer.push_command(
        2,
        PlayerId::PLAYER_2,
        GameCommand::Stop { entities: vec![] },
    );

    // Drain tick 1
    let t1 = buffer.drain_for_tick(1);
    assert_eq!(t1.len(), 1);
    assert_eq!(t1[0].tick, 1);

    // Drain tick 2
    let t2 = buffer.drain_for_tick(2);
    assert_eq!(t2.len(), 2);
    assert!(t2.iter().all(|c| c.tick == 2));

    // Drain tick 3
    let t3 = buffer.drain_for_tick(3);
    assert_eq!(t3.len(), 1);
    assert_eq!(t3[0].tick, 3);

    // Buffer should be empty
    assert!(buffer.is_empty());
}

/// Test that late commands (tick already passed) are still processed
#[test]
fn test_late_commands() {
    let mut buffer = CommandBuffer::default();

    // Add command for tick 5
    buffer.push_command(
        5,
        PlayerId::PLAYER_1,
        GameCommand::Stop { entities: vec![] },
    );

    // Drain for tick 10 (command is late but should still be included)
    let commands = buffer.drain_for_tick(10);
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].tick, 5);
}
