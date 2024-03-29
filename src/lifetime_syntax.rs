pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// このコードはコンパイルできない
// -> &strはxとyどちらの参照？
// fn longest_1(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

pub fn run() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The lingest string is {}", result);

    // ダングリングポインタ
    // {
    //     let r;
    //
    //     {
    //         let x = 5;
    //         // xがスコープから抜ける
    //         // rは外側のスコープにいるので有効
    //         r = &x;
    //     }
    //
    //     println!("r: {}", r);
    // }
}
