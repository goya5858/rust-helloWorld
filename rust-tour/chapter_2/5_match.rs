fn main() {
    let x = 42;

    match x {
        0 => { println!("found zero"); }
        1 | 2 => { println!("found 1 or 2"); }
        // 範囲にマッチ
        3..=9 => { println!("found 3 to 9"); }
        // マッチした数字を変数に束縛
        match_num @ 10..=100 => {
            println!("found {}", match_num);
        }
        // どのパターンでもなかった場合
        _ => { println!("found other case"); }
    }
}