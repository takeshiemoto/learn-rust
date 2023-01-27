use std::fs::File;
use std::io;
use std::io::Read;

// Resultを返す関数を律儀にmatchしている関数の例
fn read_username_from_file_a() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?で呼び出し元に返す例
fn read_username_from_file_b() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // Errに入れば呼び出し元にエラーを返す
    let mut s = String::new();
    f.read_to_string(&mut s)?; // 同じく
    Ok(s)
}

// さらに短く書いた例
fn read_username_from_file_c() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
