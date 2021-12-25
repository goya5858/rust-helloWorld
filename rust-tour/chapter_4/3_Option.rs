//enum Option<T> {
//    None,
//    Some(T),
//}

struct BagOfHolding<T> {
    item: Option<T>, // これはNULL型を許容する
}

fn main() {
    // i32型が入るべき場所にNULLを入れることができる
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("Bag is None");
    } else {
        println!("something in Bag");
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("something in bag");
        println!("{:#?}", i32_bag.item)
    } else {
        println!("nothing in Bag");
    }    

    // matchによってOptionはいい感じに分解される
    match i32_bag.item {
        Some(v) => println!("バッグに{}が見つかった", v),
        None    => println!("nothing in bag"),
    }
}