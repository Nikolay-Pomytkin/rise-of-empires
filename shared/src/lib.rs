//! Shared types for Rise RTS
//!
//! This crate contains types shared between the simulation and client:
//! - Entity and player identifiers
//! - Game commands with tick stamps
//! - Resource types and bundles
//! - World snapshots for rendering

pub mod commands;
pub mod ids;
pub mod resources;
pub mod snapshot;

pub use commands::*;
pub use ids::*;
pub use resources::*;
pub use snapshot::*;
