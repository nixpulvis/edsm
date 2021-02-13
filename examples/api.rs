use edsm::*;

fn main() {
    dbg!(&system("Sol").unwrap());
    dbg!(&bodies("Sol").unwrap().bodies.as_ref().unwrap()[4].details);
}
