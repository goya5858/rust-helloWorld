struct SeaCreacher {
    noise: String,
}

// impl構文で構造体にメソッドを定義する
impl SeaCreacher {
    fn get_sound(&self) -> &str {
        &self.noise
    }
}

fn main() {
    let creature = SeaCreacher {
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound() );
}