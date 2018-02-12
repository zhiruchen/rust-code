// RUST_BACKTRACE=1 cargo run

fn main() {
    visitor_not_exist_element();
}

fn visitor_not_exist_element() {
    let v = vec![1,2,3];
    v[99];
}