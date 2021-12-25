static PI: f64 = 3.14;

fn main() {
    // スタティック変数は関数スコープでも定義可能
    static mut SECRET: &'static str = "swordfish";

    let msg: &'static str = "hello world";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // ルールを破ることはできますが、それを明示する必要があります
    unsafe {
        SECRET = "abracatabra";
        println!("{}", SECRET);
    }
}