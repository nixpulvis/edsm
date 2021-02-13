use edsm::json;

fn main() {
    // XXX: Very slow.
    let systems = json("edsm/dumps/systemsWithoutCoordinates.json");
    dbg!(&systems[0]);
}
