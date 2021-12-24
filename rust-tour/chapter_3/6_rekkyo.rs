#![allow(dead_code)] //コンパイル時のエラーを止める

enum Species {
    Crab,
    Octpus,
    Fish,
    Clam
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} is crab", ferris.name),
        Species::Octpus => println!("{} is octpus", ferris.name),
        Species::Fish => println!("{} is fish", ferris.name),
        Species::Clam => println!("{} is clam", ferris.name),

    }
}