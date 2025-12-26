//! Input handling
//!
//! Selection, commands, and input state management.

use bevy::ecs::message::{Message, MessageReader, MessageWriter};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputState::default())
            .insert_resource(SelectionState::default())
            .insert_resource(BuildingPlacementState::default())
            .add_message::<SelectionEvent>()
            .add_message::<CommandEvent>()
            .add_systems(Update, handle_mouse_input)
            .add_systems(Update, update_drag_box.after(handle_mouse_input))
            .add_systems(Update, process_selection.after(update_drag_box))
            .add_systems(Update, process_commands.after(process_selection));
    }
}

/// Current input state
#[derive(Resource, Default)]
pub struct InputState {
    /// Is left mouse button held?
    pub left_mouse_held: bool,
    /// Position where left mouse was pressed
    pub left_mouse_start: Option<Vec2>,
    /// Is a drag box active?
    pub dragging: bool,
    /// Current mouse world position
    pub mouse_world_pos: Option<Vec3>,
}

/// Current selection state
#[derive(Resource, Default)]
pub struct SelectionState {
    /// Currently selected entities
    pub selected: Vec<Entity>,
    /// Active player (for multiplayer, would be set differently)
    pub active_player: shared::PlayerId,
}

#[derive(Resource, Default)]
pub struct BuildingPlacementState {
    pub placing: Option<shared::BuildingType>,
    pub valid: bool,
}

impl SelectionState {
    pub fn clear(&mut self) {
        self.selected.clear();
    }

    pub fn select(&mut self, entities: Vec<Entity>) {
        self.selected = entities;
    }

    pub fn add(&mut self, entity: Entity) {
        if !self.selected.contains(&entity) {
            self.selected.push(entity);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
}

/// Selection event
#[derive(Message, Clone)]
pub enum SelectionEvent {
    /// Clear current selection
    Clear,
    /// Select entities (replace current)
    Select(Vec<Entity>),
    /// Add to current selection
    Add(Vec<Entity>),
    /// Toggle selection
    Toggle(Entity),
}

/// Command event (from input to bridge)
#[derive(Message, Clone)]
pub enum CommandEvent {
    Move {
        target: Vec3,
    },
    Gather {
        node: Entity,
    },
    Stop,
    Build {
        building_type: shared::BuildingType,
        tile_x: i32,
        tile_z: i32,
    },
}

/// Handle mouse input
fn handle_mouse_input(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
    mut input_state: ResMut<InputState>,
    mut placement_state: ResMut<BuildingPlacementState>,
    mut selection_events: MessageWriter<SelectionEvent>,
    mut command_events: MessageWriter<CommandEvent>,
    selectable: Query<(Entity, &sim::SimPosition, &sim::Owner), With<sim::SimEntity>>,
    resource_nodes: Query<Entity, With<sim::ResourceNode>>,
    selection_state: Res<SelectionState>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    let placement_mode = placement_state.placing.is_some();

    // Update mouse world position
    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
            // Intersect with Y=0 plane
            let t = -ray.origin.y / ray.direction.y;
            if t > 0.0 {
                input_state.mouse_world_pos = Some(ray.origin + ray.direction * t);
            }
        }
    }

    // Escape cancels placement
    if keyboard.just_pressed(KeyCode::Escape) && placement_mode {
        placement_state.placing = None;
        return;
    }

    // Left click while placing = confirm placement
    if mouse.just_pressed(MouseButton::Left) && placement_mode {
        if let Some(building_type) = placement_state.placing {
            if let Some(world_pos) = input_state.mouse_world_pos {
                // Convert to tile coordinates
                let tile_x = world_pos.x.round() as i32;
                let tile_z = world_pos.z.round() as i32;

                command_events.write(CommandEvent::Build {
                    building_type,
                    tile_x,
                    tile_z,
                });
                placement_state.placing = None;
            }
        }
        return; // Don't do normal selection
    }

    // Left mouse button - selection
    if mouse.just_pressed(MouseButton::Left) {
        input_state.left_mouse_held = true;
        if let Some(cursor_pos) = window.cursor_position() {
            input_state.left_mouse_start = Some(cursor_pos);
        }
        input_state.dragging = false;
    }

