fn swap(x: i32, y: i32) -> (i32, i32) {
    return (x, y);
}

fn main() {
    let result = swap(123, 321); // tupleで返す 
    println!("{} {}", result.0, result.1);

    let (a, b) = swap(123, 321); // 展開
    println!("{} {}", a, b);
}