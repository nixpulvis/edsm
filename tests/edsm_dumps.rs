use edsm::json;

#[test]
fn systems_without_coordinates() {
    let systems = json("tests/systemsWithoutCoordinates.json");
    assert!(!systems.is_empty());
}

#[test]
fn systems_with_coordinates() {
    let systems = json("tests/systemsWithCoordinates.json");
    assert!(!systems.is_empty());
}

#[test]
fn systems_populated() {
    let systems = json("tests/systemsPopulated.json");
    assert!(!systems.is_empty());
    for system in systems {
        assert!(system.information.state.is_some());
    }
}
