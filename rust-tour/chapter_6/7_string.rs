fn main() {
    let mut hellloworld = String::from("hello");
    hellloworld.push_str(" world");
    hellloworld = hellloworld + " !";
    println!("{}", hellloworld);
}