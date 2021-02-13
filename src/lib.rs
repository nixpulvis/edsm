use std::collections::HashMap;
use std::fs::File;
use reqwest::{Result, Url};
use chrono::naive::NaiveDateTime;
use serde::Deserialize;

mod serde_utils;

/// Base URL for the [System (also called Body) API](https://www.edsm.net/api-system-v1)
pub const SYSTEM_URL: &'static str = "https://www.edsm.net/api-system-v1";
/// Base URL for the [Systems API](https://www.edsm.net/api-v1)
pub const SYSTEMS_URL: &'static str = "https://www.edsm.net/api-v1";

#[derive(Deserialize, Debug, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// EDSM's representation of a solar system
#[derive(Deserialize, Debug)]
pub struct System {
    pub name: String,
    pub id: Option<u64>,
    pub id64: Option<u64>,

    // TODO: Support Optional date.
    // #[serde(deserialize_with = "serde_utils::space_seperated_datetime")]
    // pub date: Option<NaiveDateTime>,

    // from showCoords API parameter.
    pub coords: Option<Coordinate>,
    #[serde(rename = "coordsLocked")]
    pub coords_locked: Option<bool>,

    // from showInformation API parameter.
    #[serde(flatten, deserialize_with = "deserialize_information")]
    pub information: Information,

    // from showPermit API parameter.
    #[serde(rename = "requirePermit")]
    pub require_permit: Option<bool>,
    #[serde(rename = "permitName")]
    pub permit_name: Option<String>,

    // NOTE: Skipping this for now, use bodies to access if needed. Use the
    // showPrimaryStar API parameter when needed.
    // pub primary_star: Option<PrimaryStar>,

    // from system bodies API.
    #[serde(rename = "bodyCount")]
    pub body_count: Option<u64>,
    pub bodies: Option<Vec<Body>>,

    // TODO
    // pub stations: Option<Vec<Station>>,
    // #[serde(rename = "controllingFaction")]

    // TODO
    // pub factions: Option<Vec<Faction>>,
    // pub controlling_faction: Option<Faction>,

    pub deaths: Option<Statistic>,

    pub traffic: Option<Statistic>,
    #[serde(rename = "breakdown")]
    pub traffic_breakdown: Option<HashMap<String, u64>>,
}

fn deserialize_information<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    struct Both<T> {
        #[serde(flatten)]
        flat: Option<T>,
        #[serde(rename = "information")]
        not_flat: Option<T>,
    }

    let both: Both<T> = serde::Deserialize::deserialize(deserializer)?;
    match (both.flat, both.not_flat) {
        (Some(t), None) | (None, Some(t)) => Ok(t),
        (None, None) => Err(serde::de::Error::missing_field("information")),
        // TODO: Actually merge the fields on each... though I'm not sure we'll ever see a case
        // where this is needed.
        (Some(_), Some(t)) => Ok(t),
    }
}

/// General [System] information
#[derive(Deserialize, Debug)]
pub struct Information {
    pub allegiance: Option<String>,
    pub government: Option<String>,

    // TODO: Revisit if these can be a Faction object easily.
    pub faction: Option<String>,
    #[serde(alias = "factionState")]
    pub state: Option<String>,

    pub population: Option<u64>,
    pub security: Option<String>,
    pub economy: Option<String>,
    #[serde(rename = "secondEconomy")]
    pub second_economy: Option<String>,
    pub reserve: Option<String>,
}

/// Statistics for how much ship traffic a [System] gets.
#[derive(Deserialize, Debug)]
pub struct Statistic {
    pub total: u64,
    pub week: u64,
    pub day: u64,
}

/// EDSM's representation of a celestial body
#[derive(Deserialize, Debug)]
pub struct Body {
    pub name: String,
    pub id: u64,
    pub id64: Option<u64>,
    #[serde(rename = "bodyId")]
    pub body_id: Option<u64>,

    // NOTE: The tag of details' enum is the body type.
    #[serde(flatten)]
    pub details: Details,
    #[serde(rename = "subType")]
    pub subtype: String,

