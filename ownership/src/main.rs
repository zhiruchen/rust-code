fn main() {
    let s = "hello"; // string literal, immutable, hardcoded directly into final executable

    let mut s1 = String::from("hello"); // allocate a mount of memory on heap
    s1.push_str(", world!");

    println!("{}", s1);

    let x = 5;
    let y = x;

    let s2 = String::from("hello2");
    // meaning we copy the pointer, the length, and the capacity that are on the stack. 
    // We do not copy the data on the heap that the pointer refers to

    // s2 moved to s3, Rust will never automatically create “deep” copies of your data
    let s3 = s2; 

    // println!("s2: {}", s2);  // value borrowed here after move

    // deeply copy heap data
    let ss = String::from("hello ss");
    let deepcopy_ss = ss.clone();
    println!("ss: {}, deepcopy_ss: {}", ss, deepcopy_ss);

    takes_ownership(ss);
    takes_ownership(deepcopy_ss);
    // println!("deepcopy_ss after move: {}", deepcopy_ss); // value borrowed here after move
    makes_copy(x);

    let s5 = gives_ownership(); // moves it's return value into s5
    let s6 = String::from("s6");
    let back_ss = takes_give_back(s6);
} // back_ss goes out of scope, and droped, s6 goes out of scope and moved, so nothing happens
// s5 goes out of scope and sroped. 

// ss was moved
fn takes_ownership(ss: String) {
    println!("{}", ss);
}

// copy integer
fn makes_copy(integer: i32) {
    println!("{}", integer);
}

// gives ownership
fn gives_ownership() -> String {
    let s = String::from("Hello");
    s
}

fn takes_give_back(s: String) -> String {
    s
}