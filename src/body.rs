use std::collections::HashMap;
use chrono::naive::NaiveDateTime;
use serde::Deserialize;
use crate::serde_utils;

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

/// A reference to another [Body] within the [System][crate::System] which is of gravitational
/// influence
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
