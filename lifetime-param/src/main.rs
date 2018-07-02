fn main() {
    println!("Hello, world!");
    let x = String::from("longest string");
    let y = "shorter str";
    println!("the longest str is: {}", longest(x.as_str(), y));

    let a = String::from("the long ong ong string");
    {
        let b = String::from("the short str");
        let result = longest(a.as_str(), b.as_str());
        println!("the longest str is: {}", result);
    }
}

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

