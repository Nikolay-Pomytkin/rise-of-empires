//! World state hashing for determinism validation

use shared::WorldSnapshot;

/// Compute a stable hash of a world snapshot
///
/// This hash is used to validate that two simulation runs with
/// identical inputs produce identical outputs.
pub fn compute_world_hash(snapshot: &WorldSnapshot) -> u64 {
    snapshot.compute_hash()
}

/// Compute hash using seahash for better distribution
pub fn compute_world_hash_seahash(snapshot: &WorldSnapshot) -> u64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = seahash::SeaHasher::new();

    // Hash tick
    snapshot.tick.hash(&mut hasher);

    // Hash entities deterministically
    for entity in &snapshot.entities {
        entity.id.0.hash(&mut hasher);

        // Hash position as fixed-point
        let px = (entity.position[0] * 10000.0) as i64;
        let py = (entity.position[1] * 10000.0) as i64;
        let pz = (entity.position[2] * 10000.0) as i64;
        px.hash(&mut hasher);
        py.hash(&mut hasher);
        pz.hash(&mut hasher);

        if let Some((current, max)) = entity.health {
            current.hash(&mut hasher);
            max.hash(&mut hasher);
        }

        if let Some(remaining) = entity.resource_remaining {
            remaining.hash(&mut hasher);
        }

        if let Some(ref gs) = entity.gatherer_state {
            gs.carry_amount.hash(&mut hasher);
            gs.is_returning.hash(&mut hasher);
        }

        if let Some(ref queue) = entity.production_queue {
            queue.items.len().hash(&mut hasher);
            for item in &queue.items {
                item.ticks_remaining.hash(&mut hasher);
            }
        }
    }

    // Hash player states
    for player in &snapshot.players {
        player.id.0.hash(&mut hasher);
        player.resources.food.hash(&mut hasher);
        player.resources.wood.hash(&mut hasher);
        player.resources.gold.hash(&mut hasher);
        player.resources.stone.hash(&mut hasher);
        player.population.hash(&mut hasher);
        player.population_cap.hash(&mut hasher);
    }

    hasher.finish()
}

/// Compare two snapshots for equality (ignoring selection state)
pub fn snapshots_equal(a: &WorldSnapshot, b: &WorldSnapshot) -> bool {
    if a.tick != b.tick {
        return false;
    }

    if a.entities.len() != b.entities.len() {
        return false;
    }

    if a.players.len() != b.players.len() {
        return false;
    }

    // Compare entities
    for (ea, eb) in a.entities.iter().zip(b.entities.iter()) {
        if ea.id != eb.id {
            return false;
        }

        // Compare positions with tolerance
        const EPSILON: f32 = 0.0001;
        for i in 0..3 {
            if (ea.position[i] - eb.position[i]).abs() > EPSILON {
                return false;
            }
        }

        if ea.health != eb.health {
            return false;
        }

        if ea.resource_remaining != eb.resource_remaining {
            return false;
        }
    }

    // Compare players
    for (pa, pb) in a.players.iter().zip(b.players.iter()) {
        if pa.id != pb.id {
            return false;
        }
        if pa.resources != pb.resources {
            return false;
        }
        if pa.population != pb.population {
            return false;
        }
    }

    true
}
