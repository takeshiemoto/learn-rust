use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort()
    }
}

pub fn run() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);

    sort_works(&mut table);
}

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

pub fn run_2() {
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;

    // 暗黙で参照解決される
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // 明示的に参照解決
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}

struct Point {
    x: i32,
    y: i32,
}

pub fn run_3() {
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    // .演算子は何段でも参照解決を行う。
    assert_eq!(rrr.y, 729);
}

// 'staticなライフタイムを持つ
static mut STASH: &i32 = &10;

// 関数fに渡される参照aはfの間しかライフタイムがない
// fn f(a: &i32) {
//     unsafe {
//         STASH = a;
//     }
// }

// staticライフタイムを
fn f(a: &'static i32) {
    unsafe {
        STASH = a;
    }
}
