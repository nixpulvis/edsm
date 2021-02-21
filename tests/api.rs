use edsm::Coordinate;
use edsm::api::*;

#[test]
fn test_systems() {
    let system = systems("Sol").expect("EDSM API");
    assert!(system.len() == 1);

    let systems = systems("Soli").expect("EDSM API");
    assert!(systems.len() > 0);
    assert!(systems.len() > 1);
}

mod sphere {
    use super::*;

    #[test]
    fn test_systems_sphere_default() {
        let systems = systems_sphere("Sol", None, None).expect("EDSM API");
        assert_eq!(1238, systems.len());
    }

    #[test]
    fn test_systems_sphere_max() {
        let systems = systems_sphere("EV Cancri", Some(100.), None).expect("EDSM API");
        assert_eq!(140, systems.len());
    }

    #[test]
    fn test_systems_sphere_float_high() {
        let systems = systems_sphere("Sol", Some(17.8), None).expect("EDSM API");
        assert_eq!(82, systems.len());
    }

    #[test]
    fn test_systems_sphere_float_low() {
        let systems = systems_sphere("Sol", Some(17.3), None).expect("EDSM API");
        assert_eq!(78, systems.len());
    }

    #[test]
    fn test_systems_sphere_min_radius() {
        let systems = systems_sphere("Sol", Some(17.3), Some(10.)).expect("EDSM API");
        assert_eq!(66, systems.len());
    }

    // TODO: WTF is EDSM doing here?
    #[test]
    #[ignore]
    fn test_systems_sphere_min_greater_than_max() {
        let systems = systems_sphere("Alpha Centauri", Some(5.), Some(10.)).expect("EDSM API");
        dbg!(&systems);
        assert_eq!(0, systems.len());
    }
}

mod cube {
    use super::*;

    #[test]
    fn test_systems_cube_default() {
        let systems = systems_cube("EV Cancri", None).expect("EDSM API");
        assert_eq!(107, systems.len());
    }

    #[test]
    fn test_systems_cube_max() {
        let systems = systems_cube("EV Cancri", Some(200.)).expect("EDSM API");
        assert_eq!(174, systems.len());
    }

    // NOTE: The cube API seems to be rounding the size, unlike the radius for the sphere API.

    #[test]
    fn test_systems_cube_float_high() {
        let systems = systems_cube("Sol", Some(24.)).expect("EDSM API");
        assert_eq!(44, systems.len());
    }

    #[test]
    fn test_systems_cube_float_low() {
        let systems = systems_cube("Sol", Some(23.9)).expect("EDSM API");
        assert_eq!(40, systems.len());
    }
}

#[test]
fn test_system() {
    let system = system("Sol").expect("EDSM API");
    assert_eq!("Sol", system.name);
    assert_eq!(Some(27), system.id);
    assert_eq!(Some(Coordinate { x: 0.0, y: 0.0, z: 0.0 }), system.coords);
    assert_eq!(Some(true), system.require_permit);
}

#[test]
fn test_bodies() {
    let system = bodies("Sol").expect("EDSM API");
    assert_eq!("Sol", system.name);
    assert_eq!(Some(27), system.id);
    assert!(system.coords.is_none());
    assert!(system.require_permit.is_none());
    let bodies = system.bodies.expect("EDSM API (bodies)");
    assert_eq!(system.body_count.unwrap(), bodies.len() as u64);
    // TODO more
}

#[test]
fn test_factions() {
    let system = factions("Meliae", false).expect("EDSM API");
    assert_eq!("Meliae", system.name);
    assert_eq!(Some(1062), system.id);
    let controlling_faction = system.controlling_faction.unwrap();
    assert_eq!(81861, controlling_faction.id);
    assert_eq!("New Pilots Initiative", controlling_faction.name);
    assert_eq!("Independent", controlling_faction.allegiance);
    assert_eq!("Corporate", controlling_faction.government);
    let total_inf: f64 = system.factions.unwrap().iter().map(|f| f.influence).sum();
    assert_eq!(1., total_inf);
}

#[test]
fn test_traffic() {
    let system = traffic("Sol").expect("EDSM API");
    assert_eq!("Sol", system.name);
    let traffic = system.traffic.expect("EDSM API (traffic)");
    assert!(traffic.total < 500000);
    assert!(traffic.week < 1000);
    assert!(traffic.day < 100);
    // TODO breakdown
}

#[test]
fn test_deaths() {
    let system = deaths("Sol").expect("EDSM API");
    assert_eq!("Sol", system.name);
    let deaths = system.deaths.expect("EDSM API (deaths)");
    assert!(deaths.total < 5000);
    assert!(deaths.week < 100);
    assert!(deaths.day < 10);
    // TODO breakdown
}
