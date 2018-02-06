
struct Foo<'a> {
    x: &'a i32,
}

impl <'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    println!("left time examples");

    let y = &5;
    let f = Foo { x: y };
    println!("{}", f.x);

    println!("x is: {}", f.x());
}
