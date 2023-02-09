pub fn run() {
    let s = "learn rust language .";
    for i in 0..s.len() {
        println!("{}: {}", i, s.as_bytes()[i]);
    }

    // 無効なコード
    // 元のベクタがミュータブルでもイテレータを適用した場合はイミュータブル
    // let mut v = vec![10, 20, 30];
    // for item in v.into_iter() {
    //     item += 1;
    //     println!("", item);
    // }

    // 正解
    let mut v = vec![10, 20, 30];
    for mut item in v.into_iter() {
        item += 1;
        println!("{}", item)
    }

    // 反復処理中のシーケンスに変更を加える
    let s1 = &[3, 4, 5];
    let s2 = &[7, 8];

    let mut iterator = s1.iter();
    for item_ref in iterator {
        println!("{}", *item_ref);
    }

    println!("; ");

    iterator = s2.iter();
    for item_ref in iterator {
        println!("{}", *item_ref);
    }

    print_nth_char("John", 2);
    print_codes("John");
}

pub fn print_nth_char(s: &str, mut n: u32) {
    let mut iter: std::str::Chars = s.chars();
    loop {
        let item: Option<char> = iter.next();
        match item {
            Some(c) => {
                if n == 0 {
                    println!("{}", c);
                    break;
                }
            }
            None => {
                break;
            }
        }
        n -= 1;
    }
}

pub fn print_codes(s: &str) {
    let mut iter = s.chars();
    loop {
        match iter.next() {
            Some(c) => {
                println!("{}: {}", c, c as u32);
            }
            None => {
                break;
            }
        }
    }
}
