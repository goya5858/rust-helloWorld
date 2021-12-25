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
        println!("{}", &self.noise);
    }
}

fn static_make_noise(creature: &SeaCreature) {
    creature.make_noise();
}

// noise_makerをimplされた構造体みんな引数にとれる
fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
    noise_maker.make_noise();
}

fn main() {
    let creature = SeaCreature{
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };

    static_make_noise(&creature);
    static_make_noise(&creature);
}