    if mouse.just_released(MouseButton::Left) {
        let shift_held =
            keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

        if input_state.dragging {
            // Drag box selection is handled in update_drag_box
        } else {
            // Click selection
            if let Some(world_pos) = input_state.mouse_world_pos {
                // Find entity under cursor
                let clicked =
                    find_entity_at_position(&selectable, world_pos, selection_state.active_player);

                if let Some(entity) = clicked {
                    if shift_held {
                        selection_events.write(SelectionEvent::Toggle(entity));
                    } else {
                        selection_events.write(SelectionEvent::Select(vec![entity]));
                    }
                } else if !shift_held {
                    selection_events.write(SelectionEvent::Clear);
                }
            }
        }

        input_state.left_mouse_held = false;
        input_state.left_mouse_start = None;
        input_state.dragging = false;
    }

    // Right mouse button - commands
    if mouse.just_pressed(MouseButton::Right) {
        if !selection_state.is_empty() {
            if let Some(world_pos) = input_state.mouse_world_pos {
                // Check if clicking on a resource node
                let clicked_node =
                    find_resource_node_at_position(&selectable, &resource_nodes, world_pos);

                if let Some(node_entity) = clicked_node {
                    command_events.write(CommandEvent::Gather { node: node_entity });
                } else {
                    command_events.write(CommandEvent::Move { target: world_pos });
                }
            }
        }
    }

    // Stop command (S key)
    if keyboard.just_pressed(KeyCode::KeyS) && !keyboard.pressed(KeyCode::ControlLeft) {
        if !selection_state.is_empty() {
            command_events.write(CommandEvent::Stop);
        }
    }
}

/// Update drag box state
fn update_drag_box(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut input_state: ResMut<InputState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection_events: MessageWriter<SelectionEvent>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>,
    selectable: Query<(Entity, &sim::SimPosition, &sim::Owner), With<sim::SimEntity>>,
    selection_state: Res<SelectionState>,
) {
    if !input_state.left_mouse_held {
        return;
    }

    let Ok(window) = windows.single() else { return };
    let Some(start) = input_state.left_mouse_start else {
        return;
    };
    let Some(current) = window.cursor_position() else {
        return;
    };

    // Check if dragging
    let drag_threshold = 5.0;
    let drag_distance = (current - start).length();

    if drag_distance > drag_threshold {
        input_state.dragging = true;
    }

    // If drag box released, select entities in box
    if input_state.dragging && !input_state.left_mouse_held {
        let Ok((camera, camera_transform)) = camera.single() else {
            return;
        };

        // Get box corners in world space
        let min_screen = start.min(current);
        let max_screen = start.max(current);

        let shift_held =
            keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

        // Find all entities in the box
        let mut selected_entities = Vec::new();

        for (entity, pos, owner) in selectable.iter() {
            // Only select own units
            if owner.player_id != selection_state.active_player {
                continue;
            }

            // Project entity position to screen
            let world_pos = Vec3::new(pos.x, pos.y, pos.z);
            if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, world_pos) {
                if screen_pos.x >= min_screen.x
                    && screen_pos.x <= max_screen.x
                    && screen_pos.y >= min_screen.y
                    && screen_pos.y <= max_screen.y
                {
                    selected_entities.push(entity);
                }
            }
        }

        if !selected_entities.is_empty() {
            if shift_held {
                selection_events.write(SelectionEvent::Add(selected_entities));
            } else {
                selection_events.write(SelectionEvent::Select(selected_entities));
            }
        }
    }
}

/// Process selection events
fn process_selection(
    mut selection_events: MessageReader<SelectionEvent>,
    mut selection_state: ResMut<SelectionState>,
    mut selectables: Query<&mut sim::Selected>,
) {
    for event in selection_events.read() {
        // Clear old selection markers
        for entity in &selection_state.selected {
            if let Ok(mut selected) = selectables.get_mut(*entity) {
                selected.deselect(selection_state.active_player);
            }
        }

        match event {
            SelectionEvent::Clear => {
                selection_state.clear();
            }
            SelectionEvent::Select(entities) => {
                selection_state.select(entities.clone());
            }
            SelectionEvent::Add(entities) => {
                for entity in entities {
                    selection_state.add(*entity);
                }
            }
            SelectionEvent::Toggle(entity) => {
                if selection_state.selected.contains(entity) {
                    selection_state.selected.retain(|e| e != entity);
                } else {
                    selection_state.add(*entity);
                }
            }
        }

        // Set new selection markers
        for entity in &selection_state.selected {
            if let Ok(mut selected) = selectables.get_mut(*entity) {
                selected.select(selection_state.active_player);
            }
        }
    }
}

