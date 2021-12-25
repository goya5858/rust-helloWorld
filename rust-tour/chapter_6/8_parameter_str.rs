fn say_it_loud(msg: &str) {
    println!("{} !!!", msg.to_string().to_uppercase() );
}

fn main() {
    // &strをそのまま貸出てる
    say_it_loud("hello");

    // Stringの参照 : &String　も貸し出せる
    say_it_loud( &String::from("goodbye") );
}