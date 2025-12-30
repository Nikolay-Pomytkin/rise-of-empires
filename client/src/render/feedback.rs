//! Visual feedback systems (2D)
//!
//! Health bars, gathering indicators, floating resource numbers

use bevy::prelude::*;

use super::{layers, TILE_SIZE};

/// Plugin for visual feedback
pub struct VisualFeedbackPlugin;

impl Plugin for VisualFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_health_bars,
                update_health_bars,
                update_gathering_indicators,
                spawn_floating_text,
                update_floating_text,
            ),
        );
    }
}

// =============================================================================
// Health Bars
// =============================================================================

/// Marker for health bar background
#[derive(Component)]
pub struct HealthBarBackground {
    pub parent: Entity,
}

/// Marker for health bar fill
#[derive(Component)]
pub struct HealthBarFill {
    pub parent: Entity,
}

/// Spawn health bars for entities with Health component
fn spawn_health_bars(
    mut commands: Commands,
    entities_without_bars: Query<
        (Entity, &sim::Health, &sim::SimPosition),
        (With<sim::SimEntity>, Without<HealthBarBackground>),
    >,
    existing_bars: Query<&HealthBarBackground>,
) {
    // Check which entities already have bars
    let entities_with_bars: Vec<Entity> = existing_bars.iter().map(|b| b.parent).collect();

    for (entity, health, pos) in entities_without_bars.iter() {
        if entities_with_bars.contains(&entity) {
            continue;
        }

        let bar_width = TILE_SIZE * 0.8;
        let bar_height = TILE_SIZE * 0.1;
        let bar_y_offset = TILE_SIZE * 1.0; // Above the entity

        let world_x = pos.x * TILE_SIZE;
        let world_y = pos.z * TILE_SIZE + bar_y_offset;

        // Background (dark) - high Z layer for UI elements
        commands.spawn((
            Sprite {
                color: Color::srgba(0.1, 0.1, 0.1, 0.8),
                custom_size: Some(Vec2::new(bar_width, bar_height)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, layers::SELECTION + 1.0),
            HealthBarBackground { parent: entity },
        ));

        // Fill (green/yellow/red based on health)
        let health_percent = health.current as f32 / health.max as f32;
        let fill_width = bar_width * health_percent;
        let fill_color = health_color(health_percent);

        commands.spawn((
            Sprite {
                color: fill_color,
                custom_size: Some(Vec2::new(fill_width.max(1.0), bar_height * 0.8)),
                ..default()
            },
            Transform::from_xyz(
                world_x - (bar_width - fill_width) / 2.0,
                world_y,
                layers::SELECTION + 2.0,
            ),
            HealthBarFill { parent: entity },
        ));
    }
}

/// Update health bar positions and fill
fn update_health_bars(
    mut commands: Commands,
    entities: Query<(&sim::SimPosition, &sim::Health)>,
    mut backgrounds: Query<(Entity, &HealthBarBackground, &mut Transform), Without<HealthBarFill>>,
    mut fills: Query<(Entity, &HealthBarFill, &mut Transform, &mut Sprite)>,
) {
    let bar_width = TILE_SIZE * 0.8;
    let bar_y_offset = TILE_SIZE * 1.0;

    // Update backgrounds
    for (bg_entity, bg, mut transform) in backgrounds.iter_mut() {
        if let Ok((pos, _)) = entities.get(bg.parent) {
            transform.translation.x = pos.x * TILE_SIZE;
            transform.translation.y = pos.z * TILE_SIZE + bar_y_offset;
        } else {
            // Parent entity gone, despawn bar
            commands.entity(bg_entity).despawn();
        }
    }

    // Update fills
    for (fill_entity, fill, mut transform, mut sprite) in fills.iter_mut() {
        if let Ok((pos, health)) = entities.get(fill.parent) {
            let health_percent = health.current as f32 / health.max as f32;
            let fill_width = bar_width * health_percent;

            transform.translation.x = pos.x * TILE_SIZE - (bar_width - fill_width) / 2.0;
            transform.translation.y = pos.z * TILE_SIZE + bar_y_offset;

            sprite.color = health_color(health_percent);
            sprite.custom_size = Some(Vec2::new(fill_width.max(1.0), TILE_SIZE * 0.08));
        } else {
            // Parent entity gone, despawn bar
            commands.entity(fill_entity).despawn();
        }
    }
}

fn health_color(percent: f32) -> Color {
    if percent > 0.6 {
        Color::srgba(0.2, 0.8, 0.2, 0.9) // Green
    } else if percent > 0.3 {
        Color::srgba(0.9, 0.8, 0.1, 0.9) // Yellow
    } else {
        Color::srgba(0.9, 0.2, 0.1, 0.9) // Red
    }
}

// =============================================================================
// Gathering Indicators
// =============================================================================

/// Marker for gathering indicator
#[derive(Component)]
pub struct GatheringIndicator {
    pub parent: Entity,
    pub pulse_timer: f32,
}

/// Update gathering indicators (pulsing effect on gathering units)
fn update_gathering_indicators(
    mut commands: Commands,
    time: Res<Time>,
    gatherers: Query<(Entity, &sim::Gatherer, &sim::SimPosition)>,
    mut indicators: Query<(Entity, &mut GatheringIndicator, &mut Transform, &mut Sprite)>,
) {
    // Track which gatherers are actively gathering
    let gathering_entities: Vec<Entity> = gatherers
        .iter()
        .filter(|(_, g, _)| g.state == sim::GathererState::Gathering)
        .map(|(e, _, _)| e)
        .collect();

    // Remove indicators for non-gathering entities
    for (ind_entity, indicator, _, _) in indicators.iter() {
        if !gathering_entities.contains(&indicator.parent) {
            commands.entity(ind_entity).despawn();
        }
    }

    // Get existing indicator parents
    let existing_parents: Vec<Entity> = indicators.iter().map(|(_, i, _, _)| i.parent).collect();

    // Spawn new indicators
    for (entity, gatherer, pos) in gatherers.iter() {
        if gatherer.state != sim::GathererState::Gathering {
            continue;
        }
        if existing_parents.contains(&entity) {
            continue;
        }

        // Spawn a pulsing circle indicator
        let world_x = pos.x * TILE_SIZE;
        let world_y = pos.z * TILE_SIZE;

        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 0.8, 0.0, 0.6),
                custom_size: Some(Vec2::splat(TILE_SIZE * 0.5)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y - TILE_SIZE * 0.3, layers::UNITS_MAX + 1.0),
            GatheringIndicator {
                parent: entity,
                pulse_timer: 0.0,
            },
        ));
    }

    // Update existing indicators (pulse effect)
    for (_, mut indicator, mut transform, mut sprite) in indicators.iter_mut() {
        indicator.pulse_timer += time.delta_secs() * 3.0;
        let pulse = (indicator.pulse_timer.sin() + 1.0) / 2.0; // 0 to 1
        let scale = 0.8 + pulse * 0.4; // 0.8 to 1.2
        transform.scale = Vec3::splat(scale);

        // Pulse alpha too
        sprite.color = Color::srgba(1.0, 0.8, 0.0, 0.4 + pulse * 0.4);

        // Update position to follow parent
        if let Ok((_, _, pos)) = gatherers.get(indicator.parent) {
            transform.translation.x = pos.x * TILE_SIZE;
            transform.translation.y = pos.z * TILE_SIZE - TILE_SIZE * 0.3;
        }
    }
}

// =============================================================================
// Floating Text (Resource Numbers)
// =============================================================================

/// Floating text that rises and fades
#[derive(Component)]
pub struct FloatingText {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec3,
}

/// Event to spawn floating text
#[derive(bevy::ecs::event::Event)]
pub struct SpawnFloatingTextEvent {
    pub position: Vec3,
    pub text: String,
    pub color: Color,
}

fn spawn_floating_text(// For now, this is a stub - would need text rendering setup
    // In a full implementation, you'd use bevy's text2d
) {
    // TODO: Implement floating text spawning with Text2d
}

fn update_floating_text(
    mut commands: Commands,
    time: Res<Time>,
    mut texts: Query<(Entity, &mut FloatingText, &mut Transform)>,
) {
    for (entity, mut text, mut transform) in texts.iter_mut() {
        text.lifetime += time.delta_secs();

        if text.lifetime >= text.max_lifetime {
            commands.entity(entity).despawn();
            continue;
        }

        // Move upward and fade
        transform.translation += text.velocity * time.delta_secs();

        // Scale down as it fades
        let alpha = 1.0 - (text.lifetime / text.max_lifetime);
        transform.scale = Vec3::splat(alpha);
    }
}
