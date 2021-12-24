fn example() -> i32 {
    let x = 42;
    // 3高演算子
    let v = if x < 42 {-1} else {1};
    println!("v is {}", v);

    let food = "hunbuger";
    let result = match food {
        "hotdog" => "hotdog", //{ } は省略可能
        _ => "not hotdog",
    };
    println!("food is {}", result);

    let v = {
        let a = 1;
        let b = 2;
        a+b
    };
    println!("sum is {}", v);

    v+4
}

fn main() {
    println!("関数より{}", example());
}

