struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature {
        animal_type: String::from("crab"),
        name: String::from("feris"),
        arms: 2, 
        legs: 4,
        weapon: String::from("claw")
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("none"),
    };

    println!(
        "{} is a {}. they have {} arms, {} legs and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon  
    );

    println!(
        "{} is a {}. they have {} arms, {} legs and a {} weapon",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs, sarah.weapon  
    );
}

