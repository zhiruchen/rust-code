fn main() {
    let s1 = String::from("hello");

    // pass s1 reference to calculate_length
    let length = calculate_length(&s1);
    println!("s1: {}, length: {}", s1, length);

    // use mut reference to change ss
    let mut ss = String::from("hello");
    println!("ss before change: {}", ss);
    change(&mut ss);
    println!("ss after change: {}", ss);

    // can have only one mut reference to particular data in particular scope
    // prevent data races at compile time
    // cannot borrow `ss` as mutable more than once at a time
    // let mut_ss1 = &mut ss;
    // let mut_ss2 = &mut ss;
    // println!("{}, {}", mut_ss1, mut_ss2);


    // use curly brackets to create new scope 
    {
        let mut_ss1 = &mut ss;
        println!("mut_ss1: {}", mut_ss1); 
    }

    let mut_ss2 = &mut ss;
    println!("mut_ss2: {}", mut_ss2);

    combine_imu_mut();
}

// s is a reference to a string
// we call having references as function parameters borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn combine_imu_mut() {
    let mut s = String::from("immu and mut");
    let immu_s1 = &s;
    let immu_s2 = &s;
    println!("immu ref: {}, {}", immu_s1, immu_s2);
    // immu_s1 and immu_s2 are no longer used after this point

    // These scopes don’t overlap, so this code is allowed
    let mut_ref = &mut s;
    println!("mut ref: {}", mut_ref);

    // let ref_to_dangling = dangling_ref();
    // println!("{}", ref_to_dangling); 
}

// missing lifetime specifier
// fn dangling_ref() -> &String {
//     let s = String::from("string");
//     &s
// }