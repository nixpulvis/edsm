use std::fs::File;

mod serde_utils;

/// Faction information and state tracking
///
/// A populated system contains many factions, including it's primary faction which has the largest
/// influence. All faction's influence together add up to 1.0, meaning that a single faction
/// gaining influence means that others must lose some.
///
/// Here a faction is exclusively referring to the game's minor factions, not to be confused with
/// the powers which are unions of minor factions operating at a larger political level within the
/// game's power play.
pub mod faction;
pub use self::faction::Faction;
use self::faction::*;

/// A star system in space, containing the bodies, factions, stations, etc
pub mod system;
pub use self::system::{Coordinate, System};

/// The bodies of a system, including it's stars, planets, moons, rings, etc
pub mod body;
pub use self::body::{Belt, Body, Orbit};

/// The EDSM web API
#[cfg(unix)]
pub mod api;
// #[cfg(not(unix))]
// pub mod api_web;

// TODO
// Require a market ID, someone some stations have.
// /market
// /shipyard
// /outfitting

pub fn json(file_path: &str) -> Vec<System> {
    let file = File::open(file_path).unwrap();
    serde_json::from_reader(file).unwrap()
}
