fn main() {
    let number_list = vec![34, 50, 35, 60, 100];
    let result = largest(&number_list);
    println!("the largest number is: {}", result);
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