/// Process command events and send to sim
fn process_commands(
    mut command_events: MessageReader<CommandEvent>,
    selection_state: Res<SelectionState>,
    tick: Res<sim::TickScheduler>,
    mut command_buffer: ResMut<sim::CommandBuffer>,
    sim_entities: Query<&sim::SimEntity>,
    units: Query<&sim::Unit>,
) {
    for event in command_events.read() {
        let entity_ids: Vec<shared::EntityId> = selection_state
            .selected
            .iter()
            .filter_map(|e| sim_entities.get(*e).ok().map(|se| se.id))
            .collect();

        if entity_ids.is_empty() {
            continue;
        }

        let command = match event {
            CommandEvent::Move { target } => shared::GameCommand::Move {
                entities: entity_ids,
                target_x: target.x,
                target_z: target.z,
            },
            CommandEvent::Gather { node } => {
                if let Ok(sim_entity) = sim_entities.get(*node) {
                    shared::GameCommand::Gather {
                        entities: entity_ids,
                        node: sim_entity.id,
                    }
                } else {
                    continue;
                }
            }
            CommandEvent::Stop => shared::GameCommand::Stop {
                entities: entity_ids,
            },
            CommandEvent::Build {
                building_type,
                tile_x,
                tile_z,
            } => {
                // Find first villager in selection to be the builder
                let builder_id = selection_state
                    .selected
                    .iter()
                    .filter_map(|e| {
                        let sim_entity = sim_entities.get(*e).ok()?;
                        let unit = units.get(*e).ok()?;
                        if unit.unit_type == shared::UnitType::Villager {
                            Some(sim_entity.id)
                        } else {
                            None
                        }
                    })
                    .next();

                if let Some(builder) = builder_id {
                    shared::GameCommand::Build {
                        builder,
                        building_type: *building_type,
                        tile_x: *tile_x,
                        tile_z: *tile_z,
                    }
                } else {
                    // No villager selected, can't build
                    continue;
                }
            }
        };

        command_buffer.push_command(
            tick.tick() + 1, // Execute next tick
            selection_state.active_player,
            command,
        );
    }
}

fn find_entity_at_position(
    selectable: &Query<(Entity, &sim::SimPosition, &sim::Owner), With<sim::SimEntity>>,
    world_pos: Vec3,
    active_player: shared::PlayerId,
) -> Option<Entity> {
    let click_radius = 1.0;

    let mut closest: Option<(Entity, f32)> = None;

    for (entity, pos, owner) in selectable.iter() {
        // Prefer own units
        let distance = ((pos.x - world_pos.x).powi(2) + (pos.z - world_pos.z).powi(2)).sqrt();

        if distance < click_radius {
            let priority_distance = if owner.player_id == active_player {
                distance
            } else {
                distance + 0.5 // Slight penalty for enemy units
            };

            if closest.is_none() || priority_distance < closest.unwrap().1 {
                closest = Some((entity, priority_distance));
            }
        }
    }

    closest.map(|(e, _)| e)
}

fn find_resource_node_at_position(
    selectable: &Query<(Entity, &sim::SimPosition, &sim::Owner), With<sim::SimEntity>>,
    resource_nodes: &Query<Entity, With<sim::ResourceNode>>,
    world_pos: Vec3,
) -> Option<Entity> {
    let click_radius = 1.5;

    for (entity, pos, _) in selectable.iter() {
        if !resource_nodes.contains(entity) {
            continue;
        }

        let distance = ((pos.x - world_pos.x).powi(2) + (pos.z - world_pos.z).powi(2)).sqrt();

        if distance < click_radius {
            return Some(entity);
        }
    }

    None
}
