// RUST_BACKTRACE=1 cargo run
use std::io;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    // visitor_not_exist_element();
    // visit_file();
    // visit_file1();
    // unwrap_error();
    let r = read_usrename_from_file();
    println!("{:?}", r);
}

fn visitor_not_exist_element() {
    let v = vec![1,2,3];
    v[99];
}

fn visit_file() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("open file error: {:?}", error)
        },
    };
}

fn visit_file1() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("create file error: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("open file hello.txt error: {:?}", error)
        },
    };
}

fn unwrap_error() {
    let f = File::open("hello.txt").expect("Failed to open file!");
}

// ? 只能用于返回Result的函数
fn read_usrename_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?; // 错误传播
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}