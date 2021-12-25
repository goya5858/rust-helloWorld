struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

///////////////
/// 抽象クラスやinterface的な何か
trait NoiseMaker {
    fn make_noise(&self);
}

// NoiseMaker を SeaCreature に実装する
impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("bulb"),
    };
    creature.make_noise();
}