    pub parents: Option<Vec<Parent>>,
    #[serde(rename = "distanceToArrival")]
    pub distance_to_arrival: u64,
    #[serde(rename = "surfaceTemperature")]
    pub surface_temperature: f64,
    #[serde(flatten)]
    pub orbit: Orbit,
    pub belts: Option<Vec<Belt>>,
    #[serde(deserialize_with = "serde_utils::space_seperated_datetime")]
    #[serde(rename = "updateTime")]
    updated_at: NaiveDateTime,
}

/// Orbital information about a [Body]
#[derive(Deserialize, Debug)]
pub struct Orbit {
    #[serde(rename = "orbitalPeriod")]
    pub orbital_period:  Option<f64>,
    #[serde(rename = "semiMajorAxis")]
    pub semi_major_axis: Option<f64>,
    #[serde(rename = "orbitalEccentricity")]
    pub orbital_eccentricity: Option<f64>,
    #[serde(rename = "orbitalInclination")]
    pub orbital_inclination: Option<f64>,
    #[serde(rename = "argOfPeriapsis")]
    pub arg_of_periapsis: Option<f64>,
    #[serde(rename = "rotationalPeriod")]
    pub rotational_period: Option<f64>,
    #[serde(rename = "rotationalPeriodTidallyLocked")]
    pub rotational_period_tidally_locked: bool,
    #[serde(rename = "axialTilt")]
    pub axial_tilt: Option<f64>,
}

/// Detailed information for each type of [Body]
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Details {
    Star {
        age: u64,
        #[serde(rename = "isMainStar")]
        is_main_star: bool,
        #[serde(rename = "isScoopable")]
        is_scoopable: bool,
        #[serde(rename = "solarMasses")]
        solar_masses: Option<f64>,
        #[serde(rename = "solarRadius")]
        solar_radius: Option<f64>,
        #[serde(rename = "spectralClass")]
        spectral_class: Option<String>,
        luminosity: Option<String>,
        #[serde(rename = "absoluteMagnitude")]
        absolute_magnitude: Option<f64>,
    },
    Planet {
        #[serde(rename = "earthMasses")]
        earth_masses: f64,
        radius: f64,
        #[serde(rename = "isLandable")]
        is_landable: bool,
        gravity: Option<f64>,
        #[serde(rename = "surfacePressure")]
        surface_pressure: Option<f64>,
        #[serde(rename = "volcanismType")]
        volcanism_type: Option<String>,
        #[serde(rename = "atmosphereType")]
        atmosphere_type: Option<String>,
        #[serde(rename = "atmosphereComposition")]
        atmosphere_composition: Option<HashMap<String, f64>>,
        #[serde(rename = "solidComposition")]
        solid_composition: Option<HashMap<String, f64>>,
        #[serde(rename = "terraformingState")]
        terraforming_state: Option<String>,
    }
}

/// A reference to another [Body] within the [System] which is of gravitational influence
#[derive(Deserialize, Debug)]
pub enum Parent {
    Null(u64),
    Star(u64),
    Planet(u64),
}

/// A ring of small objects trapped in a ring around a [Body].
#[derive(Deserialize, Debug)]
pub struct Belt {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub mass: f64,
    #[serde(rename = "innerRadius")]
    pub inner_radius: f64,
    #[serde(rename = "outerRadius")]
    pub outer_radius: f64,
}

pub fn json(file_path: &str) -> Vec<System> {
    let file = File::open(file_path).unwrap();
    serde_json::from_reader(file).unwrap()
}

/// Request many [Systems][System] by name
///
/// This function will only return a single system on an exact match.
pub fn systems(query: &str) -> Result<Vec<System>> {
    let mut params = vec![("systemName", query)];

    // TODO
    // // NOTE: `onlyFeatured` cannot be called with `showInformation`
    // if true {
    //     params.push(("onlyFeatured", "1"));
    // }

    // TODO: only(Un)knownCoordinates
    // TODO: startDateTime and endDateTime

    if true {
        params.push(("showId", "1"));
    }
    if true {
        params.push(("showCoordinates", "1"));
    }
    if true {
        params.push(("showPermit", "1"));
    }
    if true {
        params.push(("showInformation", "1"));
    }

    let path = format!("{}/systems", SYSTEMS_URL);
    let url = Url::parse_with_params(&path, &params).unwrap();
    reqwest::blocking::get(url)?.json::<Vec<System>>()
}

