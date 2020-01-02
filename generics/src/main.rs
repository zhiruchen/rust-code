#[derive(Debug)]
struct Point<T> {  // Point<T> struct is generic over some type T
    x: T,
    y: T
}

#[derive(Debug)]
struct Point1<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 35, 60, 100];
    let result = largest(&number_list);
    println!("the largest number is: {}", result);

    // println!("the generic largest number: {}", generic_largest(&number_list));

    let integer_p = Point {x: 5, y: 6};
    let float_p = Point {x: 5.6, y: 6.6};
    println!("generic structs: {:?}, {:?}", integer_p, float_p);

    let pp = Point1 {x: 5, y: 6.6};
    println!("generic structs with multi types: {:?}", pp);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn generic_largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }