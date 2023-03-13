// 複数のデータをまとめて扱う
// メソッドを持たせることができる

struct Customer {
    id: u32,
    name: String,
    address: String,
    email: String,
}

impl Customer {
    // 型関連関数
    // インスタンス作成不要
    fn new(id: u32, name: String, address: String, email: String) -> Self {
        Self {
            id,
            name,
            address,
            email,
        }
    }

    // メソッド
    // 第一引数にselfの参照を取る
    // インスタンスを作成する必要がある
    #[allow(dead_code)]
    fn get_name(&self) -> String {
        self.name.clone()
    }

    #[allow(dead_code)]
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[allow(dead_code)]
pub fn use_new() {
    let customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@sample.com"),
    );

    println!("id = {}", customer.id);
    println!("name = {}", customer.name);
    println!("address = {}", customer.address);
    println!("email = {}", customer.email);

    let mut c2 = Customer::new(
        100,
        String::from("John"),
        String::from("Tokyo city"),
        String::from("john@sample.com"),
    );

    c2.set_name(String::from("Paul"));
    println!("{}", c2.get_name())
}

// 参照型フィールド
// ライフタイム注釈が必要
// フィールドが保持する参照は構造体インスタンスのライフタイムより短い意味
#[allow(dead_code)]
pub struct Member<'a> {
    id: u32,
    name: &'a str,
    address: &'a str,
    email: &'a str,
}

impl<'a> Member<'a> {
    fn new(id: u32, name: &'a str, address: &'a str, email: &'a str) -> Self {
        Self {
            id,
            name,
            address,
            email,
        }
    }

    #[allow(dead_code)]
    fn get_name(&self) -> &str {
        self.name
    }

    #[allow(dead_code)]
    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
}

// タプル型構造体

struct Coordinates(usize, usize);

pub fn generate_tuple() {
    let c = Coordinates(100, 200);
    println!("{}", c.0);
    println!("{}", c.1);
}

#[allow(dead_code)]
pub fn run() {
    let customer = Customer {
        id: 100,
        name: String::from("山田太郎"),
        address: String::from("東京都新宿区"),
        email: String::from("yamada@sample.com"),
    };

    println!("id = {}", customer.id);
    println!("name = {}", customer.name);
    println!("address = {}", customer.address);
    println!("email = {}", customer.email);

    use_new();
}