// TODO: enum for allowing coords as well as systemName.
pub fn systems_sphere(name: &str, radius: Option<f64>, min_radius: Option<f64>)
    -> Result<Vec<System>>
{
    let mut params = vec![("systemName", name.to_string())];

    if let Some(r) = radius {
        // TODO: Better error handling.
        if r > 100. { panic!("radius too large") }
        params.push(("radius", r.to_string()));
    }
    if let Some(m) = min_radius {
        params.push(("minRadius", m.to_string()));
    }

    if true {
        params.push(("showId", "1".into()));
    }
    if true {
        params.push(("showCoordinates", "1".into()));
    }
    if true {
        params.push(("showPermit", "1".into()));
    }
    if true {
        params.push(("showInformation", "1".into()));
    }

    let path = format!("{}/sphere-systems", SYSTEMS_URL);
    let url = Url::parse_with_params(&path, &params).unwrap();
    reqwest::blocking::get(url)?.json::<Vec<System>>()
}

// TODO: enum for allowing coords as well as systemName.
// TODO: What exactly is `size`?
pub fn systems_cube(name: &str, size: Option<f64>) -> Result<Vec<System>> {
    let mut params = vec![("systemName", name.to_string())];

    if let Some(s) = size {
        params.push(("size", s.to_string()));
    }

    if true {
        params.push(("showId", "1".into()));
    }
    if true {
        params.push(("showCoordinates", "1".into()));
    }
    if true {
        params.push(("showPermit", "1".into()));
    }
    if true {
        params.push(("showInformation", "1".into()));
    }

    let path = format!("{}/cube-systems", SYSTEMS_URL);
    let url = Url::parse_with_params(&path, &params).unwrap();
    reqwest::blocking::get(url)?.json::<Vec<System>>()
}

// TODO: unify both sphere and cube functions.

/// Request a single [System]
pub fn system(name: &str) -> Result<System> {
    let mut params = vec![("systemName", name)];
    if true {
        // sets `id` and `id64`
        params.push(("showId", "1"));
    }
    if true {
        // sets `coords`
        params.push(("showCoordinates", "1"));
    }
    if true {
        // sets `require_permit` and `permit_name`
        params.push(("showPermit", "1"));
    }
    if true {
        // sets `allegiance`, `government`, `faction`, `faction_state`, `population`, `security`,
        // and `economy`
        params.push(("showInformation", "1"));
    }

    let path = format!("{}/system", SYSTEMS_URL);
    let url = Url::parse_with_params(&path, &params).unwrap();
    reqwest::blocking::get(url)?.json::<System>()
    // TODO: add option to call bodies and merge, then remove `bodies` function.
}

// TODO: fn /estimated-value

/// Request a single [System] populated with many [Bodies][Body]
pub fn bodies(system_name: &str) -> Result<System> {
    let url = Url::parse_with_params(
        &format!("{}/bodies", SYSTEM_URL),
        &[("systemName", system_name)]).unwrap();
    reqwest::blocking::get(url)?.json::<System>()
}

pub fn factions(system_name: &str) -> Result<System> {
    let url = Url::parse_with_params(
        &format!("{}/factions", SYSTEM_URL),
        &[("systemName", system_name)]).unwrap();
    reqwest::blocking::get(url)?.json::<System>()
}

/// Request a single [System]'s traffic report
pub fn traffic(system_name: &str) -> Result<System> {
    let url = Url::parse_with_params(
        &format!("{}/traffic", SYSTEM_URL),
        &[("systemName", system_name)]).unwrap();
    reqwest::blocking::get(url)?.json::<System>()
}

/// Request a single [System]'s death report
pub fn deaths(system_name: &str) -> Result<System> {
    let url = Url::parse_with_params(
        &format!("{}/deaths", SYSTEM_URL),
        &[("systemName", system_name)]).unwrap();
    reqwest::blocking::get(url)?.json::<System>()
}

// TODO
// scan values
// // Require a market ID, someone some stations have.
// /market
// /shipyard
// /outfitting
