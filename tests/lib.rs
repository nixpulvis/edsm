use edsm::json;

#[test]
fn test_json() {
    let systems = json("tests/systemsWithoutCoordinates.json");
    assert!(!systems.is_empty());
    let systems = json("tests/systemsWithCoordinates.json");
    assert!(!systems.is_empty());
    let systems = json("tests/systemsPopulated.json");
    assert!(!systems.is_empty());
    for system in systems {
        assert!(system.information.state.is_some());
    }
}
