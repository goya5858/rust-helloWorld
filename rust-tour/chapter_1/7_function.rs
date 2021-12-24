fn add(x: i32, y: i32) -> i32 {
    return x + y
    // x + y // 最後に書かれた ; のない分もリターンになる
}

fn main() { 
    println!("{}", add(42, 13));
}