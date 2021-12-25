fn main() {
    let a: &'static str = "こんにちわ";
    println!("{} {}", a, a.len());
}