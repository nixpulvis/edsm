use reqwest::{Result, Url};
use crate::System;

/// Base URL for the [System (also called Body) API](https://www.edsm.net/api-system-v1)
pub const SYSTEM_URL: &'static str = "https://www.edsm.net/api-system-v1";
/// Base URL for the [Systems API](https://www.edsm.net/api-v1)
pub const SYSTEMS_URL: &'static str = "https://www.edsm.net/api-v1";

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

/// Request a single [System] populated with many [Bodies][crate::Body]
pub fn bodies(system_name: &str) -> Result<System> {
    let url = Url::parse_with_params(
        &format!("{}/bodies", SYSTEM_URL),
        &[("systemName", system_name)]).unwrap();
    reqwest::blocking::get(url)?.json::<System>()
}

/// Fetch a system with it's factions
///
/// Passing a value of `true` for `history` will populate the appropriate history structures within
/// each faction.
pub fn factions(system_name: &str, history: bool) -> Result<System> {
    let url = Url::parse_with_params(
        &format!("{}/factions", SYSTEM_URL),
        &[("systemName", system_name),
          ("showHistory", &(history as u8).to_string())]).unwrap();
    reqwest::blocking::get(url)?.json::<System>()
}
