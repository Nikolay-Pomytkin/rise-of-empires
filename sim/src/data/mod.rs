//! Data loading and definitions
//!
//! External data files (RON) for techs, units, empires, etc.

pub mod empire_data;
pub mod game_data;
pub mod tech_data;
pub mod unit_data;

pub use empire_data::*;
pub use game_data::*;
pub use tech_data::*;
pub use unit_data::*;
