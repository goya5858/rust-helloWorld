struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

struct Ocean {
    animals: Vec< Box<dyn NoiseMaker> >,// Box< NoiseMakerを継承したStruct >てこと
    //animals: Vec< &'a dyn NoiseMaker >,
}

fn main() {
    let ferris = SeaCreature{
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    let sarah = SeaCreature{
        name: String::from("Sarah"),
        noise: String::from("swish"),
    };
    let ocean = Ocean {
        animals: vec![ Box::new(ferris), Box::new(sarah) ],
        //animals: vec![ &ferris, &sarah ],
    };
    for ani in ocean.animals.iter() {
        ani.make_noise();
    };

}