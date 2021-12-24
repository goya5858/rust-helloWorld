fn main() {
    // static method
    let s = String::from("hello world");
    
    // instance method は　Instance.method() でいつものように使える
    println!("{} is {} characters long.", s, s.len());
}