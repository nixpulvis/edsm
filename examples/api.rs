use edsm::api::*;

fn main() {
    dbg!(&system("Sol").unwrap());
    dbg!(&bodies("Sol").unwrap().bodies.as_ref().unwrap()[4].details);
    dbg!(&factions("LHS 6282", false).unwrap());
}
